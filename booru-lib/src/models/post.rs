use serde_with::{serde_as, DefaultOnNull};
use crate::models::common::*;
use crate::models::serde::SpaceDelimited;
use crate::error::BooruError;
use std::path::Path;

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub created_at: Timestamp,
    pub uploader_id: UserId,
    pub score: i64,
    pub source: String,
    pub md5: String,
    pub last_comment_bumped_at: Option<Timestamp>,
    #[serde_as(deserialize_as = "DefaultOnNull")] 
    pub rating: Rating,
    pub image_width: u64,
    pub image_height: u64,
    #[serde(rename = "tag_string")]
    pub tags: SpaceDelimited,
    pub is_note_locked: bool,
    pub fav_count: u64,
    pub file_ext: String,
    pub last_noted_at: Option<Timestamp>,
    pub is_rating_locked: bool,
    pub parent_id: Option<PostId>,
    pub has_children: bool,
    pub approver_id: Option<UserId>,
    pub tag_count_general: u64,
    pub tag_count_artist: u64,
    pub tag_count_character: u64,
    pub tag_count_copyright: u64,
    pub file_size: u64,
    pub is_status_locked: bool,
    pub pool_string: String,
    pub up_score: i64,
    pub down_score: i64,
    pub is_pending: bool,
    pub is_flagged: bool,
    pub is_deleted: bool,
    pub tag_count: u64,
    pub updated_at: Timestamp,
    pub is_banned: bool,
    pub pixiv_id: Option<u64>,
    pub last_commented_at: Option<Timestamp>,
    pub has_active_children: bool,
    pub bit_flags: u64,
    pub tag_count_meta: u64,
    #[serde_as(deserialize_as = "DefaultOnNull")] 
    pub has_large: bool,
    pub has_visible_children: bool,
    #[serde(rename = "tag_string_general")]
    pub tags_general: SpaceDelimited,
    #[serde(rename = "tag_string_character")]
    pub tags_character: SpaceDelimited,
    #[serde(rename = "tag_string_copyright")]
    pub tags_copyright: SpaceDelimited,
    #[serde(rename = "tag_string_artist")]
    pub tags_artist: SpaceDelimited,
    #[serde(rename = "tag_string_meta")]
    pub tags_meta: SpaceDelimited,
    pub file_url: String,
    pub large_file_url: String,
    pub preview_file_url: String,
}

impl Post {
    pub fn save(&self, path: &Path) -> Result<(), BooruError> {
        if !path.is_dir() {
            return Err(BooruError::PathIsNotDirectory(path.to_str().unwrap().to_string()))
        }
        let filename = format!("{}.{}", self.id, self.file_ext);
        let path = path.join(&filename);
        let bytes = reqwest::blocking::get(&self.large_file_url)?.bytes()?;
        let img = image::load_from_memory(&bytes)?;
        img.save(path)?;
        Ok(())
    }
}