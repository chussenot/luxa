"""FastAPI app exposing the configured webhooks.

Each entry in ``config.webhooks`` becomes a ``/webhook/<name>`` route accepting
GET or POST. Handlers are plain ``def`` so FastAPI runs them in its threadpool —
a multi-second blink won't block the event loop, and the Controller's lock
serializes the actual device writes.
"""

from __future__ import annotations

from fastapi import FastAPI, HTTPException

from .config import ServerConfig
from .controller import Controller
from .device import LuxaforError


def create_app(config: ServerConfig, controller: Controller) -> FastAPI:
    app = FastAPI(title="luxa webhooks", version="0.1.0")

    @app.get("/health")
    def health() -> dict:
        return {"status": "ok", "webhooks": sorted(config.webhooks)}

    @app.api_route("/webhook/{name}", methods=["GET", "POST"])
    def trigger(name: str) -> dict:
        spec = config.webhooks.get(name)
        if spec is None:
            raise HTTPException(
                status_code=404,
                detail=f"unknown webhook {name!r}; known: {sorted(config.webhooks)}",
            )
        try:
            controller.run(spec)
        except LuxaforError as exc:
            raise HTTPException(status_code=503, detail=str(exc)) from exc
        return {"triggered": name, "behavior": spec.behavior, "color": spec.color}

    return app
