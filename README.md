# luxa

A tiny [Typer](https://typer.tiangolo.com/) CLI to flash/blink a **Luxafor Flag**
(USB `04d8:f372`). Managed with [mise](https://mise.jdx.dev/) + [uv](https://docs.astral.sh/uv/).

```bash
mise install && mise run install   # python + uv + deps
uv run luxa flash blue             # ⚡ strobe
uv run luxa blink green --times 8  # 💡 on/off loop
uv run luxa off
```

Linux needs a one-time udev rule for device access — see [docs/getting-started.md](docs/getting-started.md).

## Webhooks

Map HTTP webhooks to device behaviors via YAML, then run a server:

```bash
cp luxa.example.yaml luxa.yaml   # edit to taste
uv run luxa serve                # POST /webhook/<name>
curl -X POST localhost:8765/webhook/deploy-success
```

See [docs/webhooks.md](docs/webhooks.md).

## Docs

- [Getting started](docs/getting-started.md) — install, permissions, usage
- [Webhooks](docs/webhooks.md) — YAML config → behaviors over HTTP
- [Debugging notes](docs/debugging.md) — how we got the device talking
- [Architecture decisions](docs/adr/) — ADRs
