mod chat;
mod file;
mod messages;
mod user;
mod workspace;

use serde::{Deserialize, Serialize};

pub use chat::CreateChat;
pub use messages::{CreateMessage, ListMessages};
pub use user::{CreateUser, SigninUser};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String,
    pub hash: String,
}
