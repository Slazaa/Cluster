use std::io::{self, Write, Read};
use std::net::TcpStream;

use crate::output::*;

pub struct Client {
	stream: TcpStream
}

impl Client {
	pub fn connect(address: &str) -> Result<Self, String> {
		let stream = TcpStream::connect(address).expect("Invalid address");
		stream.set_nonblocking(true).expect("Failed setting non blocking");

		Ok(Self {
			stream
		})
	}

	pub fn handle(&mut self) {
		let mut msg: Vec<u8> = Vec::new();
		let mut buffer = [0; 3];

		match self.stream.read(&mut buffer) {
			Ok(received) => {
				if received < 1 {
					output("Connection with host lost");
					return;
				}

				for (i, c) in buffer.iter().enumerate() {
					if i >= received {
						break;
					}

					if *c == '\n' as u8 {
						output(&format!("<Server> {}", String::from_utf8(msg).unwrap()));
						msg = Vec::new();
						continue;
					}

					msg.push(*c);
				}
			}
			Err(e) => {
				if e.kind() != io::ErrorKind::WouldBlock {
					output("Connection with host lost");
					return;
				}
			}
		}
	}

	pub fn send(&mut self, message: &str) {
		match write!(self.stream, "{}\n", message) {
			Ok(_) => (),
			Err(_) => println!("Failed sending message to server")
		}
	}
}