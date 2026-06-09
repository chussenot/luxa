"""Named behaviors mapping a webhook to a device action.

A behavior takes the open device, a parsed RGB colour, and a ``params`` object
(any object exposing the optional attributes ``times``, ``on``, ``off``,
``speed``, ``repeat``, ``duration``, ``wave_type`` — typically a
:class:`luxa.config.WebhookSpec`). Behaviors read the params they care about and
fall back to sensible defaults.

This module is intentionally free of config/server imports so it stays the
single source of truth for what behaviors exist.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Protocol

from .device import RGB, Luxafor


class Params(Protocol):
    """Optional tuning knobs a webhook may set; ``None`` means "use default"."""

    times: int | None
    on: float | None
    off: float | None
    speed: int | None
    repeat: int | None
    duration: int | None
    wave_type: int | None


def _opt(value, default):
    return default if value is None else value


@dataclass(frozen=True)
class Behavior:
    name: str
    description: str
    run: Callable[[Luxafor, RGB, Params], None]


def _solid(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.set_color(rgb)


def _off(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.off()


def _blink(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.blink(rgb, times=_opt(p.times, 5), on=_opt(p.on, 0.3), off=_opt(p.off, 0.3))
    dev.off()


def _slow_blink(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.blink(rgb, times=_opt(p.times, 3), on=_opt(p.on, 0.8), off=_opt(p.off, 0.8))
    dev.off()


def _fast_blink(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.blink(rgb, times=_opt(p.times, 8), on=_opt(p.on, 0.1), off=_opt(p.off, 0.1))
    dev.off()


def _flash(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.strobe(rgb, speed=_opt(p.speed, 10), repeat=_opt(p.repeat, 5))


def _fade(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.fade(rgb, duration=_opt(p.duration, 30))


def _pulse(dev: Luxafor, rgb: RGB, p: Params) -> None:
    dev.wave(
        rgb,
        wave_type=_opt(p.wave_type, 1),
        repeat=_opt(p.repeat, 3),
        speed=_opt(p.speed, 30),
    )


BEHAVIORS: dict[str, Behavior] = {
    b.name: b
    for b in [
        Behavior("solid", "Set a steady colour", _solid),
        Behavior("off", "Turn the light off", _off),
        Behavior("blink", "Software blink, ~0.3s on/off", _blink),
        Behavior("slow_blink", "Slow software blink, ~0.8s on/off", _slow_blink),
        Behavior("fast_blink", "Fast software blink, ~0.1s on/off", _fast_blink),
        Behavior("flash", "Hardware strobe/flash", _flash),
        Behavior("fade", "Fade to a colour", _fade),
        Behavior("pulse", "Hardware wave/pulse effect", _pulse),
    ]
}


def behavior_names() -> list[str]:
    return list(BEHAVIORS)
