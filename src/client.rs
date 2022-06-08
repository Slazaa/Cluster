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
			addr.push(':');
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
					if let Some(content) = content {
						let value: serde_json::Value = match serde_json::from_str(&content) {
							Ok(x) => x,
							Err(_) => {
								output("Server sent invalid message");
								continue;
							}
						};

						if let Some(content) = value.get("username") {
							let content = content.as_str().unwrap();

							if !content.is_empty() {
								client.server_username = content.to_owned();
							}
						}

						if let Some(content) = value.get("valid") {
							if !content.as_bool().unwrap() {
								return Err("Invalid password".to_owned());
							}
						}

						break;
					}
				}
				Err(_) => return Err("Connection with host lost".to_string())
			}
		}

		Ok(client)
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

						if *c == b'\n' {
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

		Err("Connection with host lost".to_string())
	}

	pub fn send(&mut self, message: &str) {
		if writeln!(self.stream, "{}", message).is_err() {
			output("Failed sending message to host");
		}
	}

	pub fn handle(&mut self) -> bool {
		match self.receive() {
			Ok(content) => {
				if let Some(content) = content {
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
			}
			Err(e) => {
				output(&e);
				return false;
			}
		}

		true
	}
}