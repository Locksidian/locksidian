//! Blockchain version structure.

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
	
	pub fn version(&self) -> String {
		self.version.clone()
	}
}