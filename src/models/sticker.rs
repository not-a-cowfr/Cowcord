use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;
use crate::models::types::Snowflake;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StickerPack {
	pub id:               Snowflake,
	pub stickers:         Vec<Sticker>,
	pub name:             String,
	pub sku_id:           Snowflake,
	pub cover_sticker_id: Snowflake,
	pub description:      String,
	pub banner_asset_id:  Snowflake,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Sticker {
	pub id:          Snowflake,
	pub pack_id:     Snowflake,
	pub name:        String,
	pub description: Option<String>,
	/// comma separated list of keywords, official clients literally just use the name for this though
	pub tags:        String,
	pub r#type:      StickerTypes,
	pub format_type: StickerFormatTypes,
	pub available:   bool,
	pub guild_id:    Snowflake,
	pub user:        User,
	pub sort_value:  u16,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StickerTypes {
	STANDARD = 1,
	GUILD = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StickerFormatTypes {
	PNG = 1,
	APNG = 2, // not a typo
	LOTTIE = 3,
	/// GIF stickers are not available through the CDN, and must be accessed at https://media.discordapp.net/stickers/{sticker_id}.gif
	GIF = 4,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StickerItem {
	id:          Snowflake,
	name:        String,
	format_type: StickerFormatTypes,
}
