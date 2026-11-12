# luxa

![](luxafor.jpg)

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

## Troubleshooting

**`failed to open device (errno 13)`** — the udev rule is missing or not yet
applied, so the USB node is not writable by your user. Install it and re-apply
it to the already-plugged device:

```bash
sudo cp 99-luxafor.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger --action=add --attr-match=idVendor=04d8
```

If the error comes back as `server (HTTP 503)`, the CLI forwarded the command
to a running server; fix the rule, then `systemctl --user restart luxa` if it
still fails.

**`could not read config luxa.yaml`** — `luxa serve` looks for `luxa.yaml` in
the current directory. Pass the path explicitly
(`luxa serve -c ~/.config/luxa/luxa.yaml`), or skip it entirely if the
[systemd service](docs/systemd.md) is already running: it owns the device and
port `8765`, and CLI commands forward to it.

## Docs

- [Getting started](docs/getting-started.md) — install, permissions, usage
- [Webhooks](docs/webhooks.md) — YAML config → behaviors over HTTP
- [Expose with cloudflared](docs/cloudflared.md) — tunnel the webhook port to the web
- [Run as a service](docs/systemd.md) — systemd user units for the server + tunnel
- [Debugging notes](docs/debugging.md) — how we got the device talking
- [Architecture decisions](docs/adr/) — ADRs
