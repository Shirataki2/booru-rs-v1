use crate::models::common::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    pub is_deleted: bool,
    pub is_banned: bool,
    pub group_name: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub other_names: Vec<String>,
}
