use std::io::{self, Read};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

use crate::output::*;

pub struct Server {
	listener: TcpListener
}

impl Server {
	pub fn open(port: u16) -> Result<Self, String> {
		let mut address = "localhost:".to_owned();
		address.push_str(&port.to_string());

		let listener = TcpListener::bind(address).expect("Failed creating a listener");
		listener.set_nonblocking(true).expect("Failed setting non blocking");

		Ok(Self {
			listener
		})
	}

	fn handle_client(mut stream: TcpStream, address: SocketAddr) {
		let mut msg: Vec<u8> = Vec::new();

		loop {
			let mut buffer = [0; 10];

			match stream.read(&mut buffer) {
				Ok(received) => {
					if received < 1 {
						output(&format!("Client disconnected from: '{}'", address));
						return;
					}

					for (i, c) in buffer.iter().enumerate() {
						if i >= received {
							break;
						}

						if *c == '\n' as u8 {
							output(&format!("<{}> {}", address, String::from_utf8(msg).unwrap()));
							msg = Vec::new();
							continue;
						}

						msg.push(*c);
					}
				}
				Err(e) => {
					if e.kind() != io::ErrorKind::WouldBlock {
						output(&format!("Client disconnected from: '{}'", address));
						return;
					}
				}
			}
		}
	}

	pub fn handle(&mut self) {
		match self.listener.accept() {
			Ok((stream, address)) => {
				output(&format!("Client connected from: '{}'", address));

				thread::spawn(move || {
					Server::handle_client(stream, address);
				});
			}
			Err(_) => ()
		}
	}

	pub fn send(&mut self, message: &str) {

	}
}