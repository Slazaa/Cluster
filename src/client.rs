use std::io::{self, Write, Read};
use std::net::TcpStream;

use crate::output::*;

pub struct Client {
	stream: TcpStream
}

impl Client {
	pub fn connect(address: &str) -> Result<Self, String> {
		let stream = match TcpStream::connect(address) {
			Ok(x) => x,
			Err(_) => return Err("Invalid address".to_owned())
		};

		match stream.set_nonblocking(true) {
			Ok(x) => x,
			Err(_) => return Err("Failed setting non blocking".to_owned())
		}

		Ok(Self {
			stream
		})
	}

	pub fn handle(&mut self) -> bool {
		const BUFFER_SIZE: usize = 256;

		let mut msg: Vec<u8> = Vec::new();
		let mut buffer = [0; BUFFER_SIZE];

		match self.stream.read(&mut buffer) {
			Ok(received) => {
				if received < 1 {
					output("Connection with host lost");
					return false;
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
					return false;
				}
			}
		}

		true
	}

	pub fn send(&mut self, message: &str) {
		match write!(self.stream, "{}\n", message) {
			Ok(_) => (),
			Err(_) => output("Failed sending message to host")
		}
	}
}