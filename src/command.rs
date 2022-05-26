use std::sync::{Arc, Mutex};

use crate::constants::DEFAULT_PORT;
use crate::network::NetworkState;
use crate::server::Server;
use crate::client::Client;
use crate::output::*;

pub fn execute(network_state: Arc<Mutex<NetworkState>>, name: &str, args: Vec<&str>) {
	match name {
		"help" => {
			let command_names = ["help", "host", "join", "leave", "quit"];
			let command_descriptions = ["\
- Shows the available commands
- Arguments:
	- [command_name]
",
"\
- Host a server
- Arguments:
	- -p <port>
	- -pw <password>
	- -u <username>
",
"\
- Join a server
- Arguments:
	- <address>
	- -pw <password>
	- -u <username>
",
"\
- Leave the current server
",
"\
- Quit the software
			"];

			if args.is_empty() {
				for (i, name) in command_names.iter().enumerate() {
					output(name);
					output(command_descriptions[i]);
				}

				return;
			}

			// TODO: Individual commands
		}
		"host" => {
			let mut leave = false;

			match *network_state.lock().unwrap() {
				NetworkState::None => (),
				_ => leave = true
			}

			if leave {
				execute(Arc::clone(&network_state), "leave", vec![]);
			}

			let mut args_iter = args.iter();

			let mut port = DEFAULT_PORT;
			let mut password = String::new();
			let mut username = String::new();
			let mut logger = false;

			while let Some(arg) = args_iter.next() {
				if *arg == "-p" {
					match args_iter.nth(0) {
						Some(arg) => port = match arg.parse::<u16>() {
							Ok(x) => x,
							Err(_) => {
								output("Invalid port");
								return;
							}
						},
						None => {
							output("Expected argument");
							return;
						}
					}
				} else if *arg == "-pw" {
					match args_iter.nth(0) {
						Some(arg) => password = (*arg).to_owned(),
						None => {
							output("Expected argument");
							return;
						}
					}
				} else if *arg == "-u" {
					match args_iter.nth(0) {
						Some(arg) => username = (*arg).to_owned(),
						None => {
							output("Expected argument");
							return;
						}
					}
				} else if *arg == "-l" {
					logger = true;
				} else {
					output("Invalid argument");
					return;
				}
			}

			output("Opening Cluster");

			*network_state.lock().unwrap() = NetworkState::Server(match Server::open(port, &password, &username, logger) {
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

			let mut leave = false;

			match *network_state.lock().unwrap() {
				NetworkState::None => (),
				_ => leave = true
			}

			if leave {
				execute(Arc::clone(&network_state), "leave", vec![]);
			}

			let mut it = args.iter().skip(1);

			let mut password = String::new();
			let mut username = String::new();

			while let Some(arg) = it.next() {
				if *arg == "-pw" {
					match it.nth(0) {
						Some(arg) => password = (*arg).to_owned(),
						None => {
							output("Expected argument");
							return;
						}
					}
				} else if *arg == "-u" {
					match it.nth(0) {
						Some(arg) => username = (*arg).to_owned(),
						None => {
							output("Expected argument");
							return;
						}
					}
				} else {
					output("Invalid argument");
					return;
				}
			}

			output("Joining Cluster");

			let client = NetworkState::Client(match Client::connect(args[0], &password, &username) {
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