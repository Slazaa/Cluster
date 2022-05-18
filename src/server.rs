use std::net::TcpListener;

pub struct Server {
	listener: TcpListener
}

impl Server {
	pub fn open(port: u16) -> Result<Self, String> {
		let mut address = "localhost:".to_owned();
		address.push_str(&port.to_string());

		Ok(Self {
			listener: match TcpListener::bind(address) {
				Ok(x) => x,
				Err(_) => return Err("Failed creating a listener".to_owned())
			}
		})
	}
}