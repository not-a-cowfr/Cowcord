#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PremiumReferral {
	pub id:                 Snowflake,
	pub user_id:            Snowflake,
	pub trial_id:           Snowflake,
	// pub subscription_trial: SubsciptionTrial,
	pub subscription_trial: Value,
	pub expires_at:         Timestamp,
	pub referrer_id:        Snowflake,
	pub referrer:           User,
	pub redeemed_at:        Timestamp,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PremiumReferralEligibility {
	pub referrals_remaining:        u8,
	pub sent_user_ids:              Vec<Snowflake>,
	pub refresh_at:                 Option<Timestamp>,
	pub has_eligible_friends:       bool,
	/// https://docs.discord.food/resources/premium-referral#premium-referral-recipient-status
	pub recipient_status:           HashMap<Snowflake, u8>,
	pub is_eligible_for_incentive:  bool,
	pub is_qualified_for_incentive: bool,
	/// https://docs.discord.food/resources/premium-referral#premium-referral-incentive-status
	pub referral_incentive_status:  u8,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumReferralRecipientStatus {
	REDEEMED = 1,
	PENDING = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumReferralIncentiveStatus {
	NOT_ELIGIBLE = 0,
	ELIGIBLE = 1,
	QUALIFIED = 2,
	COOLDOWN = 3,
	UNAPPLIED = 4,
}
