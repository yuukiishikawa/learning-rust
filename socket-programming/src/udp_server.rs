use std::net::UdpSocket;
use std::str;

pub fn server(address: &str) -> Result<(), failure::Error> {
  let server_socket = UdpSocket::bind(address)?;
}