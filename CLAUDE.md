# CLAUDE.md

Guidance for Claude Code when working in this repo.

## What this is

`luxa` — a small [Typer](https://typer.tiangolo.com/) CLI that controls a
**Luxafor Flag** (USB HID `04d8:f372`): flash, blink, set colors, turn off.
Tooling is managed with **mise** (Python 3.12 + uv) and **uv** for deps.

## Layout

| Path                  | Purpose                                            |
|-----------------------|----------------------------------------------------|
| `src/luxa/device.py`  | Low-level HID driver (`Luxafor` class), wire bytes |
| `src/luxa/cli.py`     | Typer app — one function per command               |
| `mise.toml`           | Toolchain pin + `flash`/`blink`/`off`/`install` tasks |
| `pyproject.toml`      | Deps + `luxa` console script                       |
| `docs/`               | Setup (`getting-started.md`) and `debugging.md`    |
| `99-luxafor.rules`    | udev rule granting device access on Linux          |

## Commands

```bash
mise install && mise run install     # toolchain + deps (uv sync)
uv run luxa --help                   # all commands
uv run luxa flash blue --repeat 5    # hardware strobe
uv run luxa blink green --times 8    # software on/off loop
mise run flash | blink | off         # task shortcuts
```

Run CLI through mise/uv so the pinned interpreter and `.venv` are used:
`mise exec -- uv run luxa <cmd>`.

## Conventions

- **New effects:** add the wire-level method to `device.py` first (build the byte
  report, pad to report-id + 8 bytes), then a thin Typer command in `cli.py`
  following the existing pattern: color via `parse_color` (name or hex),
  wrap in `try/except LuxaforError`, report success with `typer.secho`.
- Colors are names (`red`, `green`, …) or hex (`#ff8800`); extend `COLORS` rather
  than hardcoding RGB tuples.
- Keep edits minimal and match surrounding style. Update `docs/` when setup or
  behavior changes.

## Hardware gotchas (see `docs/debugging.md`)

- The `hidapi` wheel uses the **libusb** backend → permissions live on the USB
  node `/dev/bus/usb/<bus>/<dev>`, **not** `/dev/hidraw*`.
- A udev rule needs a **replug** (or `udevadm trigger --action=add`) to apply to
  an already-connected device.
- `enumerate()` works but `open()` fails ⇒ write-permission issue on the USB node.

## Specialist agent

`.claude/agents/luxafor-expert.md` — delegate device-protocol, new-command, or
"device won't respond" tasks to the **luxafor-expert** subagent.

## Verifying

When hardware is connected, verify changes against the real device
(`uv run luxa flash`, etc.) and report exactly what it did — not just that tests
pass.
