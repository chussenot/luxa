//! luxa — a CLI and webhook server to control a Luxafor Flag.

mod behaviors;
mod color;
mod config;
mod controller;
mod device;
mod server;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::color::COLORS;
use crate::config::{load_config, WebhookSpec};
use crate::controller::Controller;

/// Default address of a running webhook server to forward direct commands to.
const DEFAULT_SERVER: &str = "http://127.0.0.1:8765";

#[derive(Parser)]
#[command(
    name = "luxa",
    version,
    about = "Control a Luxafor Flag: flash, blink, set colours, run a webhook server."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Hardware strobe/flash the device.
    Flash {
        /// Colour name (e.g. red) or hex like '#ff8800'.
        #[arg(default_value = "red")]
        color: String,
        /// Strobe speed, 0-255.
        #[arg(long, default_value_t = 10)]
        speed: u8,
        /// Number of flashes, 0-255.
        #[arg(long, default_value_t = 5)]
        repeat: u8,
    },
    /// Software blink: toggle the colour on and off.
    Blink {
        #[arg(default_value = "red")]
        color: String,
        /// How many on/off cycles.
        #[arg(long, default_value_t = 5)]
        times: u32,
        /// Seconds the light stays on.
        #[arg(long, default_value_t = 0.3)]
        on: f64,
        /// Seconds the light stays off.
        #[arg(long, default_value_t = 0.3)]
        off: f64,
    },
    /// Set a solid colour.
    Color {
        /// Colour name or hex.
        target: String,
    },
    /// Turn the device off.
    Off,
    /// List the built-in colour names.
    Colors,
    /// List the behaviors available to webhooks.
    Behaviors,
    /// Run the webhook server defined by a YAML config.
    Serve {
        /// Path to the YAML config.
        #[arg(short, long, default_value = "luxa.yaml")]
        config: PathBuf,
        /// Override the host from the config.
        #[arg(long)]
        host: Option<String>,
        /// Override the port from the config.
        #[arg(long)]
        port: Option<u16>,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Colors => {
            for (name, _) in COLORS {
                println!("{name}");
            }
            Ok(())
        }
        Command::Behaviors => {
            for (name, desc) in behaviors::BEHAVIORS {
                println!("{name:<12} {desc}");
            }
            Ok(())
        }
        Command::Flash {
            color,
            speed,
            repeat,
        } => {
            let spec = WebhookSpec {
                behavior: "flash".into(),
                color: color.clone(),
                speed: Some(speed),
                repeat: Some(repeat),
                ..Default::default()
            };
            let on = dispatch(&spec)?;
            println!("⚡ flashing {color} (x{repeat}){}", via(on));
            Ok(())
        }
        Command::Blink {
            color,
            times,
            on,
            off,
        } => {
            let spec = WebhookSpec {
                behavior: "blink".into(),
                color: color.clone(),
                times: Some(times),
                on: Some(on),
                off: Some(off),
                ..Default::default()
            };
            let ran = dispatch(&spec)?;
            println!("💡 blinked {color} (x{times}){}", via(ran));
            Ok(())
        }
        Command::Color { target } => {
            let spec = WebhookSpec {
                behavior: "solid".into(),
                color: target.clone(),
                ..Default::default()
            };
            let on = dispatch(&spec)?;
            println!("🎨 set to {target}{}", via(on));
            Ok(())
        }
        Command::Off => {
            let spec = WebhookSpec {
                behavior: "off".into(),
                color: "off".into(),
                ..Default::default()
            };
            let on = dispatch(&spec)?;
            println!("⏻ off{}", via(on));
            Ok(())
        }
        Command::Serve { config, host, port } => serve(config, host, port),
    }
}

/// Where a direct command ran.
enum RanOn {
    Device,
    Server,
}

fn via(ran: RanOn) -> &'static str {
    match ran {
        RanOn::Server => " (via server)",
        RanOn::Device => "",
    }
}

/// Base URL of a running server to forward to (`LUXA_SERVER` env, else default).
fn server_base() -> String {
    match std::env::var("LUXA_SERVER") {
        Ok(v) if !v.is_empty() => {
            if v.starts_with("http://") || v.starts_with("https://") {
                v
            } else {
                format!("http://{v}")
            }
        }
        _ => DEFAULT_SERVER.to_string(),
    }
}

/// Run a behavior: forward to a running server if one is reachable (it owns the
/// USB device while serving), otherwise drive the device directly.
fn dispatch(spec: &WebhookSpec) -> Result<RanOn, String> {
    let url = format!("{}/run", server_base());
    let config = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_secs(120)))
        .build();
    let agent = ureq::Agent::new_with_config(config);

    match agent.post(&url).send_json(spec) {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                Ok(RanOn::Server)
            } else {
                let body = resp.into_body().read_to_string().unwrap_or_default();
                let detail = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(String::from))
                    .unwrap_or_else(|| body.trim().to_string());
                Err(format!("server (HTTP {}): {detail}", status.as_u16()))
            }
        }
        // No server reachable — drive the device directly.
        Err(_) => {
            Controller::new().run(spec).map_err(|e| e.to_string())?;
            Ok(RanOn::Device)
        }
    }
}

fn serve(config_path: PathBuf, host: Option<String>, port: Option<u16>) -> Result<(), String> {
    let mut cfg = load_config(&config_path)?;
    if let Some(h) = host {
        cfg.host = h;
    }
    if let Some(p) = port {
        cfg.port = p;
    }
    let addr = format!("{}:{}", cfg.host, cfg.port);
    let webhooks: Vec<(String, String, String)> = cfg
        .webhooks
        .iter()
        .map(|(n, s)| (n.clone(), s.behavior.clone(), s.color.clone()))
        .collect();

    let app = server::app(Arc::new(cfg), Arc::new(Mutex::new(Controller::new())));

    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async move {
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| format!("could not bind {addr}: {e}"))?;
        println!("🚦 serving {} webhook(s) on http://{addr}", webhooks.len());
        for (name, behavior, col) in &webhooks {
            println!("   POST /webhook/{name} → {behavior} {col}");
        }
        axum::serve(listener, app).await.map_err(|e| e.to_string())
    })
}
