"""Thread-safe access to the single Luxafor device for the server.

Opens one USB handle and serializes every behavior behind a lock, so concurrent
webhook requests can't interleave writes. Reconnects once if the device drops.
"""

from __future__ import annotations

import threading

from .behaviors import BEHAVIORS
from .config import WebhookSpec
from .device import Luxafor, LuxaforError, parse_color


class Controller:
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._dev: Luxafor | None = None

    def _device(self) -> Luxafor:
        if self._dev is None:
            self._dev = Luxafor()
        return self._dev

    def _reset(self) -> None:
        if self._dev is not None:
            self._dev.close()
            self._dev = None

    def run(self, spec: WebhookSpec) -> None:
        """Run the behavior named by ``spec``. Reopens once on device error."""
        rgb = parse_color(spec.color)
        behavior = BEHAVIORS[spec.behavior]
        with self._lock:
            try:
                behavior.run(self._device(), rgb, spec)
            except LuxaforError:
                # Handle may be stale (replug, sleep); reopen and retry once.
                self._reset()
                behavior.run(self._device(), rgb, spec)

    def close(self) -> None:
        with self._lock:
            self._reset()
