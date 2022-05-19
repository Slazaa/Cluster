use std::sync::{Arc, Mutex};

use crate::network::NetworkState;
use crate::server::Server;
use crate::client::Client;
use crate::output::*;

pub fn execute(network_state: Arc<Mutex<NetworkState>>, name: &str, args: Vec<&str>) {
	match name {
		"host" => {
			if args.len() > 3 {
				output("Too much arguments were given");
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

			let mut port = 5000;
			
			match args.get(0) {
				Some(x) => port = match x.parse::<u16>() {
					Ok(x) => x,
					Err(_) => {
						output("Invalid port");
						return;
					}
				},
				_ => ()
			}

			output("Opening Cluster");

			*network_state.lock().unwrap() = NetworkState::Server(match Server::open(port) {
				Ok(x) => x,
				Err(e) => {
					output(&e);
					return;
				}
			});

			output("Cluster open");
		}
		"join" => {
			if args.len() < 1 {
				output("Not enough arguments");
				return;
			}

			if args.len() > 3 {
				output("Too much arguments were given");
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

			output("Joining Cluster");

			let client = NetworkState::Client(match Client::connect(args[0]) {
				Ok(x) => x,
				Err(e) => {
					output(&e);
					return;
				}
			});

			*network_state.lock().unwrap() = client;

			output("Cluster joined");
		}
		"leave" => {
			match &*network_state.lock().unwrap() {
				NetworkState::Server(server) => server.close(),
				_ => ()
			}

			match *network_state.lock().unwrap() {
				NetworkState::None => output("You are not in a Cluster"),
				_ => output("Cluster left")
			}

			*network_state.lock().unwrap() = NetworkState::None;
		}
		"quit" => std::process::exit(0),
		_ => output(&format!("Unknown command '{}'", name))
	}
}