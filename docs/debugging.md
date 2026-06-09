# Debugging notes

How we went from "device plugged in" to "device flashing". Useful if a fresh
machine refuses to talk to the Luxafor.

## 1. Confirm the device is seen

```bash
lsusb | grep -i 04d8
# Bus 007 Device 014: ID 04d8:f372 Microchip Technology, Inc. LUXAFOR FLAG
```

If this is empty, it's a cable/port/hardware problem — nothing software can fix.

## 2. First failure: permission denied

`uv run luxa flash` errored with *"Could not open the Luxafor device"*.
The HID nodes were all root-only:

```bash
ls -l /dev/hidraw*        # crw------- root root  → no user access
```

So we wrote the udev rule (`99-luxafor.rules`) granting `MODE="0666"` to the
Luxafor's VID/PID and installed it. It still failed.

## 3. Finding the real node: libusb vs hidraw

Enumeration worked but `open()` did not — a clue that **reading** was allowed
but **writing** was not:

```bash
uv run python -c "import hid; print([d['path'] for d in hid.enumerate() if d['vendor_id']==0x04d8])"
# [b'7-1.3.2:1.0']   ← a libusb-style path, not /dev/hidrawN
```

That `bus-port:config.interface` path means the pip `hidapi` wheel is built
against the **libusb** backend. So it opens the USB device node, and *that's*
the node whose permissions matter:

```bash
ls -l /dev/bus/usb/007/014
# crw-rw-r-- root root   → group/other can read, only root can write
```

The `KERNEL=="hidraw*"` half of our rule was irrelevant here; the
`SUBSYSTEM=="usb"` half is what counts.

## 4. Making the rule actually apply

`udevadm trigger` (default `change` action) did not flip the mode on the
already-connected device. A **replug** fires an `add` event, which re-evaluates
the rule and sets the node to `0666`:

```bash
ls -l /dev/bus/usb/001/004
# crw-rw-rw- root root   ✓
```

(Equivalent without replugging: `sudo udevadm trigger --action=add --attr-match=idVendor=04d8`.)

## 5. Success

```bash
uv run luxa flash blue --repeat 5   # ⚡ flashing blue (x5)
uv run luxa blink green --times 3   # 💡 blinked green (x3)
uv run luxa color "#ff8800"         # 🎨 solid
uv run luxa off                     # ⏻ off
```

## Quick triage checklist

| Symptom                                  | Likely cause / fix                                   |
|------------------------------------------|------------------------------------------------------|
| `lsusb` shows nothing                    | Cable/port/hardware                                  |
| Enumerate works, `open()` fails          | Write permission on the USB node → udev rule         |
| Rule installed but still denied          | Replug the device (re-fires the `add` event)         |
| Don't want to touch udev                 | Run the command with `sudo`                          |
