use serde::Deserialize;

#[derive(Deserialize)]
pub struct Versions {
	pub client_version: u32,
	pub server_version: u32,
	pub data_version:   u32,
}
