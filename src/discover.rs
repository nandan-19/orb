use std::{net::UdpSocket, thread, time::Duration};

pub fn run_discovery() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    socket.set_broadcast(true)?;

    let node_id = "node-123";
    let name = "ORB";
    let state = "IDLE";

    let send_socket = socket.try_clone()?;
    thread::spawn(move || {
        loop {
            let msg = format!("ORB|{}|{}|{}", node_id, name, state);
            let _ = send_socket.send_to(msg.as_bytes(), "255.255.255.255:34254");
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
