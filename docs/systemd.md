# Running luxa as a systemd service

Install the webhook server and a cloudflared tunnel as **user** services
(`systemctl --user`) so they start in the background and survive logout. They run
as your user — device access comes from the [udev rule](getting-started.md), no
root required.

## Install

```bash
mise run install-service     # or: sh scripts/install-systemd.sh
```

This will:

1. `cargo build --release` and copy the binary to `~/.local/bin/luxa`.
2. Copy `luxa.example.yaml` to `~/.config/luxa/luxa.yaml` (only if it doesn't
   exist yet — edit it to define your webhooks).
3. Install two user units to `~/.config/systemd/user/`:
   - `luxa.service` — runs `luxa serve` against your config.
   - `luxa-tunnel.service` — runs a `cloudflared` quick tunnel to port 8765
     (skipped if `cloudflared` isn't found).
4. `enable --now` both, so they start immediately and on login.

## Manage

```bash
systemctl --user status luxa.service
journalctl --user -u luxa.service -f

# The quick-tunnel public URL (changes on each restart):
journalctl --user -u luxa-tunnel.service | grep trycloudflare

systemctl --user restart luxa.service
```

Start at boot without an active login session:

```bash
sudo loginctl enable-linger "$USER"
```

## Notes

- While the service runs it owns the USB device; direct `luxa flash`/`blink`/etc.
  commands automatically forward to it (you will see `(via server)`). See
  [webhooks](webhooks.md#direct-commands-while-the-server-is-running).

- After editing `~/.config/luxa/luxa.yaml`, run
  `systemctl --user restart luxa.service`.
- The units listen on port `8765`; if you change it in the config, update the
  `ExecStart` lines in both units (or reinstall) to match.
- A quick tunnel is unauthenticated and its URL rotates. For a stable, protected
  endpoint use a named tunnel + Cloudflare Access — see [cloudflared](cloudflared.md)
  — and point `luxa-tunnel.service` at `cloudflared tunnel run <name>` instead.

## Uninstall

```bash
mise run uninstall-service   # removes the services; keeps binary + config
```
