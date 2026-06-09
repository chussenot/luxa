# luxa

A small Rust CLI and webhook server to flash/blink a **Luxafor Flag**
(USB `04d8:f372`). Talks to the device directly over USB with the pure-Rust
[`nusb`](https://docs.rs/nusb) crate — no libusb/hidapi system libraries.
Tooling via [mise](https://mise.jdx.dev/) (Rust toolchain) + Cargo.

```bash
mise install                       # rust toolchain + build
cargo run -- flash blue            # ⚡ strobe
cargo run -- blink green --times 8 # 💡 on/off loop
cargo run -- off
```

Linux needs a one-time udev rule for device access — see [docs/getting-started.md](docs/getting-started.md).

## Webhooks

Map HTTP webhooks to device behaviors via YAML, then run a server:

```bash
cp luxa.example.yaml luxa.yaml     # edit to taste
cargo run -- serve                 # POST /webhook/<name>
curl -X POST localhost:8765/webhook/deploy-success
```

See [docs/webhooks.md](docs/webhooks.md).

## Docs

- [Getting started](docs/getting-started.md) — install, permissions, usage
- [Webhooks](docs/webhooks.md) — YAML config → behaviors over HTTP
- [Expose with cloudflared](docs/cloudflared.md) — tunnel the webhook port to the web
- [Run as a service](docs/systemd.md) — systemd user units for the server + tunnel
- [Debugging notes](docs/debugging.md) — how we got the device talking
- [Architecture decisions](docs/adr/) — ADRs
