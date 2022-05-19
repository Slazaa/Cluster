use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::output::*;

pub struct Server {
	listener: TcpListener,
	clients: Vec<Arc<Mutex<(TcpStream, SocketAddr)>>>,
	closed: Arc<Mutex<bool>>,
}

impl Server {
	pub fn open(port: u16) -> Result<Self, String> {
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
			clients: Vec::new(),
			closed: Arc::new(Mutex::new(false)),
		})
	}

	pub fn close(&self) {
		*self.closed.lock().unwrap() = true;
	}

	fn handle_client(client: Arc<Mutex<(TcpStream, SocketAddr)>>, closed: Arc<Mutex<bool>>) {
		const BUFFER_SIZE: usize = 256;

		let mut msg: Vec<u8> = Vec::new();

		while !closed.lock().unwrap().clone() {
			let mut buffer = [0; BUFFER_SIZE];
			let result = client.lock().unwrap().0.read(&mut buffer);

			match result {
				Ok(received) => {
					if received < 1 {
						output(&format!("Client disconnected from: '{}'", client.lock().unwrap().1));
						return;
					}

					for (i, c) in buffer.iter().enumerate() {
						if i >= received {
							break;
						}

						if *c == '\n' as u8 {
							output(&format!("<{}> {}", client.lock().unwrap().1, String::from_utf8(msg).unwrap()));
							msg = Vec::new();
							continue;
						}

						msg.push(*c);
					}
				}
				Err(e) => {
					if e.kind() != io::ErrorKind::WouldBlock {
						output(&format!("Client disconnected from: '{}'", client.lock().unwrap().1));
						return;
					}
				}
			}
		}
	}

	pub fn handle(&mut self) {
		match self.listener.accept() {
			Ok(client) => {
				self.clients.push(Arc::new(Mutex::new(client)));
			
				output(&format!("Client connected from: '{}'", self.clients.last().unwrap().lock().unwrap().1));

				let client = Arc::clone(&self.clients.last().unwrap());
				let closed = Arc::clone(&self.closed);

				thread::spawn(move || {
					Server::handle_client(client, closed);
				});
			}
			Err(_) => ()
		}
	}

	pub fn send(&mut self, message: &str) {
		for mut client in self.clients.iter().map(|x| x.lock().unwrap()) {
			match write!(client.0, "{}\n", message) {
				Ok(_) => (),
				Err(_) => output("Failed sending message to host")
			}
		}
	}
}