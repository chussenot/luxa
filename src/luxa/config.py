"""Load and validate the YAML webhook configuration."""

from __future__ import annotations

from pathlib import Path

import yaml
from pydantic import BaseModel, ConfigDict, field_validator

from .behaviors import BEHAVIORS
from .device import LuxaforError, parse_color


class WebhookSpec(BaseModel):
    """One webhook: which behavior to run, in which colour, with what tuning."""

    model_config = ConfigDict(extra="forbid")

    behavior: str
    color: str = "red"

    # Optional per-behavior tuning; None => the behavior's default is used.
    times: int | None = None
    on: float | None = None
    off: float | None = None
    speed: int | None = None
    repeat: int | None = None
    duration: int | None = None
    wave_type: int | None = None

    @field_validator("behavior", mode="before")
    @classmethod
    def _coerce_yaml_bool(cls, v: object) -> object:
        # YAML 1.1 parses bare `off`/`on` as booleans; map them back to strings
        # so `behavior: off` works unquoted.
        if isinstance(v, bool):
            return "off" if v is False else "on"
        return v

    @field_validator("behavior")
    @classmethod
    def _known_behavior(cls, v: str) -> str:
        if v not in BEHAVIORS:
            raise ValueError(
                f"unknown behavior {v!r}; choose one of {', '.join(BEHAVIORS)}"
            )
        return v

    @field_validator("color")
    @classmethod
    def _parseable_color(cls, v: str) -> str:
        try:
            parse_color(v)
        except LuxaforError as exc:
            raise ValueError(str(exc)) from exc
        return v


class ServerConfig(BaseModel):
    """Top-level config: where to listen and the webhook map."""

    model_config = ConfigDict(extra="forbid")

    host: str = "127.0.0.1"
    port: int = 8765
    webhooks: dict[str, WebhookSpec]


class ConfigError(RuntimeError):
    """Raised when the config file is missing or invalid."""


def load_config(path: Path) -> ServerConfig:
    """Read and validate a YAML config file into a :class:`ServerConfig`."""
    if not path.exists():
        raise ConfigError(f"config file not found: {path}")
    try:
        data = yaml.safe_load(path.read_text()) or {}
    except yaml.YAMLError as exc:
        raise ConfigError(f"invalid YAML in {path}: {exc}") from exc
    try:
        return ServerConfig.model_validate(data)
    except Exception as exc:  # pydantic ValidationError -> friendly message
        raise ConfigError(f"invalid config in {path}:\n{exc}") from exc
