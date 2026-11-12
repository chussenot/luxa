# Getting started

Build `luxa`, give your user access to the device, and light it once.

## 1. Install the toolchain and build

The project is pinned with `mise` (Rust 1.96) and built with Cargo.

```bash
mise install        # installs the Rust toolchain per mise.toml
mise run install    # cargo build
# or an optimized binary:
mise run build      # cargo build --release  -> target/release/luxa
```

## 2. Grant device access (Linux, one-time)

The Luxafor is a USB HID device that only `root` can write to by default.
Install the udev rule and apply it to the connected device:

```bash
sudo cp 99-luxafor.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger --action=add --attr-match=idVendor=04d8
```

Unplugging and replugging the device has the same effect as the last command.
Without the rule you can still run any command with `sudo`. If access is still
denied, see [Troubleshooting](troubleshooting.md).

## 3. Light it

```bash
cargo run -- flash blue      # hardware strobe
cargo run -- color "#ff8800" # solid colour
cargo run -- off
```

After `mise run build` you can call `target/release/luxa <cmd>` directly.

## Next

- Every command and flag: [CLI reference](cli.md).
- React to CI, deploys or alerts: [Webhooks](webhooks.md).
- Keep it running in the background: [Run as a service](systemd.md).
