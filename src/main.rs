use std::io;
mod discover;
#[derive(Debug)]
enum State {
    Idle,
    Send,
    Receive,
}

fn sendFiles(Receiver: &str, files: &str) {
    //files are sent here
}
fn handshake(Receiver: &str) -> Option<u32> {
    //doing udp check for nearby devices;

    let response = 1; // here we will actually try to connect with the receiver
    if response > 1 { Some(response) } else { None }
}
fn send(state: &mut State) {
    println!("Select the files to send:");
    let mut files = String::new();
    io::stdin()
        .read_line(&mut files)
        .expect("Error reading file names");

    println!("Looking for nearby devices...");
    println!("Found 2 users:");
    println!("1. asdlfks");
    println!("2. asdfsadf");
    println!("Select option:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading choice");

    let trimmed = choice.trim();
    let response = handshake(&trimmed);
    match response {
        Some(value) => print!("Succesfully connected started tranmitting..."),
        None => println!("Timeout or the user rejected"),
    }

    let response = sendFiles(&trimmed, &files);

    *state = State::Idle;
}

fn receive(state: &mut State) {
    println!("Waiting to receive files...");
    println!("No invitations. Going back to IDLE.");

    *state = State::Idle;
}

fn main() {
    println!("Welcome to ORB!");
    if let Err(e) = discover::run_discovery() {
        eprintln!("Discovery error{}", e);
    }
    // let mut state = State::Idle;
    //
    // println!("You are in IDLE state.");
    // while true {
    //     println!("Enter 'send' or 'receive' or 'stop':");
    //
    //     let mut option = String::new();
    //     io::stdin()
    //         .read_line(&mut option)
    //         .expect("Failed to read input");
    //
    //     let option = option.trim(); // ✅ VERY IMPORTANT
    //
    //     match option {
    //         "send" => {
    //             state = State::Send;
    //             send(&mut state);
    //         }
    //         "receive" => {
    //             state = State::Receive;
    //             receive(&mut state);
    //         }
    //         "stop" => {
    //             break;
    //         }
    //         _ => {
    //             println!("Unknown command");
    //         }
    //     }
    // }
    // println!("Final state: {:?}", state);
}
