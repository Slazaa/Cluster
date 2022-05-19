use std::io::{self, Write};

use terminal::cursor;
use terminal::utils::Position;

pub fn output(message: &str) {
	cursor::set_pos(Position::new(0, cursor::get_pos().y));

	println!("{}", message);
	print!(" > ");

	io::stdout()
		.flush()
		.unwrap();
}