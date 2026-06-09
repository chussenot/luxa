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

## Docs

- [Getting started](docs/getting-started.md) — install, permissions, usage
- [Debugging notes](docs/debugging.md) — how we got the device talking
