use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp"=> match role {
            "server"=>{
                tcp_server::serve(address).unwrap_or_else(|e|error!("{}", e));

            }
            "client"=>{
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _=> {
                missind_role();
            }
        },
        "udp"=>match role{
            "server"=>{
                
            }
            "client"=>{

            }
            _=> {
                missind_role();
            }
        },
        _ => {
            error!("Please specify tsp or client on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missind_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}