use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn start_tcp_listener(local_id: String, local_name: String) -> Option<TcpStream> {
    let listener = TcpListener::bind("0.0.0.0:34255").expect("Failed to bind TCP listener");
    println!("Listening for incoming connections on 0.0.0.0:34255...");

    if let Some(stream_result) = listener.incoming().next() {
        match stream_result {
            Ok(mut stream) => {
                let mut buf = [0u8; 256];
                if let Ok(len) = stream.read(&mut buf) {
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    let parts: Vec<&str> = msg.split("|").collect();

                    // FIX 1: Check length carefully
                    if parts.len() < 3 {
                        return None;
                    }

                    // FIX 2: Use index 2 for name (HELLO|ID|NAME|SEND)
                    let sender_name = parts[2];

                    print!("Request from: {}. Accept (y/n)? ", sender_name);
                    // Flush stdout to ensure the prompt appears before input
                    let _ = io::stdout().flush();

                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice).unwrap();

                    if choice.trim().eq_ignore_ascii_case("y") {
                        let reply = format!("OK|{}|{}", local_id, local_name);
                        stream.write_all(reply.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        // FIX 3: CRITICAL - Return the stream!
                        return Some(stream);
                    } else {
                        let reply = format!("REJECTED|{}|{}", local_id, local_name);
                        let _ = stream.write_all(reply.as_bytes());
                    }
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    None
}
