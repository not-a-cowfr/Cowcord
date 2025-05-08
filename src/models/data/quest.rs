#![allow(non_camel_case_types)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::types::{Snowflake, Timestamp};
use crate::{bitflags, enum_number};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Quest {
	pub id:               Snowflake,
	pub config:           Config,
	pub user_status:      Option<UserStatus>,
	pub targeted_content: Option<Vec<u8>>,
	pub preview:          bool,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum ContentType {
		GIFT_INVENTORY_SETTINGS_BADGE = 0,
		QUEST_BAR = 1,
		QUEST_INVENTORY_CARD = 2,
		QUESTS_EMBED = 3,
		ACTIVITY_PANEL = 4,
		QUEST_LIVE_STREAM = 5,
		MEMBERS_LIST = 6,
		QUEST_BADGE = 7,
		GIFT_INVENTORY_FOR_YOU = 8,
		GIFT_INVENTORY_OTHER = 9,
		QUEST_BAR_V2 = 10,
		QUEST_HOME_DESKTOP = 11,
		QUEST_HOME_MOBILE = 12,
		QUEST_BAR_MOBILE = 13,
		THIRD_PARTY_APP = 14,
		QUEST_BOTTOM_SHEET = 15,
		QUEST_EMBED_MOBILE = 16,
		QUEST_HOME_MOVE_CALLOUT = 17,
		DISCOVERY_SIDEBAR = 18,
		QUEST_SHARE_LINK = 19,
		CONNECTIONS_MODAL = 20,
		DISCOVERY_COMPASS = 21,
		TROPHY_CASE_CARD = 22,
		VIDEO_MODAL = 23,
		VIDEO_MODAL_END_CARD = 24,
		REWARD_MODAL = 25,
		EXCLUDED_QUEST_EMBED = 26,
		VIDEO_MODAL_MOBILE = 27,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
	/// https://docs.discord.food/resources/quests#quest-config-version
	pub config_version: u8,
	pub id:             Snowflake,
	pub starts_at:      Timestamp,
	pub expires_at:     Timestamp,
	/// https://docs.discord.food/resources/quests#quest-feature
	pub features:       Vec<u8>,
	pub experiments:    Rollout,
	pub application:    Application,
	pub assets:         Assets,
	pub colors:         Gradient,
	pub messages:       Messages,
	pub task_config:    TaskConfig,
	pub rewards_config: RewardsConfig,
	pub video_metadata: VideoMetadata,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum ConfigVersion {
		Active = 1,
		Discontinued = 2,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum Feature {
		POST_ENROLLMENT_CTA = 1,
		PLAYTIME_CRITERIA = 2,
		QUEST_BAR_V2 = 3,
		// EXCLUDE_MINORS = 4,
		EXCLUDE_RUSSIA = 5,
		IN_HOUSE_CONSOLE_QUEST = 6,
		MOBILE_CONSOLE_QUEST = 7,
		START_QUEST_CTA = 8,
		REWARD_HIGHLIGHTING = 9,
		FRACTIONS_QUEST = 10,
		ADDITIONAL_REDEMPTION_INSTRUCTIONS = 11,
		PACING_V2 = 12,
		DISMISSAL_SURVEY = 13,
		MOBILE_QUEST_DOCK = 14,
		QUESTS_CDN = 15,
		PACING_CONTROLLER = 16,
		QUEST_HOME_FORCE_STATIC_IMAGE = 17,
		VIDEO_QUEST_FORCE_HLS_VIDEO = 18,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Rollout {
	pub rollout:   Option<String>,
	pub targeting: Option<String>,
	pub preview:   Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Application {
	pub id:   Snowflake,
	pub name: String,
	pub link: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Assets {
	pub hero:                 String,
	pub hero_video:           Option<String>,
	pub quest_bar_hero:       String,
	pub quest_bar_hero_video: Option<String>,
	pub game_tile:            String,
	pub logotype:             String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Gradient {
	// both are in hex format
	pub primary:   String,
	pub secondary: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Messages {
	pub quest_name:     String,
	pub game_title:     String,
	pub game_publisher: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TaskConfig {
	/// https://docs.discord.food/resources/quests#quest-task-config-type
	pub r#type:                   u8,
	pub join_operator:            String, // "and" or "or"
	pub enrollment_url:           String,
	pub developer_application_id: Snowflake,
	pub tasks:                    HashMap<String, Task>,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum TaskConfigType {
		FIRST_PARTY = 1,
		THIRD_PARTY = 2,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Task {
	/// https://docs.discord.food/resources/quests#quest-task-event-name
	pub event_name:   String,
	pub target:       u32,
	pub external_ids: Vec<String>,
	pub description:  String,
}

pub enum TaskEventName {
	STREAM_ON_DESKTOP,
	PLAY_ON_DESKTOP,
	PLAY_ON_DESKTOP_V2,
	PLAY_ON_XBOX,
	PLAY_ON_PLAYSTATION,
	WATCH_VIDEO,
	PLAY_ACTIVITY,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RewardsConfig {
	/// https://docs.discord.food/resources/quests#quest-reward-assignment-method
	pub assignment_method: u8,
	pub rewards:           Vec<Reward>,
	pub rewards_expire_at: Option<Timestamp>,
	/// https://docs.discord.food/resources/quests#quest-platform-type
	pub platforms:         Vec<u8>,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum RewardAssignmentMethod {
		ALL = 1,
		TIERED = 2,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum PlatformType {
		CROSS_PLATFORM = 0,
		XBOX = 1,
		PLAYSTATION = 2,
		SWITCH = 3,
		PC = 4,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Reward {
	/// https://docs.discord.food/resources/quests#quest-reward-type
	pub r#type:             u8,
	pub sku_id:             Snowflake,
	pub asset:              Option<String>,
	pub asset_video:        Option<String>,
	pub messages:           RewardMessages,
	pub approximate_count:  Option<u32>,
	pub redemption_link:    Option<String>,
	pub expires_at:         Option<Timestamp>,
	pub expires_at_premium: Option<Timestamp>,
	/// https://docs.discord.food/resources/quests#quest-reward-expiration-mode
	pub expiration_mode:    u8,
	pub orb_quantity:       u32,
	pub quantity:           u8,
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum RewardType {
		REWARD_CODE = 1,
		IN_GAME = 2,
		COLLECTIBLE = 3,
		VIRTUAL_CURRENCY = 4,
		FRACTIONAL_PREMIUM = 5,
		_ => Unknown(u8),
	}
}

enum_number! {
	#[derive(Deserialize, Serialize)]
	#[serde(from = "u8", into = "u8")]
	pub enum ExpirationMode {
		NORMAL = 1,
		PREMIUM_EXTENSION = 2,
		PREMIUM_PERMANENT = 3,
		_ => Unknown(u8),
	}
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RewardMessages {
	pub name:                                       String,
	pub name_with_article:                          String,
	pub reward_redemption_instructions_by_platform: HashMap<u8, String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VideoMetadata {
	pub messages: VideoMessages,
	pub assets:   VideoAssets,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VideoMessages {
	pub video_title:                String,
	pub video_end_cta_title:        String,
	pub video_end_cta_subtitle:     String,
	pub video_end_cta_button_label: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VideoAssets {
	pub video_player_video_hls:      Option<String>,
	pub video_player_video:          String,
	pub video_player_thumbnail:      Option<String>,
	pub video_player_video_low_res:  String,
	pub video_player_caption:        String,
	pub video_player_transcript:     String,
	pub quest_bar_preview_video:     Option<String>,
	pub quest_bar_preview_thumbnail: Option<String>,
	pub quest_home_video:            Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClaimedQuest {
	pub id:          Snowflake,
	pub config:      ClaimedConfig,
	pub user_status: UserStatus,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClaimedConfig {
	pub id:         Snowflake,
	pub starts_at:  Timestamp,
	pub expires_at: Timestamp,
	/// https://docs.discord.food/resources/quests#quest-feature
	pub features:   Vec<u8>,
	pub colors:     Gradient,
	pub assets:     Assets,
	pub messages:   Messages,
	pub rewards:    Vec<ClaimedQuestReward>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClaimedQuestReward {
	/// https://docs.discord.food/resources/quests#quest-reward-type
	pub r#type:              u8,
	pub sku_id:              Snowflake,
	pub name:                String,
	pub name_with_article:   String,
	pub asset:               String,
	pub asset_video:         Option<String>,
	pub orb_quantity:        Option<u8>,
	/// "collectible object" but not the user collectibles object, idk man just fix it when it gets used
	pub collectible_product: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserStatus {
	pub user_id:                  Snowflake,
	pub quest_id:                 Snowflake,
	pub enrolled_at:              Timestamp,
	pub completed_at:             Option<Timestamp>,
	pub claimed_at:               Option<Timestamp>,
	pub claimed_tier:             Option<u8>,
	pub last_stream_heartbeat_at: Option<Timestamp>,
	pub stream_progress_seconds:  Timestamp,
	/// https://docs.discord.food/resources/quests#dismissible-quest-content-flags
	pub dismissed_quest_content:  u8,
	pub progress:                 HashMap<String, TaskProgress>,
}

bitflags! {
  pub struct DismissibleQuestContentFlags: u64 {
	const GIFT_INVENTORY_SETTINGS_BADGE = 1 << 0;
	const QUEST_BAR = 1 << 1;
	const ACTIVITY_PANEL = 1 << 2;
		const QUEST_LIVE_STREAM = 1 << 3;
  }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TaskProgress {
	pub event_name:   String,
	pub value:        u32,
	pub updated_at:   Timestamp,
	pub completed_at: Option<Timestamp>,
	pub heartbeat:    Option<TaskHeartbeat>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TaskHeartbeat {
	pub last_beat_at: Timestamp,
	pub expires_at:   Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RewardCode {
	pub quest_id:   Snowflake,
	pub code:       String,
	/// https://docs.discord.food/resources/quests#quest-platform-type
	pub platform:   String,
	pub user_id:    Snowflake,
	pub claimed_at: Timestamp,
	pub tier:       Option<u8>,
}
