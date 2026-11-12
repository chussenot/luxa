# luxa

`luxa` drives a **Luxafor Flag** (USB `04d8:f372`) from the command line and
from HTTP webhooks. It is a single Rust binary that talks to the device directly
over USB.

| If you want to…                              | Read                                    |
|----------------------------------------------|-----------------------------------------|
| Build it and light the device once           | [Getting started](getting-started.md)   |
| Look up a command, flag or colour            | [CLI reference](cli.md)                 |
| Map HTTP calls to light behaviors            | [Webhooks](webhooks.md)                 |
| Keep the server running across reboots       | [Run as a service](systemd.md)          |
| Trigger the light from outside your machine  | [Expose with cloudflared](cloudflared.md) |
| Fix a device that does not respond           | [Troubleshooting](troubleshooting.md)   |
| Understand why it is built this way          | [Decision records](adr/index.md)        |
