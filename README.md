# luxa

![](luxafor.jpg)

**Notifications you see without looking.** A desk light that any script or
service can switch on.

## Why

Notifications compete for the same screen you are trying to work on, and most
of them lose: the tab is hidden, the channel is muted, the headphones are on.
A light in the corner of your eye does not need focus, a window, or sound.

`luxa` makes a **Luxafor Flag** scriptable. One command sets the light from a
shell; one YAML file maps HTTP webhooks to colours and blink patterns, so
anything that can send a request can reach your desk. It is a single binary
that speaks USB directly, with no vendor application and no system libraries.

## Three ways to use it

1. **A coding agent or long job that needs you.** Turn the light yellow when an
   agent is waiting for an answer and green when the build or test run is done.
   You stop checking the terminal and come back when the light changes.
2. **A doorbell you can see.** Have your home automation call a webhook when
   someone rings or the phone goes off. Useful with noise-cancelling headphones,
   and for people who are deaf or hard of hearing.
3. **An "occupied" sign for a shared resource.** Light it red while someone
   holds the staging environment, a deploy freeze is on, or the shared GPU is
   busy. The team sees the state without asking in chat.

## Documentation

Start with [Getting started](docs/getting-started.md); the full set is in
[`docs/`](docs/index.md). Linux needs a one-time udev rule, covered there.
Browse it as a site with `mise run docs`.

## Disclaimer

`luxa` is an independent project. It is not affiliated with, endorsed by, or
supported by Luxafor. "Luxafor" is a trademark of its respective owner.
