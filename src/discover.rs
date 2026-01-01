use crate::identity;
use get_if_addrs::{IfAddr, get_if_addrs};
use std::{
    net::{Ipv4Addr, UdpSocket},
    thread,
    time::Duration,
};

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
pub fn run_discovery() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    socket.set_broadcast(true)?;
    let node_id = identity::get_node_id();
    let id = node_id.clone();
    let name = format!("{}-{}", whoami::hostname(), whoami::username());
    let state = "IDLE";
    let send_socket = socket.try_clone()?;
    thread::spawn(move || {
        let interfaces = get_ipv4_interfaces();
        loop {
            let msg = format!("ORB|{}|{}|{}", node_id, name, state);
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
        println!("Discovered {}: from {}", src, msg);
    }
}
