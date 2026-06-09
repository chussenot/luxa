#!/bin/sh
# Expose the local webhook port via a cloudflared quick tunnel and print the
# public URL. Port overridable with LUXA_PORT (default 8765).
set -eu

port="${LUXA_PORT:-8765}"
log="$(mktemp)"

cloudflared tunnel --url "http://127.0.0.1:${port}" > "$log" 2>&1 &
cf=$!
trap 'kill "$cf" 2>/dev/null || true' EXIT INT TERM

echo "starting cloudflared tunnel -> http://127.0.0.1:${port} ..."

url=""
i=0
while [ "$i" -lt 60 ]; do
  url="$(grep -oE 'https://[a-z0-9-]+\.trycloudflare\.com' "$log" | head -1 || true)"
  [ -n "$url" ] && break
  kill -0 "$cf" 2>/dev/null || break
  sleep 0.5
  i=$((i + 1))
done

if [ -n "$url" ]; then
  echo ""
  echo "🌐 Public URL: $url"
  echo "   Trigger:    curl -X POST $url/webhook/incident"
  echo "   (Ctrl-C to stop the tunnel)"
  echo ""
else
  echo "Could not detect the tunnel URL; cloudflared output:"
  cat "$log"
fi

wait "$cf"
