"""Low-level driver for the Luxafor Flag over USB HID.

Protocol reference (Luxafor Flag, VID 0x04d8 / PID 0xf372). Every command is
an 8-byte report written with a leading report-id byte (0):

    [report_id=0, mode, ...args]

Modes used here:
    1  static colour : [0, 1, led, R, G, B]
    2  fade          : [0, 2, led, R, G, B, duration]
    3  strobe (flash): [0, 3, led, R, G, B, speed, 0, repeat]
    4  wave          : [0, 4, wave_type, R, G, B, 0, repeat, speed]
"""

from __future__ import annotations

import time

import hid

VENDOR_ID = 0x04D8
PRODUCT_ID = 0xF372

# LED targets
LED_ALL = 0xFF
LED_FRONT = 0x41  # back side of the flag
LED_BACK = 0x42  # front side of the flag

# Command modes
_MODE_STATIC = 1
_MODE_FADE = 2
_MODE_STROBE = 3
_MODE_WAVE = 4

RGB = tuple[int, int, int]

# A few convenient named colours.
COLORS: dict[str, RGB] = {
    "red": (255, 0, 0),
    "green": (0, 255, 0),
    "blue": (0, 0, 255),
    "yellow": (255, 255, 0),
    "cyan": (0, 255, 255),
    "magenta": (255, 0, 255),
    "white": (255, 255, 255),
    "orange": (255, 80, 0),
    "purple": (128, 0, 128),
    "pink": (255, 40, 90),
    "off": (0, 0, 0),
}


class LuxaforError(RuntimeError):
    """Raised when the device cannot be found or written to."""


def parse_color(value: str) -> RGB:
    """Parse a colour name (``red``) or hex string (``#ff8800`` / ``ff8800``)."""
    key = value.strip().lower()
    if key in COLORS:
        return COLORS[key]
    hexstr = key.lstrip("#")
    if len(hexstr) == 6:
        try:
            return (
                int(hexstr[0:2], 16),
                int(hexstr[2:4], 16),
                int(hexstr[4:6], 16),
            )
        except ValueError:
            pass
    raise LuxaforError(
        f"Unknown colour {value!r}. Use a name ({', '.join(COLORS)}) or hex like '#ff8800'."
    )


class Luxafor:
    """A connected Luxafor Flag device.

    Use as a context manager so the handle is always closed::

        with Luxafor() as lux:
            lux.set_color((255, 0, 0))
    """

    def __init__(self) -> None:
        self._dev = hid.device()
        try:
            self._dev.open(VENDOR_ID, PRODUCT_ID)
        except OSError as exc:  # pragma: no cover - hardware dependent
            raise LuxaforError(
                "Could not open the Luxafor device. Is it plugged in, and do you "
                "have permission? On Linux you may need the udev rule "
                "(see README)."
            ) from exc

    def __enter__(self) -> "Luxafor":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()

    def close(self) -> None:
        try:
            self._dev.close()
        except Exception:  # pragma: no cover - best effort
            pass

    def _write(self, payload: list[int]) -> None:
        report = [0] + payload
        report += [0] * (9 - len(report))  # pad to report-id + 8 bytes
        try:
            self._dev.write(report)
        except OSError as exc:  # pragma: no cover - hardware dependent
            raise LuxaforError(f"Write to Luxafor failed: {exc}") from exc

    def set_color(self, rgb: RGB, led: int = LED_ALL) -> None:
        """Set a solid colour immediately."""
        r, g, b = rgb
        self._write([_MODE_STATIC, led, r, g, b])

    def fade(self, rgb: RGB, duration: int = 30, led: int = LED_ALL) -> None:
        """Fade to a colour. ``duration`` is in device ticks (~10ms)."""
        r, g, b = rgb
        self._write([_MODE_FADE, led, r, g, b, duration])

    def off(self) -> None:
        """Turn all LEDs off."""
        self.set_color((0, 0, 0))

    def strobe(
        self, rgb: RGB, speed: int = 10, repeat: int = 5, led: int = LED_ALL
    ) -> None:
        """Hardware strobe/flash. ``speed`` and ``repeat`` are 0-255."""
        r, g, b = rgb
        self._write([_MODE_STROBE, led, r, g, b, speed, 0, repeat])

    def wave(
        self, rgb: RGB, wave_type: int = 1, repeat: int = 3, speed: int = 30
    ) -> None:
        """Hardware wave effect. ``wave_type`` 1-5, ``repeat``/``speed`` 0-255."""
        r, g, b = rgb
        self._write([_MODE_WAVE, wave_type, r, g, b, 0, repeat, speed])

    def blink(
        self,
        rgb: RGB,
        times: int = 5,
        on: float = 0.3,
        off: float = 0.3,
        led: int = LED_ALL,
    ) -> None:
        """Software blink: toggle the colour on/off ``times`` times."""
        for _ in range(times):
            self.set_color(rgb, led)
            time.sleep(on)
            self.set_color((0, 0, 0), led)
            time.sleep(off)
