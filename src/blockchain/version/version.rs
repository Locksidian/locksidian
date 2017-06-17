//! Blockchain version structure.

#![allow(dead_code)]

#[derive(
	Debug, Clone,
	Serialize, Deserialize
)]
pub struct Version {
	package: String,
	version: String,
	description: String,
	authors: String
}

impl Version {
	
	pub fn new(package: &str, version: &str, description: &str, authors: &str) -> Self {
		Version {
			package: String::from(package),
			version: String::from(version),
			description: String::from(description),
			authors: String::from(authors)
		}
	}
	
	pub fn package(&self) -> String {
		self.package.clone()
	}
	
	pub fn version(&self) -> String {
		self.version.clone()
	}
	
	pub fn description(&self) -> String {
		self.description.clone()
	}
	
	pub fn authors(&self) -> String {
		self.description.clone()
	}
}