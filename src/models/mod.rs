pub mod user;
pub mod post;

pub use user::User;
pub use post::{Post, Comment};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Like {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
}
