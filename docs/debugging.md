# Debugging notes

How we got the Luxafor talking, and how to triage a machine that won't.

## 1. Confirm the device is seen

```bash
lsusb | grep -i 04d8
# Bus 007 Device 014: ID 04d8:f372 Microchip Technology, Inc. LUXAFOR FLAG
```

If this is empty, it's a cable/port/hardware problem — nothing software can fix.

## 2. Permissions live on the USB node

`luxa` uses the pure-Rust `nusb` crate, which talks to the kernel via **usbfs**
(`/dev/bus/usb/<bus>/<dev>`). It detaches the kernel HID driver and claims
interface 0, then writes to the interrupt OUT endpoint. So the node whose
permissions matter is the USB device node, not `/dev/hidraw*`:

```bash
ls -l /dev/bus/usb/007/014
# crw-rw-r-- root root   -> only root can write  (before the udev rule)
# crw-rw-rw- root root   -> writable by everyone (after the rule + replug)
```

By default it's `0664` (root-only write). The udev rule (`99-luxafor.rules`)
grants `0666` for the Flag's VID/PID.

## 3. Making the rule apply

`udevadm trigger` (default `change` action) does not always flip the mode on an
already-connected device. A **replug** fires an `add` event, which re-evaluates
the rule and sets the node to `0666`. Equivalent without replugging:

```bash
sudo udevadm trigger --action=add --attr-match=idVendor=04d8
```

## 4. Success

```bash
cargo run -- flash blue --repeat 5   # ⚡ flashing blue (x5)
cargo run -- blink green --times 3   # 💡 blinked green (x3)
cargo run -- off                     # ⏻ off
```

## Quick triage checklist

| Symptom                                  | Likely cause / fix                                   |
|------------------------------------------|------------------------------------------------------|
| `lsusb` shows nothing                    | Cable/port/hardware                                  |
| "could not claim interface" / open fails | Write permission on the USB node → udev rule         |
| Rule installed but still denied          | Replug the device (re-fires the `add` event)         |
| Don't want to touch udev                 | Run the command with `sudo`                          |

> Historical note: the original Python implementation used `hidapi` (libusb
> backend), which also opened the USB node — so the permission story is the same.
> The Rust rewrite drops that C dependency and uses `nusb` directly.
