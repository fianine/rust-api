use crate::models::{Comment, Follow, Like, Post, User};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
    pub posts: Arc<RwLock<HashMap<Uuid, Post>>>,
    pub comments: Arc<RwLock<HashMap<Uuid, Comment>>>,
    pub likes: Arc<RwLock<Vec<Like>>>,
    pub follows: Arc<RwLock<Vec<Follow>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
