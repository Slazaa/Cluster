use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::constants::MAX_BUFFER_SIZE;
use crate::output::*;

pub struct ClientInfos {
	pub stream: TcpStream,
	pub address: SocketAddr,
	pub username: String
}

pub struct Server {
	listener: TcpListener,
	password: String,
	username: String,
	//logger: bool,
	clients: Vec<Arc<Mutex<ClientInfos>>>,
	closed: Arc<Mutex<bool>>,
}

impl Server {
	pub fn open(port: u16, password: &str, username: &str, _logger: bool) -> Result<Self, String> {
		let mut address = "0.0.0.0:".to_owned();
		address.push_str(&port.to_string());

		let listener = match TcpListener::bind(address) {
			Ok(x) => x,
			Err(_) => return Err("Failed creating a listener".to_owned())
		};

		match listener.set_nonblocking(true) {
			Ok(x) => x,
			Err(_) => return Err("Failed setting non blocking".to_owned())
		}

		Ok(Self {
			listener,
			password: password.to_owned(),
			username: username.to_owned(),
			//logger,
			clients: Vec::new(),
			closed: Arc::new(Mutex::new(false)),
		})
	}

	pub fn close(&self) {
		*self.closed.lock().unwrap() = true;
	}

	fn receive(client: &Arc<Mutex<ClientInfos>>) -> Result<Option<String>, String> {
		let mut message: Vec<u8> = Vec::new();

		loop {
			let mut buffer = [0; MAX_BUFFER_SIZE];
			let result = client.lock().unwrap().stream.read(&mut buffer);

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
							return Ok(Some(String::from_utf8(message).unwrap()));
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

		return Err(format!("{} left", client.lock().unwrap().username));
	}

	pub fn send(stream: &mut TcpStream, message: &str) {
		if writeln!(stream, "{}", message).is_err() {
			output("Failed sending message to client");
		}
	}

	pub fn send_all(&mut self, message: &str) {
		for mut client in self.clients.iter().map(|x| x.lock().unwrap()) {
			Server::send(&mut client.stream, message);
		}
	}

	fn handle_client(client: Arc<Mutex<ClientInfos>>, closed: Arc<Mutex<bool>>, password: &str, username: &str) {
		match Server::receive(&client) {
			Ok(content) => {
				if let Some(content) = content {
					let value: serde_json::Value = match serde_json::from_str(&content) {
						Ok(x) => x,
						Err(_) => {
							output("Client sent invalid message");
							return;
						}
					};

					if let Some(content) = value.get("password") {
						if content.as_str().unwrap() == password {
							Server::send(&mut client.lock().unwrap().stream, &format!(r#"{{"username":"{}","valid":true}}"#, username));
						} else {
							Server::send(&mut client.lock().unwrap().stream, r#"{"valid":false}"#);
							return;
						}
					}

					if let Some(content) = value.get("username") {
						client.lock().unwrap().username = content.as_str().unwrap().to_owned();
					}
				}
			}
			Err(_) => return
		}

		output(&format!("{} joined", client.lock().unwrap().username));

		while !*closed.lock().unwrap() {
			match Server::receive(&client) {
				Ok(content) => {
					if let Some(content) = content {
						let value: serde_json::Value = match serde_json::from_str(&content) {
							Ok(x) => x,
							Err(_) => {
								output("Client sent invalid message");
								continue;
							}
						};

						if let Some(content) = value.get("message") {
							let message = content.as_str().unwrap();
							output(&format!("<{}> {}", client.lock().unwrap().username, message));
						}
					}
				}
				Err(e) => {
					output(&e);
					return;
				}
			}
		}
	}

	pub fn handle(&mut self) {
		let remove_client = Arc::new(Mutex::new(None));
		let remove_client_clone = Arc::clone(&remove_client);

		if let Ok((stream, address)) = self.listener.accept() {
			self.clients.push(Arc::new(Mutex::new(ClientInfos { stream, address, username: "unknown".to_owned() })));
			let index = self.clients.len();

			let client_clone = Arc::clone(self.clients.last().unwrap());
			let closed = Arc::clone(&self.closed);
			let password = self.password.clone();
			let username = self.username.clone();

			thread::spawn(move || {
				Server::handle_client(client_clone, closed, &password, &username);
				*remove_client_clone.lock().unwrap() = Some(index);
			});
		}

		let remove_client = *remove_client.lock().unwrap();

		if let Some(index) = remove_client {
			self.clients.remove(index);
		}
	}
}