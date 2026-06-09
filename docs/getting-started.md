# Getting started

## 1. Install the toolchain and build

The project is pinned with `mise` (Rust 1.96) and built with Cargo.

```bash
mise install        # installs the Rust toolchain per mise.toml
mise run install    # cargo build
# or an optimized binary:
mise run build      # cargo build --release  -> target/release/luxa
```

## 2. Grant device access (Linux, one-time)

The Luxafor is a USB HID device that only `root` can write to by default. `luxa`
talks to it via usbfs (`/dev/bus/usb/<bus>/<dev>`), so permissions apply to that
node. Install the udev rule, then **unplug and replug** the device:

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
cargo run -- --help                 # all commands
cargo run -- flash                  # hardware strobe, red
cargo run -- flash blue --repeat 10 --speed 20
cargo run -- blink green --times 8
cargo run -- color "#ff8800"        # solid colour
cargo run -- off
cargo run -- colors                 # list named colours
cargo run -- behaviors              # list webhook behaviors
```

(After `mise run build`, you can call `target/release/luxa <cmd>` directly.)

Or via mise tasks: `mise run flash`, `mise run blink`, `mise run off`.

| Command    | What it does                          |
|------------|---------------------------------------|
| `flash`    | Hardware strobe (device-driven blink) |
| `blink`    | Software on/off toggle loop           |
| `color`    | Set a solid colour                    |
| `off`      | Turn all LEDs off                     |
| `colors`   | List built-in colour names            |
| `behaviors`| List webhook behaviors                |
| `serve`    | Run the webhook server                |

Colours accept a name (`red`, `green`, `blue`, …) or hex (`#ff8800`).
