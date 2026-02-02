use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub image_url: String,
    pub caption: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub image_url: String,
    pub caption: String,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub image_url: String,
    pub caption: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}
