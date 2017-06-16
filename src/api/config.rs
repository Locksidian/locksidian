//! Server configuration structure.

pub struct ServerConfig {
	pub local_only: bool,
	pub protected: bool,
	pub entrypoint: Option<String>
}