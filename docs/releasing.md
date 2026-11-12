# Releasing

Pushing a version tag builds the binaries and publishes a GitHub release
(`.github/workflows/release.yml`).

Versions and the changelog are managed by
[cocogitto](https://docs.cocogitto.io/) (`cog.toml`), from
[Conventional Commits](https://www.conventionalcommits.org/).

```bash
cog bump --auto              # next version from the commits since the last tag
git push origin master --tags
```

`cog bump` sets the version in `Cargo.toml` and `Cargo.lock`, updates
`CHANGELOG.md`, commits, and creates the `v<version>` tag. It does not push.

- `--auto` only bumps when there is a `feat` or `fix` commit (or a breaking
  change) since the last tag; `docs`, `ci` and `chore` commits alone do nothing.
- To choose the version yourself, including the first release:
  `cog bump --version 0.1.0`.
- Preview without changing anything: `cog bump --auto --dry-run`.

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
