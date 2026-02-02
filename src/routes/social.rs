use axum::{
    extract::State,
    routing::{get, post, delete},
    Json, Router,
};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::{Follow, Post, PostResponse};
use crate::state::AppState;

const USER_ID_HEADER: &str = "x-user-id";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/follow/:user_id", post(follow_user).delete(unfollow_user))
        .route("/feed", get(feed))
}

async fn follow_user(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(target_id): axum::extract::Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    if user_id == target_id {
        return Err(ApiError::BadRequest("cannot follow yourself".into()));
    }

    let users = state.users.read().await;
    if !users.contains_key(&target_id) {
        return Err(ApiError::NotFound);
    }
    drop(users);

    let mut follows = state.follows.write().await;
    if !follows
        .iter()
        .any(|f| f.follower_id == user_id && f.following_id == target_id)
    {
        follows.push(Follow {
            follower_id: user_id,
            following_id: target_id,
        });
    }

    Ok(Json(()))
}

async fn unfollow_user(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(target_id): axum::extract::Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let mut follows = state.follows.write().await;
    follows.retain(|f| !(f.follower_id == user_id && f.following_id == target_id));

    Ok(Json(()))
}

async fn feed(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let follows = state.follows.read().await;
    let following_ids: Vec<Uuid> = follows
        .iter()
        .filter(|f| f.follower_id == user_id)
        .map(|f| f.following_id)
        .collect();
    drop(follows);

    let posts = state.posts.read().await;
    let mut result: Vec<PostResponse> = posts
        .values()
        .filter(|p| following_ids.contains(&p.author_id))
        .map(|post| PostResponse {
            id: post.id,
            author_id: post.author_id,
            image_url: post.image_url.clone(),
            caption: post.caption.clone(),
            created_at: post.created_at,
        })
        .collect();

    result.sort_by_key(|p| p.created_at);
    result.reverse();

    Ok(Json(result))
}

fn extract_user_id(headers: &axum::http::HeaderMap) -> Result<uuid::Uuid, ApiError> {
    let value = headers
        .get(USER_ID_HEADER)
        .ok_or_else(|| ApiError::Unauthorized)?
        .to_str()
        .map_err(|_| ApiError::Unauthorized)?;

    Uuid::parse_str(value).map_err(|_| ApiError::Unauthorized)
}
