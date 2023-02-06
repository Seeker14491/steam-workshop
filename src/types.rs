use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct OuterResponse {
    #[serde(rename = "response")]
    pub response: InnerResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResponse {
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "publishedfiledetails")]
    pub published_file_details: Option<Vec<PublishedFileDetails>>,

    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedFileDetails {
    #[serde(rename = "result")]
    pub result: u32,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "publishedfileid")]
    pub published_file_id: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "creator")]
    pub creator: u64,

    #[serde(rename = "creator_appid")]
    pub creator_app_id: u64,

    #[serde(rename = "consumer_appid")]
    pub consumer_app_id: u64,

    #[serde(rename = "consumer_shortcutid")]
    pub consumer_shortcut_id: u64,

    #[serde(rename = "filename")]
    pub filename: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "file_size")]
    pub file_size: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "preview_file_size")]
    pub preview_file_size: i64,

    #[serde(rename = "file_url")]
    pub file_url: String,

    #[serde(rename = "preview_url")]
    pub preview_url: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "hcontent_file")]
    pub hcontent_file: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "hcontent_preview")]
    pub hcontent_preview: u64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "file_description")]
    pub file_description: String,

    #[serde(rename = "time_created")]
    pub time_created: u64,

    #[serde(rename = "time_updated")]
    pub time_updated: u64,

    #[serde(rename = "visibility")]
    pub visibility: u32,

    #[serde(rename = "flags")]
    pub flags: u64,

    #[serde(rename = "workshop_file")]
    pub workshop_file: bool,

    #[serde(rename = "workshop_accepted")]
    pub workshop_accepted: bool,

    #[serde(rename = "show_subscribe_all")]
    pub show_subscribe_all: bool,

    #[serde(rename = "num_comments_public")]
    pub num_comments_public: i64,

    #[serde(rename = "banned")]
    pub banned: bool,

    #[serde(rename = "ban_reason")]
    pub ban_reason: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "banner")]
    pub banner: u64,

    #[serde(rename = "can_be_deleted")]
    pub can_be_deleted: bool,

    #[serde(rename = "app_name")]
    pub app_name: String,

    #[serde(rename = "file_type")]
    pub file_type: u32,

    #[serde(rename = "can_subscribe")]
    pub can_subscribe: bool,

    #[serde(rename = "subscriptions")]
    pub subscriptions: i64,

    #[serde(rename = "favorited")]
    pub favorited: i64,

    #[serde(rename = "followers")]
    pub followers: i64,

    #[serde(rename = "lifetime_subscriptions")]
    pub lifetime_subscriptions: i64,

    #[serde(rename = "lifetime_favorited")]
    pub lifetime_favorited: i64,

    #[serde(rename = "lifetime_followers")]
    pub lifetime_followers: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "lifetime_playtime")]
    pub lifetime_playtime: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "lifetime_playtime_sessions")]
    pub lifetime_playtime_sessions: i64,

    #[serde(rename = "views")]
    pub views: i64,

    #[serde(rename = "num_children")]
    pub num_children: i64,

    #[serde(rename = "num_reports")]
    pub num_reports: i64,

    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,

    #[serde(rename = "vote_data")]
    pub vote_data: VoteData,

    #[serde(rename = "language")]
    pub language: u32,

    #[serde(rename = "maybe_inappropriate_sex")]
    pub maybe_inappropriate_sex: bool,

    #[serde(rename = "maybe_inappropriate_violence")]
    pub maybe_inappropriate_violence: bool,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "revision_change_number")]
    pub revision_change_number: i64,

    #[serde(rename = "revision")]
    pub revision: i64,

    #[serde(rename = "ban_text_check_result")]
    pub ban_text_check_result: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "tag")]
    pub tag: String,

    #[serde(rename = "display_name")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteData {
    #[serde(rename = "score")]
    pub score: f32,

    #[serde(rename = "votes_up")]
    pub votes_up: i64,

    #[serde(rename = "votes_down")]
    pub votes_down: i64,
}
