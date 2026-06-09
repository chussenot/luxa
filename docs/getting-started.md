# Getting started

## 1. Install tooling and dependencies

The project is pinned with `mise` (Python 3.12 + uv) and dependencies are
managed by `uv`.

```bash
mise install        # installs python + uv per mise.toml
mise run install    # uv sync — creates .venv and installs deps
```

## 2. Grant device access (Linux, one-time)

The Luxafor is a USB HID device that only `root` can write to by default.
The bundled `hidapi` wheel uses the **libusb** backend, so permissions apply to
the USB node (`/dev/bus/usb/<bus>/<dev>`), not `/dev/hidraw*`.

Install the udev rule, then **unplug and replug** the device:

```bash
sudo cp 99-luxafor.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
# then unplug + replug the Luxafor
```

A replug is what reliably re-evaluates the rule (`trigger` alone fires a
`change` event and may not re-apply the mode to an already-connected device).
Without the rule you can still run any command with `sudo`.

## 3. Use it

```bash
uv run luxa --help                  # all commands
uv run luxa flash                   # hardware strobe, red
uv run luxa flash blue --repeat 10 --speed 20
uv run luxa blink green --times 8
uv run luxa color "#ff8800"         # solid colour
uv run luxa off
uv run luxa colors                  # list named colours
```

Or via mise tasks: `mise run flash`, `mise run blink`, `mise run off`.

| Command  | What it does                          |
|----------|---------------------------------------|
| `flash`  | Hardware strobe (device-driven blink) |
| `blink`  | Software on/off toggle loop           |
| `color`  | Set a solid colour                    |
| `off`    | Turn all LEDs off                     |
| `colors` | List built-in colour names            |

Colours accept a name (`red`, `green`, `blue`, …) or hex (`#ff8800`).
