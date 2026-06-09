//! Axum app exposing the configured webhooks.
//!
//! - `GET|POST /webhook/<name>` runs a behavior defined in the config.
//! - `POST /run` runs an arbitrary behavior spec (used by the CLI to forward
//!   direct commands when the server owns the device).
//!
//! Device work is blocking (USB writes, software blinks), so it runs on
//! `spawn_blocking`; the controller mutex serializes the device.

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde_json::json;

use crate::config::{ServerConfig, WebhookSpec};
use crate::controller::Controller;
use crate::{behaviors, color};

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
        .route("/run", post(run_command))
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
    run_spec(&state, spec, json!({ "triggered": name })).await
}

/// Run an arbitrary spec (CLI forwards direct commands here).
async fn run_command(State(state): State<AppState>, Json(spec): Json<WebhookSpec>) -> Response {
    if !behaviors::is_known(&spec.behavior) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("unknown behavior {:?}", spec.behavior) })),
        )
            .into_response();
    }
    if let Err(e) = color::parse_color(&spec.color) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response();
    }
    run_spec(&state, spec, json!({ "ran": true })).await
}

/// Run `spec` on the shared device off-thread; merge `extra` into the 200 body.
async fn run_spec(state: &AppState, spec: WebhookSpec, extra: serde_json::Value) -> Response {
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
        Ok(()) => {
            let mut body = json!({ "behavior": spec.behavior, "color": spec.color });
            if let (Some(obj), Some(extra)) = (body.as_object_mut(), extra.as_object()) {
                obj.extend(extra.clone());
            }
            Json(body).into_response()
        }
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
