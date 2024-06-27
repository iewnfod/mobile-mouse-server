use std::thread;

use local_ip_address::list_afinet_netifas;
use ws::Message;

use crate::controller::{Controller, MouseKey};

const DEFAULT_PORT: usize = 57632;

fn get_ip_address() -> Vec<String> {
	if let Ok(network_interfaces) = list_afinet_netifas() {
		let mut ips = vec![];
		for (name, ip) in network_interfaces.iter() {
			if ip.is_ipv4() && !ip.is_loopback() {
				ips.push(ip.to_string());
				println!("{}:\t{:?}", name, ip);
			}
		}
		return ips
	} else {
		return vec!["127.0.0.1".to_string()]
	}
}

pub struct Server {
	hosts: Vec<String>,
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
			hosts: get_ip_address(),
			port,
		}
	}

	pub fn run(&self) {
		for address in self.get_addresses() {
			thread::spawn(move || {
				println!("Server started on {}", &address);
				if let Err(error) = ws::listen(&address, |out| {
					move |msg: Message| {
						let v: serde_json::Value = serde_json::from_str(msg.as_text().unwrap()).unwrap();
						if let Some(t) = v.get("type") {
							let raw_cmd = t.to_string();
							let cmd = raw_cmd[1..raw_cmd.len()-1].to_string();

							println!("{}: {:?}", &cmd, &v);

							let mut controller = Controller::new();

							match cmd.as_str() {
								"move" => {
									controller.move_mouse(
										v["x"].as_f64().unwrap(),
										v["y"].as_f64().unwrap()
									);
								},
								"left click" => {
									controller.click(MouseKey::Left);
								},
								"left press" => {
									controller.press(MouseKey::Left);
								},
								"left release" => {
									controller.release(MouseKey::Left);
								},
								"right click" => {
									controller.click(MouseKey::Right);
								},
								"right press" => {
									controller.press(MouseKey::Right);
								},
								"right release" => {
									controller.release(MouseKey::Right);
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
					println!("Failed to start server on {}: {:?}", &address, error);
				}
			});
		}
	}

	pub fn get_addresses(&self) -> Vec<String> {
		let mut addresses = vec![];
		for h in self.hosts.iter() {
			addresses.push(format!("{}:{}", h, self.port));
		}
		addresses
	}
}
