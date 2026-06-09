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

use crate::color::{parse_color, COLORS};
use crate::config::load_config;
use crate::controller::Controller;
use crate::device::Luxafor;

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
            let rgb = parse_color(&color).map_err(|e| e.to_string())?;
            let mut lux = Luxafor::open().map_err(|e| e.to_string())?;
            lux.strobe(rgb, speed, repeat).map_err(|e| e.to_string())?;
            println!("⚡ flashing {color} (x{repeat})");
            Ok(())
        }
        Command::Blink {
            color,
            times,
            on,
            off,
        } => {
            let rgb = parse_color(&color).map_err(|e| e.to_string())?;
            let mut lux = Luxafor::open().map_err(|e| e.to_string())?;
            lux.blink(
                rgb,
                times,
                Duration::from_secs_f64(on),
                Duration::from_secs_f64(off),
            )
            .map_err(|e| e.to_string())?;
            lux.off().map_err(|e| e.to_string())?;
            println!("💡 blinked {color} (x{times})");
            Ok(())
        }
        Command::Color { target } => {
            let rgb = parse_color(&target).map_err(|e| e.to_string())?;
            let mut lux = Luxafor::open().map_err(|e| e.to_string())?;
            lux.set_color(rgb).map_err(|e| e.to_string())?;
            println!("🎨 set to {target}");
            Ok(())
        }
        Command::Off => {
            let mut lux = Luxafor::open().map_err(|e| e.to_string())?;
            lux.off().map_err(|e| e.to_string())?;
            println!("⏻ off");
            Ok(())
        }
        Command::Serve { config, host, port } => serve(config, host, port),
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
