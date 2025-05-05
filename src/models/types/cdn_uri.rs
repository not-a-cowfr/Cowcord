use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CdnUri(Url);

impl CdnUri {
	pub fn new(uri: impl AsRef<str>) -> Result<Self, CdnUriError> {
		let url = Url::parse(uri.as_ref())?;

		if !url
			.host_str()
			.map_or(false, |host| host == "cdn.discordapp.com")
		{
			return Err(CdnUriError::InvalidUri(
				"URI must be from discord".to_string(),
			));
		}

		if url.scheme() != "https" {
			return Err(CdnUriError::InvalidUri("URI must use https".to_string()));
		}

		Ok(CdnUri(url))
	}

	pub fn as_str(&self) -> &str { self.0.as_str() }
}

impl FromStr for CdnUri {
	type Err = CdnUriError;

	fn from_str(s: &str) -> Result<Self, Self::Err> { CdnUri::new(s) }
}

impl AsRef<str> for CdnUri {
	fn as_ref(&self) -> &str { self.as_str() }
}
