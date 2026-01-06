use std::{
    collections::HashMap,
    io,
    sync::{Arc, RwLock},
    thread,
};

mod discovery;
use discovery::discover::Peer;
mod transfer;
use crate::{
    discovery::{
        discover::State, identity::get_node_id, tcp_handshake::tcp_handshake,
        tcp_listener::start_tcp_listener,
    },
    transfer::{receive_file_data, send_file_data},
};

fn set_state(state_lock: &Arc<RwLock<State>>, new_state: State) {
    let mut state = state_lock.write().unwrap();
    *state = new_state;
}

fn send_file(peers: Arc<RwLock<HashMap<String, Peer>>>) {
    println!("Scanning for receivers...");

    let peers_guard = peers.read().unwrap();

    if peers_guard.is_empty() {
        println!("No peers found");
        return;
    }
    let mut nearby_peers: HashMap<u32, Peer> = HashMap::new();
    let mut counter: u32 = 0;
    for (id, peer) in peers_guard.iter() {
        let state = &peer.state;

        if *state == State::Receive {
            let nearby_peer = peer.clone();
            counter += 1;
            println!(" {}: {} ({}) is [{}]", counter, peer.name, id, peer.state);
            nearby_peers.insert(counter, nearby_peer);
        }
    }

    if nearby_peers.is_empty() {
        println!("No receivers found");
        return;
    }

    println!("Enter your choice from 1 to {}", counter);
    let mut receiver = String::new();
    io::stdin()
        .read_line(&mut receiver)
        .expect("Please select the receiver");
    let num = receiver.trim().parse::<u32>().unwrap_or_default();
    match nearby_peers.get(&num) {
        Some(peer) => {
            match tcp_handshake(
                peer,
                &get_node_id(),
                &format!(
                    "{}-{}",
                    whoami::fallible::hostname().expect("Got error fetching hostname"),
                    whoami::fallible::username().expect("Got error fetching username")
                ),
            ) {
                Ok(stream) => {
                    println!("Enter path of file to send:");
                    let mut path = String::new();
                    io::stdin()
                        .read_line(&mut path)
                        .expect("error reading file path");
                    if let Err(e) = send_file_data(stream, path.trim()) {
                        println!("Error sending file: {}", e);
                    }
                }
                Err(e) => println!("Handshake failed: {}", e),
            }
        }

        None => {
            println!("Device not found");
        }
    }
}

fn receive_file() {
    println!("Waiting to receive file...");
    // This now BLOCKS until connection is done or failed
    if let Some(stream) = start_tcp_listener(
        get_node_id(),
        format!(
            "{}-{}",
            whoami::fallible::hostname().expect("Got error fetching hostname"),
            whoami::fallible::username().expect("Got error fetching username")
        ),
    ) {
        if let Err(e) = receive_file_data(stream) {
            println!("Error receiving file: {}", e);
        }
    }
}

fn main() {
    let peers: Arc<RwLock<HashMap<String, Peer>>> = Arc::new(RwLock::new(HashMap::new()));
    let my_state: Arc<RwLock<State>> = Arc::new(RwLock::new(State::Idle));

    let peers_for_thread = peers.clone();
    let state_for_thread = my_state.clone();

    thread::spawn(move || {
        let _ = discovery::discover::run_discovery(peers_for_thread, state_for_thread);
    });

    loop {
        println!("The current state is {}", my_state.read().unwrap());
        let mut choice = String::new();
        println!("Enter your choice: \n 1. Send \n 2. Receive \n 3. exit");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");

        let option = choice.trim();

        match option {
            "1" => {
                set_state(&my_state, State::Send);

                send_file(peers.clone());

                set_state(&my_state, State::Idle);
            }
            "2" => {
                set_state(&my_state, State::Receive);

                receive_file();

                set_state(&my_state, State::Idle);
            }
            "3" => break,
            _ => {
                println!("Enter valid input 1,2 or 3");
                break;
            }
        }
    }
}
