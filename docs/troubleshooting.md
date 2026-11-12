# Troubleshooting

Find your symptom in the table, then follow the linked section.

| Symptom                                               | Cause and fix                                                        |
|-------------------------------------------------------|----------------------------------------------------------------------|
| `lsusb` shows no `04d8:f372`                          | Cable, port or hardware. Nothing software can fix.                   |
| `failed to open device (errno 13)`                    | No write permission on the USB node → [install the udev rule](#permission-denied-errno-13) |
| `server (HTTP 503): could not open the Luxafor …`     | Same permission problem, reported by the running server → [same fix](#permission-denied-errno-13) |
| Rule installed but still denied                       | The rule was not applied to the connected device → [re-apply it](#the-rule-is-installed-but-not-applied) |
| `interface is busy` (errno 16)                        | Another process owns the device → [one owner at a time](#interface-is-busy-errno-16) |
| `could not read config luxa.yaml`                     | `serve` found no config → [point it at one](#could-not-read-config-luxayaml) |
| You do not want to touch udev                         | Run the command with `sudo`.                                         |

## Confirm the device is seen

```bash
lsusb -d 04d8:f372
# Bus 007 Device 014: ID 04d8:f372 Microchip Technology, Inc. LUXAFOR FLAG
```

The bus and device numbers give you the node to inspect below
(`/dev/bus/usb/007/014` here).

## Permission denied (errno 13)

`luxa` uses the pure-Rust `nusb` crate, which talks to the kernel via **usbfs**.
It detaches the kernel HID driver, claims interface 0 and writes to the
interrupt OUT endpoint. The permissions that matter are therefore on the USB
device node, not on `/dev/hidraw*`:

```bash
ls -l /dev/bus/usb/007/014
# crw-rw-r-- root root   -> only root can write  (no udev rule)
# crw-rw-rw- root root   -> writable by everyone (rule applied)
```

Install the rule, which grants `0666` for the Flag's vendor and product IDs:

```bash
sudo cp 99-luxafor.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger --action=add --attr-match=idVendor=04d8
```

When the error is prefixed with `server (HTTP 503)`, the CLI
[forwarded the command](cli.md#commands-while-a-server-is-running) to a running
server and the server could not open the device. Apply the same fix; if it still
fails, restart the server with `systemctl --user restart luxa.service`.

## The rule is installed but not applied

A plain `udevadm trigger` fires a `change` event, which does not always update
the mode of an already-connected device. An `add` event does. Either unplug and
replug the device, or run:

```bash
sudo udevadm trigger --action=add --attr-match=idVendor=04d8
```

## Interface is busy (errno 16)

Only one process can claim the device. A running `luxa serve` owns it, and the
direct commands forward to that server automatically. If you still see this
error, another program holds the device (a second `luxa serve`, or the vendor
application). Stop it, or send your command through the server.

## Could not read config luxa.yaml

`luxa serve` reads `luxa.yaml` from the current directory unless told otherwise.

- From a checkout: `cp luxa.example.yaml luxa.yaml`.
- With the [systemd service](systemd.md) installed, the config lives at
  `~/.config/luxa/luxa.yaml` and the server is already running. You do not need
  to start a second one; it would fail to bind port `8765`.
- Anywhere else: `luxa serve -c /path/to/luxa.yaml`.

## Verify

```bash
luxa flash blue --repeat 5   # ⚡ flashing blue (x5)
luxa blink green --times 3   # 💡 blinked green (x3)
luxa off                     # ⏻ off
```

> Historical note: the original Python implementation used `hidapi` (libusb
> backend), which also opened the USB node, so the permission story was the
> same. The Rust rewrite drops that C dependency and uses `nusb` directly. See
> [ADR 0002](adr/0002-rewrite-in-rust.md).
