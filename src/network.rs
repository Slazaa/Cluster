use crate::server::Server;
use crate::client::Client;

pub enum NetworkState {
	Server(Server),
	Client(Client),
	None
}