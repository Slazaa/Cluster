use std::io::{self, Write, Read};
use std::net::TcpStream;

use crate::constants::{DEFAULT_PORT, MAX_BUFFER_SIZE};
use crate::output::*;

pub struct Client {
	stream: TcpStream,
	username: String,
	server_username: String
}

impl Client {
	pub fn connect(address: &str, password: &str, username: &str) -> Result<Self, String> {
		let mut addr = address.to_owned();

		if !address.contains(':') {
			addr.push_str(":");
			addr.push_str(&DEFAULT_PORT.to_string());
		}

		let stream = match TcpStream::connect(addr) {
			Ok(x) => x,
			Err(_) => return Err("Invalid address".to_owned())
		};

		match stream.set_nonblocking(true) {
			Ok(x) => x,
			Err(_) => return Err("Failed setting non blocking".to_owned())
		}

		let mut client = Self {
			stream,
			username: username.to_owned(),
			server_username: "Server".to_owned()
		};

		client.send(&format!(r#"{{"password":"{}","username":"{}"}}"#, password, client.username));

		loop {
			match client.receive() {
				Ok(content) => {
					match content {
						Some(content) => {
							let value: serde_json::Value = match serde_json::from_str(&content) {
								Ok(x) => x,
								Err(_) => {
									output("Server sent invalid message");
									continue;
								}
							};
	
							if let Some(content) = value.get("valid") {
								if content.as_bool().unwrap() {
									return Ok(client);
								} else {
									return Err("Invalid password".to_owned());
								}
							}
						}
						None => ()
					}
				}
				Err(_) => ()
			}
		}
	}

	fn receive(&mut self) -> Result<Option<String>, String> {
		let mut message: Vec<u8> = Vec::new();

		loop {
			let mut buffer = [0; MAX_BUFFER_SIZE];
			let result = self.stream.read(&mut buffer);

			match result {
				Ok(received) => {
					if received < 1 {
						break;
					}

					for (i, c) in buffer.iter().enumerate() {
						if i >= received {
							continue;
						}

						if *c == '\n' as u8 {
							return Ok(Some(String::from_utf8(message.clone()).unwrap()));
						}

						message.push(*c);
					}
				}
				Err(e) => {
					if e.kind() != io::ErrorKind::WouldBlock {
						break;
					}

					return Ok(None);
				}
			}
		}

		return Err("Connection with host lost".to_string());
	}

	pub fn handle(&mut self) -> bool {
		match self.receive() {
			Ok(content) => {
				match content {
					Some(content) => {
						let value: serde_json::Value = match serde_json::from_str(&content) {
							Ok(x) => x,
							Err(_) => {
								output("Server sent invalid message");
								return true;
							}
						};

						if let Some(content) = value.get("message") {
							let message = content.as_str().unwrap();
							output(&format!("<{}> {}", self.server_username, message));
						}
					}
					None => ()
				}
			}
			Err(e) => {
				output(&e);
				return false;
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