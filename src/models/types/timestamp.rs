use iso8601_timestamp::Timestamp as isoTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub Option<isoTimestamp>);

impl Default for Timestamp {
	fn default() -> Self { Timestamp(None) }
}
