//! Public networking.

use error::*;

use std::io::Read;
use std::net::SocketAddrV4;

use hyper::Client;

/// Parse the provided `addr` string (`<ip address>:<port>`) into an IPv4 socket (`SocketAddrV4`).
pub fn to_ipv4_socket(addr: String) -> LocksidianResult<SocketAddrV4> {
	match addr.parse::<SocketAddrV4>() {
		Ok(socket) => Ok(socket),
		Err(err) => Err(LocksidianError::from_err(err))
	}
}

/// HTTP call to the `monip.org` DNS over plain HTTP in order to discover our routable IP address.
pub fn get_public_ip() -> LocksidianResult<String> {
	let client = Client::new();
	let url = "http://monip.org/";
	let mut body: String = String::new();
	
	match client.get(url).send() {
		Ok(mut res) => match res.read_to_string(&mut body) {
			Ok(_) => {
				let parts: Vec<&str> = body.split("IP : ").collect();
				
				match parts.get(1) {
					Some(body) => {
						let parts: Vec<&str> = body.split("<br>").collect();
						
						match parts.get(0) {
							Some(ip) => Ok(format!("{}", ip)),
							None => Err(LocksidianError::new(String::from("Invalid body")))
						}
					},
					None => Err(LocksidianError::new(String::from("Invalid body")))
				}
			},
			Err(err) => Err(LocksidianError::from_err(err))
		},
		Err(err) => Err(LocksidianError::from_err(err))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn ipv4_string_should_be_parsed() {
		let ipv4 = String::from("127.0.0.1:8080");
		let socket = to_ipv4_socket(ipv4).unwrap();
		
		assert_eq!(&format!("{}", socket.ip()), "127.0.0.1");
		assert_eq!(socket.port(), 8080);
	}
	
	#[test]
	fn ipv6_string_should_not_be_parsed() {
		let ipv6 = String::from("2001:0db8:0000:85a3:0000:0000:ac1f:8001");
		let socket = to_ipv4_socket(ipv6);
		
		assert!(socket.is_err());
	}
	
	#[test]
	fn random_string_should_not_be_parsed() {
		let str = String::from("Hello World!");
		let socket = to_ipv4_socket(str);
		
		assert!(socket.is_err());
	}
}