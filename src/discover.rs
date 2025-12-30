use std::{
    net::{Ipv4Addr, UdpSocket},
    thread,
    time::Duration,
};

use get_if_addrs::{IfAddr, get_if_addrs};
use sysinfo::System;
use uuid::Uuid;

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

    let node_id = Uuid::new_v4();
    let systemOption = System::host_name();
    let name;
    match systemOption {
        Some(value) => name = value,
        None => name = String::from("System Name not found"),
    }
    let state = "IDLE";

    let send_socket = socket.try_clone()?;
    thread::spawn(move || {
        loop {
            let msg = format!("{}|{}|{}", node_id, name, state);
            let interfaces = get_ipv4_interfaces();
            for (ip, mask) in interfaces {
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
        println!("Discovered {}: from {}", src, msg);
    }
}
