# Webhooks

Drive the Luxafor from external systems (CI, deploys, alerts) over HTTP. You
declare a mapping of webhook name → behavior + color in YAML; `luxa serve` turns
each entry into a route. Design rationale: [ADR 0001](adr/0001-yaml-webhooks-to-luxafor-behaviors.md).

## Configure

Copy the example and edit it:

```bash
cp luxa.example.yaml luxa.yaml
```

```yaml
host: 127.0.0.1
port: 8765
webhooks:
  deploy-success:
    behavior: blink
    color: green
    times: 3
  deploy-failed:
    behavior: flash
    color: red
    repeat: 10
  incident:
    behavior: fast_blink
    color: "#ff0033"
  idle:
    behavior: "off"      # quote it — bare `off` is a YAML boolean
```

- **behavior** — one of the names from `cargo run -- behaviors`.
- **color** — a name (`red`, `green`, …) or hex (`#ff8800`).
- **tuning** (optional, per behavior) — `times`, `on`, `off`, `speed`,
  `repeat`, `duration`, `wave_type`. Omit to use the behavior's defaults.

The config is validated on startup: an unknown behavior or unparseable color
fails immediately with a clear message.

## Behaviors

| Behavior     | Effect                              | Tuning params         |
|--------------|-------------------------------------|-----------------------|
| `solid`      | Steady colour                       | —                     |
| `off`        | Light off                           | —                     |
| `blink`      | Software blink (~0.3s on/off)       | `times`, `on`, `off`  |
| `slow_blink` | Slow blink (~0.8s)                  | `times`, `on`, `off`  |
| `fast_blink` | Fast blink (~0.1s)                  | `times`, `on`, `off`  |
| `flash`      | Hardware strobe                     | `speed`, `repeat`     |
| `fade`       | Fade to colour                      | `duration`            |
| `pulse`      | Hardware wave                       | `wave_type`, `repeat`, `speed` |

Run `cargo run -- behaviors` to list them at any time.

## Run

```bash
cargo run -- serve                      # uses ./luxa.yaml
cargo run -- serve -c other.yaml        # custom config
cargo run -- serve --host 0.0.0.0 --port 9000   # override host/port
# or: mise run serve
```

## Trigger

Each webhook accepts `GET` or `POST`:

```bash
curl -X POST http://127.0.0.1:8765/webhook/deploy-success
# {"triggered":"deploy-success","behavior":"blink","color":"green"}

curl http://127.0.0.1:8765/health
# {"status":"ok","webhooks":[...]}
```

Responses: `200` on success, `404` for an unknown webhook name, `503` if the
device can't be reached.

## Notes

- The single USB device is shared behind a lock, so overlapping requests run one
  at a time; a long behavior (e.g. a slow blink) delays queued ones.
- There's no auth — bind to `127.0.0.1` (the default) unless you add your own.
- To trigger it from the internet, tunnel the port out with
  [cloudflared](cloudflared.md) (and read the security note there first).

## Direct commands while the server is running

The USB device can only be claimed by one process. While `luxa serve` is running
(including via the systemd service), it owns the device — so a second `luxa flash`
would otherwise fail with `interface is busy`.

Instead, the direct commands (`flash`, `blink`, `color`, `off`) **automatically
forward to a running server**: if one is reachable they POST to `/run` and you'll
see `(via server)` in the output; if no server is up they drive the device
directly. No flag needed.

```bash
luxa flash blue        # ⚡ flashing blue (x5) (via server)   when serving
luxa color green       # 🎨 set to green                      when standalone
```

Point the CLI at a non-default server with the `LUXA_SERVER` env var
(`host:port` or a full URL); it defaults to `http://127.0.0.1:8765`.

`POST /run` accepts the same fields as a webhook entry, e.g.:

```bash
curl -X POST localhost:8765/run \
  -H 'Content-Type: application/json' \
  -d '{"behavior":"flash","color":"blue","repeat":10}'
```
