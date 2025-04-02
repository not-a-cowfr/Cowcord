use std::collections::HashMap;

use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use super::user::CustomStatusProto;
use crate::models::app::Versions;
use crate::models::chat::DmSpamFilterV2;
use crate::models::guild::AllGuildSettings;
use crate::models::types::Snowflake;

#[derive(Deserialize)]
pub struct UserSettings {
	pub activity_restricted_guild_ids:              Vec<Snowflake>,
	pub activity_joining_restricted_guild_ids:      Vec<Snowflake>,
	pub afk_timeout:                                u32,
	pub allow_accessibility_detection:              bool,
	pub allow_activity_party_privacy_friends:       bool,
	pub allow_activity_party_privacy_voice_channel: bool,
	pub animate_emoji:                              bool,
	pub animate_stickers:                           u8, // https://docs.discord.sex/resources/user-settings#sticker-animation-option
	pub contact_sync_enabled:                       bool,
	pub convert_emoticons:                          bool,
	pub custom_status:                              Option<CustomStatus>,
	pub default_guilds_restricted:                  bool,
	pub detect_platform_accounts:                   bool,
	pub developer_mode:                             bool,
	pub disable_games_tab:                          bool,
	pub enable_tts_command:                         bool,
	pub explicit_content_filter:                    u8,
	pub friend_discovery_flags:                     u8,
	pub friend_source_flags:                        Option<FriendSourceFlags>,
	pub gif_auto_play:                              bool,
	pub guild_folders:                              Vec<GuildFolder>,
	pub inline_attachment_media:                    bool,
	pub inline_embed_media:                         bool,
	pub locale:                                     String,
	pub message_display_compact:                    bool,
	pub native_phone_integration_enabled:           bool,
	#[deprecated]
	pub passwordless:                               bool,
	pub render_embeds:                              bool,
	pub render_reactions:                           bool,
	pub restricted_guilds:                          Vec<Snowflake>,
	pub show_current_game:                          bool,
	pub status:                                     String,
	pub stream_notifications_enabled:               bool,
	pub theme:                                      String,
	pub timezone_offset:                            i16,
	pub view_nsfw_commands:                         bool,
	pub view_nsfw_guilds:                           bool,
}

#[derive(Deserialize)]
pub struct GuildFolder {
	pub color:     Option<u32>,
	pub guild_ids: Vec<Snowflake>,
	pub id:        Option<u32>,
	pub name:      Option<String>,
}

#[derive(Deserialize)]
pub struct FriendSourceFlags {
	pub all:            bool,
	pub mutual_friends: bool,
	pub mutual_guilds:  bool,
}

#[derive(Deserialize)]
pub struct CustomStatus {
	pub text:       Option<String>,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
	pub expires_at: Option<Timestamp>,
}

#[derive(Deserialize)]
pub struct UserSettingsProto {
	pub versions:               Versions,
	pub inbox:                  Inbox,
	pub guilds:                 AllGuildSettings,
	pub user_content:           UserContentSettings,
	pub voice_and_video:        VoiceAndVideoSettings,
	pub text_and_images:        TextAndImagesSettings,
	pub notifications:          NotificationSettingsProto,
	pub privacy:                PrivacySettings,
	pub debug:                  DebugSettings,
	pub game_library:           GameLibrarySettings,
	pub status:                 StatusSettings,
	pub localization:           LocalizationSettings,
	pub appearance:             AppearanceSettings,
	pub guild_folders:          GuildFolders,
	pub favorites:              Favorites,
	pub audio_context_settings: AudioContextSettings,
	pub communities:            CommunitiesSettings,
	pub broadcast:              BroadcastSettings,
	pub clips:                  Clips,
	pub for_later:              ForLaterSettings,
	pub safety_settings:        SafetySettings,
	pub icymi_settings:         ICYMISettings,
	pub applications:           AllApplicationsSettings,
}

#[derive(Deserialize)]
pub struct AllApplicationsSettings {
	pub app_settings: HashMap<u64, ApplicationsSettings>,
}

#[derive(Deserialize)]
pub struct ApplicationsSettings {
	pub app_dm_settings: ApplicationDmSettings,
}

#[derive(Deserialize)]
pub struct ApplicationDmSettings {
	pub dm_disabled: bool,
}

#[derive(Deserialize)]
pub struct ICYMISettings {
	pub feed_generated_at: u64,
}

#[derive(Deserialize)]
pub struct SafetySettings {
	pub safety_settings_preset:            SafetySettingsPreset,
	pub ignore_profile_speedbump_disabled: bool,
}

#[derive(Deserialize)]
pub enum SafetySettingsPreset {
	UNSET,
	BALANCED,
	STRICT,
	RELAXED,
	CUSTOM,
}

#[derive(Deserialize)]
pub struct ForLaterSettings {
	pub current_tab: ForLaterTab,
}

#[derive(Deserialize)]
pub enum ForLaterTab {
	UNSPECIFIED,
	ALL,
	BOOKMARKS,
	REMINDERS,
}

#[derive(Deserialize)]
pub struct Clips {
	pub allow_voice_recording: Option<bool>,
}

#[derive(Deserialize)]
pub struct BroadcastSettings {
	pub allow_friends:     Option<bool>,
	pub allowed_guild_ids: Vec<u64>,
	pub allowed_user_ids:  Vec<u64>,
	pub auto_broadcast:    Option<bool>,
}

#[derive(Deserialize)]
pub struct CommunitiesSettings {
	#[deprecated]
	pub disable_home_auto_nav: Option<bool>,
}

#[derive(Deserialize)]
pub struct AudioContextSettings {
	pub user:   AudioContextSetting,
	pub stream: AudioContextSetting,
}

#[derive(Deserialize)]
pub struct AudioContextSetting {
	pub muted:            bool,
	pub volume:           f64,
	pub modified_at:      u64,
	pub soundboard_muted: bool,
}

#[derive(Deserialize)]
pub struct Favorites {
	pub favorite_channels: HashMap<u64, FavoriteChannel>,
	pub muted:             bool,
}

#[derive(Deserialize)]
pub struct FavoriteChannel {
	pub nickname:  String,
	pub r#type:    FavoriteChannelType,
	pub position:  u32,
	pub parent_id: u64,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum FavoriteChannelType {
	UNSET,
	REFERENCE_ORIGINAL,
	CATEGORY,
}

#[derive(Deserialize)]
pub struct GuildFolders {
	pub folders:         Vec<GuildFolderProto>,
	#[deprecated]
	pub guild_positions: Vec<u64>,
}

#[derive(Deserialize)]
pub struct GuildFolderProto {
	pub guild_ids: Vec<u64>,
	pub id:        Option<i64>,
	pub name:      Option<String>,
	pub color:     Option<u64>,
}

#[derive(Deserialize)]
pub struct AppearanceSettings {
	pub theme:                             Theme,
	pub developer_mode:                    bool,
	pub client_theme_settings:             ClientThemeSettings,
	pub mobile_redesign_disabled:          bool,
	pub channel_list_layout:               Option<String>,
	pub message_previews:                  Option<String>,
	pub search_result_exact_count_enabled: Option<bool>,
	pub timestamp_hour_cycle:              TimestampCycleHour,
	pub happening_now_cards_disabled:      Option<bool>,
	pub launch_pad_mode:                   LaunchPadMode,
	pub ui_density:                        UiDensity,
	pub swipe_right_to_left_mode:          SwipeRightToLeftMode,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum SwipeRightToLeftMode {
	UNSET,
	CHANNEL_DETAILS,
	REPLY,
}

#[derive(Deserialize)]
pub enum UiDensity {
	UNSET,
	COMPACT,
	COZY,
	RESPONSIVE,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum LaunchPadMode {
	DSIABLED,
	GESTURE_FULL_SCREEN,
	GESTURE_RIGHT_EDGE,
	PULL_TAB,
}

#[derive(Deserialize)]
pub enum TimestampCycleHour {
	AUTO,
	H12,
	H23,
}

#[derive(Deserialize)]
pub struct ClientThemeSettings {
	#[deprecated]
	pub primary_color:                 Option<u32>,
	pub background_gradient_preset_id: Option<u32>,
	#[deprecated]
	pub background_gradient_angle:     Option<f64>,
}

#[derive(Deserialize)]
pub enum Theme {
	UNSET,
	DARK,
	LIGHT,
	DARKER,
	MIDNIGHT,
}

#[derive(Deserialize)]
pub struct LocalizationSettings {
	pub local:           Option<String>,
	pub timezone_offset: Option<i32>,
}

#[derive(Deserialize)]
pub struct StatusSettings {
	pub status:               Option<String>,
	pub custom_status:        CustomStatusProto,
	pub show_current_game:    Option<bool>,
	pub status_expires_at_ms: u64,
}

#[derive(Deserialize)]
pub struct GameLibrarySettings {
	pub install_shortcut_desktop:    Option<bool>,
	pub install_shortcut_start_menu: Option<bool>,
	pub disable_games_tab:           Option<bool>,
}

#[derive(Deserialize)]
pub struct DebugSettings {
	pub rtc_panel_show_voice_states: Option<bool>,
}

#[derive(Deserialize)]
pub struct PrivacySettings {
	pub allow_activity_party_privacy_friends:       Option<bool>,
	pub allow_activity_party_privacy_voice_channel: Option<bool>,
	pub restricted_guild_ids:                       Vec<u64>,
	pub default_guilds_restricted:                  bool,
	pub allow_accessibility_detection:              bool,
	pub detect_platform_accounts:                   Option<bool>,
	#[deprecated]
	pub passwordless:                               Option<bool>,
	pub contact_sync_enabled:                       Option<bool>,
	pub friend_source_flags:                        Option<u32>,
	pub friend_discovery_flags:                     Option<u32>,
	pub activity_restricted_guild_ids:              Vec<u64>,
	pub default_guilds_activity_restricted:         GuildActivityStatusRestriction,
	pub activity_joining_restricted_guild_ids:      Vec<u64>,
	pub message_request_restricted_guild_ids:       Vec<u64>,
	pub default_message_request_restricted:         Option<bool>,
	#[deprecated]
	pub drops_opted_out:                            Option<bool>,
	pub non_spam_retraining_opt_in:                 Option<bool>,
	pub family_center_enabled:                      Option<bool>,
	pub family_center_enabled_v2:                   Option<bool>,
	pub hide_legacy_username:                       Option<bool>,
	pub inappropriate_conversation_warnings:        Option<bool>,
	pub recent_games_enabled:                       Option<bool>,
	pub guilds_leaderboard_opt_out_default:         GuildLeaderboardOptOutDefault,
	pub allow_game_friend_dms_in_discord:           Option<bool>,
	pub default_guilds_restricted_v2:               Option<bool>,
	pub slayer_sdk_receive_dms_in_game:             SlayerSdkRecieveInGameDms,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum SlayerSdkRecieveInGameDms {
	UNSET,
	ALL,
	USERS_WITH_GAME,
	NONE,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum GuildLeaderboardOptOutDefault {
	OFF_FOR_NEW_GUILDS,
	ON_FOR_NEW_GUILDS,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum GuildActivityStatusRestriction {
	OFF,
	ON_FOR_LARGE_GUILDS,
	ON,
}

#[derive(Deserialize)]
pub struct NotificationSettings {
	flags: u8, // https://docs.discord.sex/resources/user-settings#notification-settings-flags
}

#[derive(Deserialize)]
pub struct NotificationSettingsProto {
	pub show_in_app_notifications:           Option<bool>,
	pub notify_friends_on_go_live:           Option<bool>,
	pub notification_center_acked_before_id: u64,
	pub quiet_mode:                          Option<bool>,
	pub focus_mode_expires_at_ms:            u64,
	pub reaction_notifications:              ReactionNotifications,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum ReactionNotifications {
	NOTIFICATIONS_ENABLED,
	ONLY_DMS,
	NOTIFICATIONS_DISABLED,
}

#[derive(Deserialize)]
pub struct TextAndImagesSettings {
	pub diversity_surrogate:                  Option<String>,
	#[deprecated]
	pub use_rich_chat_input:                  Option<bool>,
	pub use_thread_sidebar:                   Option<bool>,
	pub render_spoilers:                      Option<String>, // https://docs.discord.sex/resources/user-settings-proto#spoiler-render-options
	pub emoji_picker_collapsed_sections:      Vec<String>,
	pub sticker_picker_collapsed_sections:    Vec<String>,
	pub view_image_descriptions:              Option<bool>,
	pub show_command_suggestions:             Option<bool>,
	pub inline_attachment_media:              Option<bool>,
	pub inline_embed_media:                   Option<bool>,
	pub gif_auto_play:                        Option<bool>,
	pub render_embeds:                        Option<bool>,
	pub render_reactions:                     Option<bool>,
	pub animate_emoji:                        Option<bool>,
	pub animate_stickers:                     Option<u32>,
	pub enable_tts_command:                   Option<bool>,
	pub message_display_compact:              Option<bool>,
	pub explicit_content_filter:              Option<u32>,
	pub view_nsfw_guilds:                     Option<bool>,
	pub convert_emoticons:                    Option<bool>,
	pub expression_suggestions_enabled:       Option<bool>,
	pub view_nsfw_commands:                   Option<bool>,
	pub use_legacy_chat_input:                Option<bool>,
	pub soundboard_picker_collapsed_sections: Vec<String>,
	#[deprecated]
	pub dm_spam_filter:                       Option<u32>,
	pub dm_spam_filter_v2:                    DmSpamFilterV2,
	pub include_stickers_in_autocomplete:     Option<bool>,
	pub explicit_content_settings:            ExplicitContentSettings,
	pub keyword_filter_settings:              KeywordFilterSettings,
	pub include_soundmoji_in_autocomplete:    Option<bool>,
}

#[derive(Deserialize)]
pub struct KeywordFilterSettings {
	pub profanity:      Option<bool>,
	pub sexual_content: Option<bool>,
	pub slurs:          Option<bool>,
}

#[derive(Deserialize)]
pub struct ExplicitContentSettings {
	pub explicit_content_guilds:        ExplicitContentRedaction,
	pub explicit_content_friend_dm:     ExplicitContentRedaction,
	pub explicit_content_non_friend_dm: ExplicitContentRedaction,
}

#[derive(Deserialize)]
pub enum ExplicitContentRedaction {
	UNSET,
	SHOW,
	BLUR,
	BLOCK,
}

#[derive(Deserialize)]
pub struct VoiceAndVideoSettings {
	pub blur:                             BlurVideoSettings,
	pub preset_option:                    u32,
	pub custom_asset:                     VideoFilterAsset,
	pub always_preview_video:             Option<bool>,
	pub afk_timeout:                      Option<u32>,
	pub stream_notifications_enabled:     Option<bool>,
	pub native_phone_integration_enabled: Option<bool>,
	pub soundboard_settings:              SoundBoardSettings,
	pub disable_stream_previews:          Option<bool>,
	pub soundmoji_volume:                 Option<f64>,
}

#[derive(Deserialize)]
pub struct BlurVideoSettings {
	pub use_blur: bool,
}

#[derive(Deserialize)]
pub struct VideoFilterAsset {
	pub id:        u64,
	pub asset_has: String,
}

#[derive(Deserialize)]
pub struct SoundBoardSettings {
	pub volume: f64,
}

#[derive(Deserialize)]
pub struct UserContentSettings {
	pub dismissed_contents:                           Vec<u8>,
	pub last_dismissed_outbound_promotion_start_date: Option<String>,
	pub premium_tier_0_modal_dismissed_at:            Timestamp,
	pub guild_onboarding_upsell_dismissed_at:         Timestamp,
	pub safety_user_sentiment_notice_dismissed_at:    Timestamp,
	pub last_received_changelog_id:                   u64,
	pub recurring_dismissible_content_states: HashMap<i32, RecurringDismissableContentState>,
}

#[derive(Deserialize)]
pub struct RecurringDismissableContentState {
	pub last_dismissed_version: u32,
	pub last_dismissed_at_ms:   u64,
}

#[derive(Deserialize)]
pub struct Inbox {
	pub current_tab:     InboxTab,
	pub viewed_tutorial: bool,
}

#[derive(Deserialize)]
#[repr(u8)]
pub enum InboxTab {
	Unspecified,
	Mentions,
	Unreads,
	Todos,
	ForYou,
	GameInvites,
	Bookmarks,
	Scheduled,
}
