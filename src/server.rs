//! Axum app exposing the configured webhooks.
//!
//! Each entry in `config.webhooks` becomes a `/webhook/<name>` route accepting
//! GET or POST. Device work is blocking (USB writes, software blinks), so it
//! runs on `spawn_blocking`; the controller mutex serializes the device.

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get},
    Json, Router,
};
use serde_json::json;

use crate::config::ServerConfig;
use crate::controller::Controller;

#[derive(Clone)]
struct AppState {
    config: Arc<ServerConfig>,
    controller: Arc<Mutex<Controller>>,
}

pub fn app(config: Arc<ServerConfig>, controller: Arc<Mutex<Controller>>) -> Router {
    let state = AppState { config, controller };
    Router::new()
        .route("/health", get(health))
        .route("/webhook/{name}", any(trigger))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let names: Vec<&String> = state.config.webhooks.keys().collect();
    Json(json!({ "status": "ok", "webhooks": names }))
}

async fn trigger(State(state): State<AppState>, Path(name): Path<String>) -> Response {
    let Some(spec) = state.config.webhooks.get(&name).cloned() else {
        let known: Vec<&String> = state.config.webhooks.keys().collect();
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("unknown webhook {name:?}"), "known": known })),
        )
            .into_response();
    };

    let controller = state.controller.clone();
    let run_spec = spec.clone();
    let result = tokio::task::spawn_blocking(move || {
        controller
            .lock()
            .expect("controller mutex poisoned")
            .run(&run_spec)
    })
    .await
    .expect("device task panicked");

    match result {
        Ok(()) => Json(json!({
            "triggered": name,
            "behavior": spec.behavior,
            "color": spec.color,
        }))
        .into_response(),
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
