use std::net::TcpStream;

pub struct Client {
	stream: TcpStream
}

impl Client {
	pub fn connect(address: &str) -> Result<Self, String> {
		Ok(Self {
			stream: match TcpStream::connect(address) {
				Ok(x) => x,
				Err(_) => return Err("Invalid address".to_owned())
			}
		})
	}
}