use crate::discovery::identity;
use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{
    collections::HashMap,
    fmt,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tokio::{runtime::Runtime, task, time::sleep};

// =======================
// DATA STRUCTURES
// =======================

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub state: State,
    pub address: SocketAddr,
    pub last_seen: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Idle,
    Send,
    Receive,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state_str = match self {
            State::Idle => "IDLE",
            State::Send => "SEND",
            State::Receive => "RECEIVE",
        };
        write!(f, "{}", state_str)
    }
}

pub type PeerTable = HashMap<String, Peer>;
pub type SharedPeers = Arc<RwLock<PeerTable>>;
pub type SharedState = Arc<RwLock<State>>;

const SERVICE_TYPE: &str = "_orb._udp.local.";
const PORT: u16 = 34254;

// =======================
// PEER HANDLER
// =======================

fn handle_peer(info: ResolvedService, peers: &SharedPeers, my_id: &str) {
    let props = &info.txt_properties;

    let peer_id = match props.get("id") {
        Some(p) => p.val_str().to_string(),
        None => return,
    };

    if peer_id == my_id {
        return;
    }

    let name = props
        .get("name")
        .map(|p| p.val_str().to_string())
        .unwrap_or_else(|| "unknown".into());

    let state = match props.get("state").map(|p| p.val_str()) {
        Some("IDLE") => State::Idle,
        Some("SEND") => State::Send,
        Some("RECEIVE") => State::Receive,
        _ => State::Idle,
    };

    let addr = match info.addresses.iter().next() {
        Some(ip) => SocketAddr::new(ip.to_ip_addr(), info.port),
        None => return,
    };

    let mut table = peers.write().unwrap();

    table.insert(
        peer_id.clone(),
        Peer {
            id: peer_id,
            name,
            state,
            address: addr,
            last_seen: Instant::now(),
        },
    );

    // TTL cleanup
    table.retain(|_, p| p.last_seen.elapsed() < Duration::from_secs(6));
}

// =======================
// REGISTER SERVICE
// =======================

fn register_service(mdns: &ServiceDaemon, node_id: &str, name: &str, state: State) {
    let instance_name = format!("orb-{}", node_id);
    let host_name = "orb.local.";

    let state_str = state.to_string();

    let txt = vec![
        ("id", node_id),
        ("name", name),
        ("state", state_str.as_str()),
    ];

    let service =
        ServiceInfo::new(SERVICE_TYPE, &instance_name, host_name, "", PORT, &txt[..]).unwrap();

    mdns.register(service).unwrap();
}

// =======================
// DISCOVERY ENTRY POINT
// =======================

pub fn run_discovery(peers: SharedPeers, my_state: SharedState) {
    let node_id = identity::get_node_id();
    let my_id = node_id.clone();

    let name = format!(
        "{}-{}",
        whoami::fallible::hostname().unwrap(),
        whoami::fallible::username().unwrap()
    );

    let mdns = ServiceDaemon::new().unwrap();

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Initial registration
    register_service(&mdns, &node_id, &name, *my_state.read().unwrap());

    // =======================
    // STATE UPDATE LOOP
    // =======================
    let mdns_clone = mdns.clone();
    let node_id_clone = node_id.clone();
    let name_clone = name.clone();
    let state_clone = my_state.clone();
    rt.block_on(async move {
        task::spawn(async move {
            let mut last_state = State::Idle;

            loop {
                let current_state = *state_clone.read().unwrap();

                if current_state != last_state {
                    // re-register service with updated state
                    register_service(&mdns_clone, &node_id_clone, &name_clone, current_state);
                    last_state = current_state;
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
        // =======================
        // BROWSER LOOP
        // =======================
        let receiver = mdns.browse(SERVICE_TYPE).unwrap();
        let peers_clone = peers.clone();

        tokio::task::spawn_blocking(move || {
            while let Ok(event) = receiver.recv() {
                if let ServiceEvent::ServiceResolved(info) = event {
                    handle_peer(*info, &peers_clone, &my_id);
                }
            }
        })
        .await
        .unwrap();
    });
}
