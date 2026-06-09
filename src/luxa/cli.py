"""Typer CLI entry point for controlling a Luxafor Flag."""

from __future__ import annotations

from pathlib import Path
from typing import Annotated

import typer

from .behaviors import BEHAVIORS
from .device import COLORS, Luxafor, LuxaforError, parse_color

app = typer.Typer(
    help="Control a Luxafor Flag: flash, blink, set colours, run a webhook server.",
    no_args_is_help=True,
    add_completion=False,
)


def _fail(msg: str) -> None:
    typer.secho(f"error: {msg}", fg=typer.colors.RED, err=True)
    raise typer.Exit(code=1)


ColorArg = Annotated[
    str,
    typer.Argument(help="Colour name (e.g. red) or hex like '#ff8800'."),
]


@app.command()
def flash(
    color: ColorArg = "red",
    speed: Annotated[int, typer.Option(help="Strobe speed, 0-255.")] = 10,
    repeat: Annotated[int, typer.Option(help="Number of flashes, 0-255.")] = 5,
) -> None:
    """Hardware strobe/flash the device."""
    try:
        rgb = parse_color(color)
        with Luxafor() as lux:
            lux.strobe(rgb, speed=speed, repeat=repeat)
        typer.secho(f"⚡ flashing {color} (x{repeat})", fg=typer.colors.GREEN)
    except LuxaforError as exc:
        _fail(str(exc))


@app.command()
def blink(
    color: ColorArg = "red",
    times: Annotated[int, typer.Option(help="How many on/off cycles.")] = 5,
    on: Annotated[float, typer.Option(help="Seconds the light stays on.")] = 0.3,
    off: Annotated[float, typer.Option(help="Seconds the light stays off.")] = 0.3,
) -> None:
    """Software blink: toggle the colour on and off."""
    try:
        rgb = parse_color(color)
        with Luxafor() as lux:
            lux.blink(rgb, times=times, on=on, off=off)
            lux.off()
        typer.secho(f"💡 blinked {color} (x{times})", fg=typer.colors.GREEN)
    except LuxaforError as exc:
        _fail(str(exc))


@app.command()
def color(target: ColorArg) -> None:
    """Set a solid colour."""
    try:
        rgb = parse_color(target)
        with Luxafor() as lux:
            lux.set_color(rgb)
        typer.secho(f"🎨 set to {target}", fg=typer.colors.GREEN)
    except LuxaforError as exc:
        _fail(str(exc))


@app.command()
def off() -> None:
    """Turn the device off."""
    try:
        with Luxafor() as lux:
            lux.off()
        typer.secho("⏻ off", fg=typer.colors.GREEN)
    except LuxaforError as exc:
        _fail(str(exc))


@app.command()
def colors() -> None:
    """List the built-in colour names."""
    for name in COLORS:
        typer.echo(name)


@app.command()
def behaviors() -> None:
    """List the behaviors available to webhooks."""
    for name, b in BEHAVIORS.items():
        typer.echo(f"{name:<12} {b.description}")


@app.command()
def serve(
    config: Annotated[
        Path, typer.Option("--config", "-c", help="Path to the YAML config.")
    ] = Path("luxa.yaml"),
    host: Annotated[
        str | None, typer.Option(help="Override the host from the config.")
    ] = None,
    port: Annotated[
        int | None, typer.Option(help="Override the port from the config.")
    ] = None,
) -> None:
    """Run the webhook server defined by a YAML config."""
    # Imported lazily so the core CLI works without the server extras installed.
    import uvicorn

    from .config import ConfigError, load_config
    from .controller import Controller
    from .server import create_app

    try:
        cfg = load_config(config)
    except ConfigError as exc:
        _fail(str(exc))

    if host is not None:
        cfg.host = host
    if port is not None:
        cfg.port = port

    controller = Controller()
    application = create_app(cfg, controller)
    typer.secho(
        f"🚦 serving {len(cfg.webhooks)} webhook(s) on http://{cfg.host}:{cfg.port}",
        fg=typer.colors.GREEN,
    )
    for name, spec in cfg.webhooks.items():
        typer.echo(f"   POST /webhook/{name} → {spec.behavior} {spec.color}")
    try:
        uvicorn.run(application, host=cfg.host, port=cfg.port)
    finally:
        controller.close()


if __name__ == "__main__":
    app()
