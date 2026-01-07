use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn start_tcp_listener(local_id: String, local_name: String) -> Option<TcpStream> {
    let listener = TcpListener::bind("0.0.0.0:34255").expect("Failed to bind TCP listener");
    println!("Listening for incoming connections ");
    if let Some(stream_result) = listener.incoming().next() {
        match stream_result {
            Ok(mut stream) => {
                let mut buf = [0u8; 256];
                if let Ok(len) = stream.read(&mut buf) {
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    let parts: Vec<&str> = msg.split("|").collect();
                    if parts.len() < 3 {
                        return None;
                    }
                    let sender_name = parts[2];
                    print!("Request from:{}. Accept (y/n)", sender_name);
                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice).unwrap();
                    if choice.trim().eq_ignore_ascii_case("y") {
                        let reply = format!("OK|{}|{}", local_id, local_name);
                        stream.write_all(reply.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return Some(stream);
                    } else {
                        let reply = format!("REJECTED|{}|{}", local_id, local_name);
                        stream.write_all(reply.as_bytes());
                    }
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    None
}
