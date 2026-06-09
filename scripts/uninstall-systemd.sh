#!/bin/sh
# Remove the luxa systemd user services. Leaves the binary and config in place.
set -u

systemctl --user disable --now luxa-tunnel.service 2>/dev/null || true
systemctl --user disable --now luxa.service 2>/dev/null || true
rm -f "$HOME/.config/systemd/user/luxa-tunnel.service" \
      "$HOME/.config/systemd/user/luxa.service"
systemctl --user daemon-reload 2>/dev/null || true

echo "Removed luxa systemd user services."
echo "Left in place: $HOME/.local/bin/luxa and $HOME/.config/luxa/"
