# Releasing

Pushing a version tag builds the binaries and publishes a GitHub release
(`.github/workflows/release.yml`).

```bash
# 1. Bump `version` in Cargo.toml, then refresh Cargo.lock
cargo build
git commit -am "chore(release): v0.2.0"

# 2. Tag and push; the tag must match the Cargo.toml version
git tag v0.2.0
git push origin master v0.2.0
```

The workflow fails early if the tag and the `Cargo.toml` version differ.

## What gets published

| Target                      | Archive   | Status                      |
|-----------------------------|-----------|-----------------------------|
| `x86_64-unknown-linux-gnu`  | `.tar.gz` | Tested on real hardware     |
| `aarch64-unknown-linux-gnu` | `.tar.gz` | Built, untested             |
| `aarch64-apple-darwin`      | `.tar.gz` | Built, untested             |
| `x86_64-apple-darwin`       | `.tar.gz` | Built, untested             |
| `x86_64-pc-windows-msvc`    | `.zip`    | Built, untested             |

Each archive holds the `luxa` binary, `README.md`, `luxa.example.yaml` and
`99-luxafor.rules`. A `SHA256SUMS` file is attached to the release.

On macOS and Windows the operating system's own HID driver holds the device,
and `luxa` can only detach that driver on Linux. Expect those builds to need
extra setup (for example a WinUSB driver on Windows) before they can claim the
device.
