use std::sync::{Arc, Mutex};

use crate::constants::DEFAULT_PORT;
use crate::network::NetworkState;
use crate::server::Server;
use crate::client::Client;

pub fn execute(network_state: Arc<Mutex<NetworkState>>, name: &str, args: Vec<&str>) {
	match name {
		"help" => {
			if !args.is_empty() {
				println!("Too many arguments were given");
				return;
			}

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
	- -l
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
- Quit the application
			"];

			for (i, name) in command_names.iter().enumerate() {
				println!("{}", name);
				println!("{}", command_descriptions[i]);
			}
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
					match args_iter.next() {
						Some(arg) => port = match arg.parse::<u16>() {
							Ok(x) => x,
							Err(_) => {
								println!("Invalid port");
								return;
							}
						},
						None => {
							println!("Expected argument");
							return;
						}
					}
				} else if *arg == "-pw" {
					match args_iter.next() {
						Some(arg) => password = (*arg).to_owned(),
						None => {
							println!("Expected argument");
							return;
						}
					}
				} else if *arg == "-u" {
					match args_iter.next() {
						Some(arg) => username = (*arg).to_owned(),
						None => {
							println!("Expected argument");
							return;
						}
					}
				} else if *arg == "-l" {
					logger = true;
				} else {
					println!("Invalid argument");
					return;
				}
			}

			println!("Opening server");

			*network_state.lock().unwrap() = NetworkState::Server(match Server::open(port, &password, &username, logger) {
				Ok(x) => x,
				Err(e) => {
					println!("{}", e);
					return;
				}
			});

			println!("Server open");
		}
		"join" => {
			if args.is_empty() {
				println!("Not enough arguments");
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
			let mut username = String::from("unknown");

			while let Some(arg) = it.next() {
				if *arg == "-pw" {
					match it.next() {
						Some(arg) => password = (*arg).to_owned(),
						None => {
							println!("Expected argument");
							return;
						}
					}
				} else if *arg == "-u" {
					match it.next() {
						Some(arg) => username = (*arg).to_owned(),
						None => {
							println!("Expected argument");
							return;
						}
					}
				} else {
					println!("Invalid argument");
					return;
				}
			}

			println!("Joining server");

			let client = NetworkState::Client(match Client::connect(args[0], &password, &username) {
				Ok(x) => x,
				Err(e) => {
					println!("{}", e);
					return;
				}
			});

			*network_state.lock().unwrap() = client;

			println!("Server joined");
		}
		"leave" => {
			if let NetworkState::Server(server) = &*network_state.lock().unwrap() {
				server.close();
			}

			match *network_state.lock().unwrap() {
				NetworkState::None => println!("You are not in a server"),
				_ => println!("Server left")
			}

			*network_state.lock().unwrap() = NetworkState::None;
		}
		"quit" => {
			if !args.is_empty() {
				println!("Too many arguments were given");
				return;
			}

			std::process::exit(0);
		}
		_ => println!("Unknown command '{}'", name)
	}
}
