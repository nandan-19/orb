use crate::discovery::identity;
use get_if_addrs::{IfAddr, get_if_addrs};
use std::{
    collections::HashMap,
    fmt,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

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
            State::Idle => "IDLE".to_string(),
            State::Send => "SEND".to_string(),
            State::Receive => "RECEIVE".to_string(),
        };
        write!(f, "{}", state_str)
    }
}

pub type PeerTable = HashMap<String, Peer>;
pub type SharedPeers = Arc<RwLock<PeerTable>>;
pub type SharedState = Arc<RwLock<State>>;
fn get_ipv4_interfaces() -> Vec<(Ipv4Addr, Ipv4Addr)> {
    let mut result = Vec::new();
    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            match iface.addr {
                IfAddr::V4(v4) => {
                    if !v4.ip.is_loopback() {
                        result.push((v4.ip, v4.netmask));
                    }
                }
                _ => {}
            }
        }
    }
    result
}

fn compute_broadcast(ip: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr {
    let ip_u32 = u32::from(ip);
    let mask_u32 = u32::from(netmask);
    Ipv4Addr::from(ip_u32 | !mask_u32)
}

pub fn run_discovery(peers: SharedPeers, my_state: SharedState) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    socket.set_broadcast(true)?;

    let node_id = identity::get_node_id();

    let id = node_id.clone();
    let name = format!(
        "{}-{}",
        whoami::fallible::hostname().expect("Unkown hostname"),
        whoami::fallible::username().expect("Unkown username")
    );
    let state_for_send = my_state.clone();

    let send_socket = socket.try_clone()?;
    thread::spawn(move || {
        let interfaces = get_ipv4_interfaces();
        loop {
            let current_state_str = *state_for_send.read().unwrap();

            let msg = format!("ORB|{}|{}|{}", node_id, name, current_state_str);
            for (ip, mask) in interfaces.clone() {
                let broadcast = compute_broadcast(ip, mask);
                let addr = format!("{}:34254", broadcast);
                let _ = send_socket.send_to(msg.as_bytes(), addr);
            }
            thread::sleep(Duration::from_secs(2));
        }
    });

    let mut buf = [0u8; 1024];
    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..len]);

        let parts: Vec<&str> = msg.split("|").collect();
        if parts.len() > 1 && parts[1] == id {
            continue;
        }

        let peer_id = parts[1].to_string();
        let peer_name = parts[2].to_string();
        let peer_state = match parts[3] {
            "IDLE" => State::Idle,
            "RECEIVE" => State::Receive,
            "SEND" => State::Send,
            _ => continue,
        };
        let mut peers_lock = peers.write().unwrap();

        peers_lock.insert(
            peer_id.clone(),
            Peer {
                id: peer_id,
                name: peer_name,
                address: src,
                state: peer_state,
                last_seen: Instant::now(),
            },
        );

        let timeout = Duration::from_secs(6);

        peers_lock.retain(|_, peer| peer.last_seen.elapsed() < timeout);

        // println!("Active peers:");
        // for peer in peers_lock.values() {
        //     println!(
        //         "- {} ({}) [{}] @ {}",
        //         peer.name, peer.id, peer.state, peer.address
        //     );
        // }
    }
}
