use crate::{models::CreateChat, AppError, AppState, UpdateChat};
use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use chat_core::User;

#[utoipa::path(
    get,
    path = "/api/chats",
    responses(
        (status = 200, description = "List of chats", body = Vec<Chat>),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chats = state.fetch_chats(user.ws_id as u64).await?;

    Ok((StatusCode::CREATED, Json(chats)))
}

#[utoipa::path(
    post,
    path = "/api/chats",
    responses(
        (status = 201, description = "Chat created", body = Chat),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.create_chat(input, user.ws_id as _).await?;

    Ok((StatusCode::CREATED, Json(chat)))
}

#[utoipa::path(
    get,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Chat found", body = Chat),
        (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn get_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.get_chat_by_id(id).await?;

    match chat {
        Some(chat) => Ok(Json(chat)),
        None => Err(AppError::NotFound(format!("Chat with id {} not found", id))),
    }
}

#[utoipa::path(
    patch,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "update chat", body = Chat),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn update_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.update_chat_by_id(id, input).await?;

    Ok(Json(chat))
}

#[utoipa::path(
    delete,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "delete chat", body = Chat),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn delete_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let ret = state.delete_chat_by_id(id).await?;

    match ret {
        true => Ok((StatusCode::OK, Json("Chat deleted successfully"))),
        false => Err(AppError::AnyError(anyhow!(
            "Failed to delete chat with id {}",
            id
        ))),
    }
}
