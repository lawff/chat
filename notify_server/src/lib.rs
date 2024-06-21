mod config;
mod error;
mod notif;
mod see;

use std::{ops::Deref, sync::Arc};

use axum::{
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chat_core::{
    middlewares::{verify_token, TokenVerify},
    DecodingKey,
};
pub use config::AppConfig;
use dashmap::DashMap;
use error::AppError;
use see::sse_handler;
use tokio::sync::broadcast;

pub use notif::*;

pub type UserMap = Arc<DashMap<u64, broadcast::Sender<Arc<AppEvent>>>>;

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

pub struct AppStateInner {
    pub config: AppConfig,
    users: UserMap,
    dk: DecodingKey,
}

const INDEX_HTML: &str = include_str!("../index.html");

pub async fn get_router(config: AppConfig) -> anyhow::Result<Router> {
    let state = AppState::new(config);
    notif::setup_pg_listener(state.clone()).await?;
    let app = Router::new()
        .route("/events", get(sse_handler))
        .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        .route("/", get(index_handler))
        .with_state(state.clone());

    Ok(app)
}

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let dk = DecodingKey::load(&config.auth.pk).expect("Failed to load public key");
        let users = Arc::new(DashMap::new());
        Self(Arc::new(AppStateInner { config, dk, users }))
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<chat_core::User, Self::Error> {
        Ok(self.dk.verify(token)?)
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
