use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
// use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut address = String::from("127.0.0.1:");
    address.push_str(&args[1]);

    println!("{}", address);

    let listener = TcpListener::bind(address).unwrap();

    let con = ConnectionManager::new();
    con.start();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("Request {}", message);
        let response = "HTTP/1.1 200\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

trait State {}

struct Init {}
struct StandBy {}
// struct Connected {}
// struct ShuttingDown {}

impl State for Init {}
impl State for StandBy {}
// impl State for Connected {}
// impl State for ShuttingDown {}

pub struct ConnectionManager {
    state: Option<Box<dyn State>>,
}

impl ConnectionManager {
    // add code here
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            state: Some(Box::new(Init {})),
        }
    }

    pub fn start(self) -> ConnectionManager {
        ConnectionManager {
            state: Some(Box::new(StandBy {})),
        }
    }

    pub fn get_my_current_state(self) {
        self.state;
    }
    pub fn send_message(self) {}
    pub fn send_message_to_all_peer(self) {}
    pub fn handle_message(self) {}
    pub fn add_peer(self) {}
    pub fn remove_peer(self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        print!(stringify!(self.state));
        assert_eq!(2 + 2, 4);
    }
}
