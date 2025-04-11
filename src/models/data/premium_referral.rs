#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::types::{Snowflake, Timestamp};
use super::user::User;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct PremiumReferral {
	pub id:                 Snowflake,
	pub user_id:            Snowflake,
	pub trial_id:           Snowflake,
	pub subscription_trial: SubsciptionTrial, // todo
	pub expires_at:         Timestamp,
	pub referrer_id:        Snowflake,
	pub referrer:           User,
	pub redeemed_at:        Timestamp,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct PremiumReferralEligibility {
	pub referrals_remaining:        u8,
	pub sent_user_ids:              Vec<Snowflake>,
	pub refresh_at:                 Option<Timestamp>,
	pub has_eligible_friends:       bool,
	pub recipient_status:           HashMap<Snowflake, u8>, // https://docs.discord.sex/resources/premium-referral#premium-referral-recipient-status
	pub is_eligible_for_incentive:  bool,
	pub is_qualified_for_incentive: bool,
	pub referral_incentive_status:  u8, // https://docs.discord.sex/resources/premium-referral#premium-referral-incentive-status
}

pub enum PremiumReferralRecipientStatus {
	REDEEMED = 1,
	PENDING = 2,
}

pub enum PremiumReferralIncentiveStatus {
	NOT_ELIGIBLE = 0,
	ELIGIBLE = 1,
	QUALIFIED = 2,
	COOLDOWN = 3,
	UNAPPLIED = 4,
}
