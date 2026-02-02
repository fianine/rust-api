use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::post::{
    CommentResponse, CreateCommentRequest, CreatePostRequest, PostResponse,
};
use crate::models::{Comment, Like, Post};
use crate::state::AppState;

const USER_ID_HEADER: &str = "x-user-id";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post))
        .route("/user/:user_id", get(list_user_posts))
        .route("/:id/like", post(like_post).delete(unlike_post))
        .route("/:id/comments", post(add_comment).get(list_comments))
}

async fn create_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let mut posts = state.posts.write().await;
    let id = Uuid::new_v4();
    let now = Utc::now();

    let post = Post {
        id,
        author_id: user_id,
        image_url: payload.image_url,
        caption: payload.caption,
        created_at: now,
    };

    posts.insert(id, post.clone());

    Ok(Json(PostResponse {
        id: post.id,
        author_id: post.author_id,
        image_url: post.image_url,
        caption: post.caption,
        created_at: post.created_at,
    }))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, ApiError> {
    let posts = state.posts.read().await;
    let post = posts.get(&id).ok_or(ApiError::NotFound)?;

    Ok(Json(PostResponse {
        id: post.id,
        author_id: post.author_id,
        image_url: post.image_url.clone(),
        caption: post.caption.clone(),
        created_at: post.created_at,
    }))
}

async fn list_user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<PostResponse>>, ApiError> {
    let posts = state.posts.read().await;
    let mut result: Vec<PostResponse> = posts
        .values()
        .filter(|p| p.author_id == user_id)
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

async fn like_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(post_id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let posts = state.posts.read().await;
    if !posts.contains_key(&post_id) {
        return Err(ApiError::NotFound);
    }
    drop(posts);

    let mut likes = state.likes.write().await;
    if !likes
        .iter()
        .any(|l| l.user_id == user_id && l.post_id == post_id)
    {
        likes.push(Like { user_id, post_id });
    }

    Ok(Json(()))
}

async fn unlike_post(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(post_id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let mut likes = state.likes.write().await;
    likes.retain(|l| !(l.user_id == user_id && l.post_id == post_id));

    Ok(Json(()))
}

async fn add_comment(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let user_id = extract_user_id(&headers)?;

    let posts = state.posts.read().await;
    if !posts.contains_key(&post_id) {
        return Err(ApiError::NotFound);
    }
    drop(posts);

    let mut comments = state.comments.write().await;
    let id = Uuid::new_v4();
    let now = Utc::now();

    let comment = Comment {
        id,
        post_id,
        author_id: user_id,
        text: payload.text,
        created_at: now,
    };

    comments.insert(id, comment.clone());

    Ok(Json(CommentResponse {
        id: comment.id,
        post_id: comment.post_id,
        author_id: comment.author_id,
        text: comment.text,
        created_at: comment.created_at,
    }))
}

async fn list_comments(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, ApiError> {
    let comments = state.comments.read().await;
    let mut result: Vec<CommentResponse> = comments
        .values()
        .filter(|c| c.post_id == post_id)
        .map(|comment| CommentResponse {
            id: comment.id,
            post_id: comment.post_id,
            author_id: comment.author_id,
            text: comment.text.clone(),
            created_at: comment.created_at,
        })
        .collect();

    result.sort_by_key(|c| c.created_at);

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
