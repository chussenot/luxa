# CLI reference

```bash
luxa <COMMAND> [OPTIONS]
luxa --help            # or: luxa <COMMAND> --help
```

From a checkout, replace `luxa` with `cargo run --`.

## Commands

| Command     | What it does                           | Arguments and options (defaults)                         |
|-------------|----------------------------------------|----------------------------------------------------------|
| `flash`     | Hardware strobe (device-driven blink)  | `[COLOR]` (`red`), `--speed 0-255` (`10`), `--repeat 0-255` (`5`) |
| `blink`     | Software on/off toggle loop            | `[COLOR]` (`red`), `--times` (`5`), `--on` seconds (`0.3`), `--off` seconds (`0.3`) |
| `color`     | Set a solid colour                     | `<COLOR>` (required)                                     |
| `off`       | Turn all LEDs off                      | —                                                        |
| `colors`    | List built-in colour names             | —                                                        |
| `behaviors` | List the behaviors webhooks can use    | —                                                        |
| `serve`     | Run the [webhook server](webhooks.md)  | `-c, --config` (`luxa.yaml`), `--host`, `--port`         |

```bash
luxa flash blue --repeat 10 --speed 20
luxa blink green --times 8
luxa color "#ff8800"
luxa off
```

## Colours

A colour is a name or a six-digit hex value (`#ff8800` or `ff8800`).

Built-in names: `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`,
`orange`, `purple`, `pink`, `off`. `luxa colors` prints the current list.

## Commands while a server is running

Only one process can claim the USB device. While `luxa serve` is running
(including as a [systemd service](systemd.md)) it owns the device, so `flash`,
`blink`, `color` and `off` **forward to the server** instead of opening the
device themselves. If no server is reachable they drive the device directly. No
flag is needed.

```bash
luxa flash blue        # ⚡ flashing blue (x5) (via server)   when serving
luxa color green       # 🎨 set to green                      when standalone
```

| Environment variable | Purpose                                   | Default                 |
|----------------------|-------------------------------------------|-------------------------|
| `LUXA_SERVER`        | Server to forward to (`host:port` or URL) | `http://127.0.0.1:8765` |

## mise tasks

Shortcuts defined in `mise.toml`, run with `mise run <task>`:

| Task                                   | Runs                                        |
|----------------------------------------|---------------------------------------------|
| `flash`, `blink`, `off`                | The matching command with its defaults      |
| `colors`, `behaviors`                  | The matching listing                        |
| `demo`                                 | A short cycle of effects on the device      |
| `serve`                                | `luxa serve` with `./luxa.yaml`             |
| `tunnel`                               | A cloudflared quick tunnel to the server    |
| `health`, `incident`                   | `curl` the running server                   |
| `install-service`, `uninstall-service` | Install or remove the systemd user services |
| `lint`, `fmt`, `test`                  | clippy, rustfmt, `cargo test`               |
| `docs`                                 | Serve this documentation locally            |
