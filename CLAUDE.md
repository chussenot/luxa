# CLAUDE.md

Guidance for Claude Code when working in this repo.

## What this is

`luxa` — a Rust CLI **and** webhook server that controls a **Luxafor Flag**
(USB HID `04d8:f372`): flash, blink, colors, off, plus an axum server mapping
YAML-configured webhooks to behaviors. The device is driven directly over USB
with the pure-Rust `nusb` crate (no libusb/hidapi). Toolchain via **mise**
(Rust 1.96) + **Cargo**.

## Layout

| Path                  | Purpose                                              |
|-----------------------|------------------------------------------------------|
| `src/device.rs`       | Low-level USB driver (`Luxafor`), 8-byte HID packets |
| `src/color.rs`        | Named colours + `parse_color` (name or hex)          |
| `src/behaviors.rs`    | Behavior registry (solid/blink/flash/…)              |
| `src/config.rs`       | YAML config (serde) + validation                     |
| `src/controller.rs`   | Shared device guarded by a mutex, reconnect-once      |
| `src/server.rs`       | axum app — `/webhook/<name>` routes                  |
| `src/main.rs`         | clap CLI — one subcommand per command                |
| `luxa.example.yaml`   | Sample webhook config                                |
| `mise.toml`           | Toolchain + cargo tasks (serve/tunnel/lint/fmt/…)    |
| `docs/`               | `getting-started`, `webhooks`, `debugging`, `adr/`   |
| `99-luxafor.rules`    | udev rule granting device access on Linux            |

## Commands

```bash
mise install                         # rust toolchain + cargo build
cargo run -- --help                  # all commands
cargo run -- flash blue --repeat 5   # hardware strobe
cargo run -- serve -c luxa.yaml      # webhook server
mise run lint | fmt | test           # clippy / rustfmt / cargo test
mise run flash | blink | off | serve | tunnel
```

## Conventions

- **New effects:** add the 8-byte command method to `device.rs` first
  (`self.write([mode, ...])`), then expose it as a clap subcommand in `main.rs`.
- **New webhook behavior:** add an arm to `behaviors::run` and an entry to
  `behaviors::BEHAVIORS` (single source of truth — `config.rs` validates against
  it and the CLI lists it).
- Colours are names (`red`, …) or hex (`#ff8800`); extend `color::COLORS`.
- Device work is blocking; in the server it runs on `spawn_blocking` behind the
  controller mutex.
- Keep `cargo clippy -- -D warnings` and `cargo fmt` clean. Update `docs/` when
  behavior or setup changes.

## Hardware gotchas (see `docs/debugging.md`)

- `nusb` opens the USB node `/dev/bus/usb/<bus>/<dev>` via usbfs; it detaches the
  kernel HID driver and claims interface 0. Permissions live on that node.
- A udev rule needs a **replug** (or `udevadm trigger --action=add`) to apply to
  an already-connected device.
- Enumeration works but `open()`/claim fails ⇒ write-permission issue on the USB node.
- The Flag has one 8-byte interrupt OUT endpoint (`0x01`); commands are 8 bytes.

## Specialist agents

- **luxafor-expert** (`.claude/agents/luxafor-expert.md`) — device-protocol,
  new-command, or "device won't respond" tasks.
- **adr-writer** (`.claude/agents/adr-writer.md`) — author ADRs in `docs/adr/`.

## Verifying

When hardware is connected, verify changes against the real device
(`cargo run -- flash`, etc.) and report exactly what it did — not just that it builds.
