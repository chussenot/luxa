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

- **behavior** — one of the names from `luxa behaviors`.
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

Run `luxa behaviors` to list them at any time.

## Run

```bash
uv run luxa serve                      # uses ./luxa.yaml
uv run luxa serve -c other.yaml        # custom config
uv run luxa serve --host 0.0.0.0 --port 9000   # override host/port
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
device can't be reached. Interactive API docs are at `/docs` (FastAPI).

## Notes

- The single USB device is shared behind a lock, so overlapping requests run one
  at a time; a long behavior (e.g. a slow blink) delays queued ones.
- There's no auth — bind to `127.0.0.1` (the default) unless you add your own.
- To trigger it from the internet, tunnel the port out with
  [cloudflared](cloudflared.md) (and read the security note there first).
