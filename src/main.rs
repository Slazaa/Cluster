mod client;
mod command;
mod network;
mod server;

use std::io::{self, Write};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

use network::NetworkState;

fn main() {
	let network_state = Arc::new(Mutex::new(NetworkState::None));
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		let command_name = args[1].as_str();
		let command_args: Vec<&str> = args.iter()
			.skip(2)
			.map(|x| x.as_str())
			.collect();

		command::execute(Arc::clone(&network_state), command_name, command_args);
	}

	let network_state_clone = Arc::clone(&network_state);

	thread::spawn(move || {
		let network_state = network_state_clone;

		loop {
			match &*network_state.lock().unwrap() {
				NetworkState::Server(server) => {

				}
				NetworkState::Client(client) => {
					
				}
				_ => ()
			}
		}
	});
	
	loop {
		print!(" > ");

		io::stdout()
			.flush()
			.unwrap();

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.unwrap();

		input = input.trim().to_owned();

		if input.starts_with('/') {
			let command: Vec<&str> = input.split(' ')
				.collect();

			let mut command_name = command.first()
				.unwrap()
				.to_string();
				
			command_name.remove(0);

			let command_args: Vec<&str> = command.iter()
				.skip(1)
				.map(|&x| x)
				.collect();

			command::execute(Arc::clone(&network_state), &command_name, command_args);
		}
	}
}