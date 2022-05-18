use std::sync::{Arc, Mutex};

use crate::network::NetworkState;
use crate::server::Server;
use crate::client::Client;

pub fn execute(network_state: Arc<Mutex<NetworkState>>, name: &str, args: Vec<&str>) {
	match name {
		"host" => {
			if args.len() < 1 {
				println!("Not enough arguments");
				return;
			}

			if args.len() > 3 {
				println!("Too much arguments were given");
				return;
			}
			
			let mut leave = false;

			match *network_state.lock().unwrap() {
				NetworkState::None => (),
				_ => leave = true
			}

			if leave {
				execute(Arc::clone(&network_state), "leave", vec![]);
			}

			let port = match args[0].parse::<u16>() {
				Ok(x) => x,
				Err(_) => {
					println!("Invalid port");
					return;
				}
			};

			println!("Opening Cluster");

			*network_state.lock().unwrap() = NetworkState::Server(match Server::open(port) {
				Ok(x) => x,
				Err(e) => {
					println!("{}", e);
					return;
				}
			});

			println!("Cluster open");
		}
		"join" => {
			if args.len() < 1 {
				println!("Not enough arguments");
				return;
			}

			if args.len() > 3 {
				println!("Too much arguments were given");
				return;
			}

			let mut leave = false;

			match *network_state.lock().unwrap() {
				NetworkState::None => (),
				_ => leave = true
			}

			if leave {
				execute(Arc::clone(&network_state), "leave", vec![]);
			}

			println!("Joining Cluster");

			*network_state.lock().unwrap() = NetworkState::Client(match Client::connect(args[0]) {
				Ok(x) => x,
				Err(e) => {
					println!("{}", e);
					return;
				}
			});

			println!("Cluster joined");
		}
		"leave" => {
			match *network_state.lock().unwrap() {
				NetworkState::None => println!("You are not in a Cluster"),
				_ => println!("Cluster left")
			}

			*network_state.lock().unwrap() = NetworkState::None;
		}
		"quit" => std::process::exit(0),
		_ => println!("Unknown command '{}'", name)
	}
}