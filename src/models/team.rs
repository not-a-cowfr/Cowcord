#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Team {
	pub id:                        Snowflake,
	pub name:                      String,
	pub icon:                      Option<String>,
	pub owner_user_id:             Snowflake,
	pub members:                   Vec<Member>,
	pub payout_account_status:     Option<PayoutAccountStatus>,
	pub stripe_connect_account_id: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PayoutAccountStatus {
	UNSUBMITTED = 1,
	PENDING = 2,
	ACTION_REQUIRED = 3,
	ACTIVE = 4,
	BLOCKED = 5,
	SUSPENDED = 6,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Member {
	pub user:             User,
	pub team_id:          Snowflake,
	pub membership_state: MembershipState,
	pub role:             MemberRoleType,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MembershipState {
	INVITED = 1,
	ACCEPTED = 2,
}

pub enum MemberRoleType {
	admin,
	developer,
	read_only,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Payout {
	pub id:                                 Snowflake,
	pub user_id:                            Snowflake,
	pub amount:                             u32,
	pub status:                             PayoutStatus,
	pub period_start:                       Timestamp,
	pub period_end:                         Option<Timestamp>,
	pub payout_date:                        Option<Timestamp>,
	pub latest_tipalti_submission_response: Value,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PayoutStatus {
	OPEN = 1,
	PAID = 2,
	PENDING = 3,
	MANUAL = 4,
	CANCELLED = 5,
	DEFERRED = 6,
	DEFERRED_INTERNAL = 7,
	PROCESSING = 8,
	ERROR = 9,
	REJECTED = 10,
	RISK_REVIEW = 11,
	SUBMITTED = 12,
	PENDING_FUNDS = 13,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Company {
	pub id:   Snowflake,
	pub name: String,
}
