mod client;
mod command;
mod constants;
mod network;
mod output;
mod server;

use std::io::{self, Write};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

use terminal::cursor;
use terminal::utils::Position;

use network::NetworkState;

use crate::constants::MAX_MESSAGE_SIZE;

fn main() {
	println!("Welcome to Cluster!");
	println!("Type '/help' to view a list of the available commands.");

	let network_state = Arc::new(Mutex::new(NetworkState::None));
	let args: Vec<String> = env::args().collect();

	println!();

	let mut input = String::new();

	if args.len() > 1 {
		let command: Vec<&str> = args.iter()
			.skip(1)
			.map(|x| x.as_str())
			.collect();

		input.push('/');
		input.push_str(command.join(" ").as_str());
	}

	let network_state_clone = Arc::clone(&network_state);

	thread::spawn(move || {
		let network_state = network_state_clone;

		loop {
			let mut leave = false;

			match &mut *network_state.lock().unwrap() {
				NetworkState::Server(server) => {
					server.handle();
				}
				NetworkState::Client(client) => {
					if !client.handle() {
						leave = true;
					}
				}
				_ => ()
			}

			if leave {
				*network_state.lock().unwrap() = NetworkState::None;
			}
		}
	});

	loop {
		if input.starts_with('/') {
			let command: Vec<&str> = input.split(' ')
				.collect();

			let mut command_name = command.first()
				.unwrap()
				.to_string();
				
			command_name.remove(0);

			let command_args: Vec<&str> = command.iter()
				.skip(1)
				.copied()
				.collect();

			command::execute(Arc::clone(&network_state), &command_name, command_args);
		} else if input.is_empty() {
			cursor::set_pos(Position::new(0, cursor::get_pos().y - 1));
		} else if input.len() < MAX_MESSAGE_SIZE {
			let message = format!(r#"{{"message":"{}"}}"#, input);

			match &mut *network_state.lock().unwrap() {
				NetworkState::Server(server) => server.send_all(&message),
				NetworkState::Client(client) => client.send(&message),
				_ => ()
			}
		} else {
			println!("Message too long");
		}

		print!(" > ");

		io::stdout()
			.flush()
			.unwrap();

		input.clear();

		io::stdin()
			.read_line(&mut input)
			.unwrap();

		input = input.trim().to_owned();
	}
}