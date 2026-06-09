#!/bin/sh
# Install luxa as systemd *user* services: the webhook server and a cloudflared
# quick tunnel. Runs as your user (no root) — device access comes from the udev
# rule. Re-runnable (idempotent); won't overwrite an existing config.
set -eu

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BIN_SRC="$REPO_DIR/target/release/luxa"
BIN_DST="$HOME/.local/bin/luxa"
CONF_DIR="$HOME/.config/luxa"
CONF_DST="$CONF_DIR/luxa.yaml"
UNIT_DIR="$HOME/.config/systemd/user"

echo "==> Building release binary"
if command -v cargo >/dev/null 2>&1; then
  (cd "$REPO_DIR" && cargo build --release)
fi
[ -f "$BIN_SRC" ] || {
  echo "error: $BIN_SRC not found — run 'cargo build --release' first" >&2
  exit 1
}

echo "==> Installing binary -> $BIN_DST"
mkdir -p "$HOME/.local/bin"
install -m 0755 "$BIN_SRC" "$BIN_DST"

echo "==> Installing config -> $CONF_DST"
mkdir -p "$CONF_DIR"
if [ -f "$CONF_DST" ]; then
  echo "    config exists, leaving it untouched"
else
  cp "$REPO_DIR/luxa.example.yaml" "$CONF_DST"
  echo "    copied from luxa.example.yaml — edit it to taste"
fi

# Resolve an absolute cloudflared path (it may be mise-managed, not on PATH).
CLOUDFLARED="$(command -v cloudflared 2>/dev/null || true)"
if [ -z "$CLOUDFLARED" ] && command -v mise >/dev/null 2>&1; then
  CLOUDFLARED="$(mise which cloudflared 2>/dev/null || true)"
fi

echo "==> Installing systemd user units -> $UNIT_DIR"
mkdir -p "$UNIT_DIR"
cp "$REPO_DIR/systemd/luxa.service" "$UNIT_DIR/luxa.service"

TUNNEL=1
if [ -n "$CLOUDFLARED" ]; then
  sed "s#@CLOUDFLARED@#$CLOUDFLARED#g" \
    "$REPO_DIR/systemd/luxa-tunnel.service" > "$UNIT_DIR/luxa-tunnel.service"
  echo "    cloudflared: $CLOUDFLARED"
else
  TUNNEL=0
  echo "    cloudflared not found — skipping the tunnel service."
  echo "    Install cloudflared and re-run to enable it."
fi

echo "==> Enabling and starting services (systemctl --user)"
systemctl --user daemon-reload
systemctl --user enable --now luxa.service
[ "$TUNNEL" -eq 1 ] && systemctl --user enable --now luxa-tunnel.service

echo
echo "Done."
echo "  status:  systemctl --user status luxa.service"
echo "  logs:    journalctl --user -u luxa.service -f"
[ "$TUNNEL" -eq 1 ] && \
echo "  URL:     journalctl --user -u luxa-tunnel.service | grep trycloudflare"
echo
echo "Run at boot without an active login:  sudo loginctl enable-linger \"$USER\""
echo "Device access needs the udev rule (see docs/getting-started.md)."
