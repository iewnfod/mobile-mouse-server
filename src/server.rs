use std::net::{IpAddr, UdpSocket};
use ws::Message;

use crate::controller::Controller;

const DEFAULT_PORT: usize = 57632;

fn get_ip_address() -> Result<IpAddr, Box<dyn std::error::Error>> {
	let socket = UdpSocket::bind("0.0.0.0:0")?;
	socket.connect("8.8.8.8:80")?;
	Ok(socket.local_addr()?.ip())
}

pub struct Server {
	host: String,
	port: usize,
}

impl Default for Server {
	fn default() -> Self {
		Self::new(DEFAULT_PORT)
	}
}

impl Server {
	pub fn new(port: usize) -> Self {
		Server {
			host: get_ip_address().unwrap().to_string(),
			port,
		}
	}

	pub fn run(&self) {
		if let Err(error) = ws::listen(self.get_serve_address(), |out| {
			move |msg: Message| {
				let v: serde_json::Value = serde_json::from_str(msg.as_text().unwrap()).unwrap();
				if let Some(t) = v.get("type") {
					let raw_cmd = t.to_string();
					let cmd = raw_cmd[1..raw_cmd.len()-1].to_string();
					println!("command type: {}", &cmd);
					let mut controller = Controller::new();
					match cmd.as_str() {
						"move" => {
							controller.move_mouse(v["x"].as_i64().unwrap(), v["y"].as_i64().unwrap());
						},
						_ => {
							println!("Invalid operation: {}", &cmd);
							out.send(format!("Invalid operation: {}", &cmd)).unwrap();
						}
					}
				}
				out.broadcast(msg)
			}
		}) {
			println!("Failed to start server: {:?}", error);
		}
	}

	pub fn get_serve_address(&self) -> String {
		format!("{}:{}", self.host, self.port)
	}
}
