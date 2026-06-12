# Katana Toggle

Instant toggle between speaker and headphone output for Creative Sound Blaster Katana V2/V2X soundbars. Single double-click, no Creative App needed.

## What it does

One click (or `katana-toggle.exe`):
1. Auto-closes Creative App if it's holding the COM port
2. Detects current audio output (speaker or headphone)
3. Flips to the other one

## Download

Grab `katana-toggle.exe` from the [Releases](https://github.com/jackytsz/katana-toggle/releases) page. No installation needed — it's a standalone 347 KB binary.

## Usage

```bash
katana-toggle.exe           # Toggle speaker ↔ headphone (default)
katana-toggle.exe toggle    # Same
katana-toggle.exe speakers  # Force speaker output
katana-toggle.exe headphones# Force headphone output
katana-toggle.exe status    # Show current output
```

Double-click the `.exe` to toggle instantly.

## Supported devices

| Model | USB PID | Status |
|-------|---------|--------|
| Sound Blaster Katana V2 (MF8380) | `0x3260` | ✅ Tested |
| Sound Blaster Katana V2X (MF8400) | `0x3283` | ✅ Supported |

## Build from source

```bash
git clone https://github.com/jackytsz/katana-toggle.git
cd katana-toggle
cargo build --release
# Binary at target/release/katana-toggle.exe
```

Requires Rust 1.80+.

## How it works

The Katana V2/V2X exposes a CDC ACM serial port over USB. All settings (EQ, lighting, output routing) are controlled through a proprietary binary protocol (`5A [cmd] [len] [payload]`) with AES-256-GCM challenge-response authentication.

This tool sends the output-switch command (`OP_OUTPUT = 0x2C`) directly, bypassing the Creative App entirely.

## Credits

All reverse engineering of the Creative Katana USB protocol was done by **Rasmus Moorats** ([nns.ee](https://blog.nns.ee/)):

- [Reverse engineering the Katana V2X](https://blog.nns.ee/2026/02/20/katana-v2x-re/) — protocol documentation, authentication scheme, firmware format
- [v2x crate](https://crates.io/crates/v2x) — Rust library and CLI (`v2x-ctl`) for full device control
- [v2x source](https://git.dog/xx/v2x) — includes V2 support added in v0.5.0

This project is a thin wrapper around the `v2x` library — it adds Windows-friendly auto-kill of Creative App, toggle logic, and a minimal CLI focused purely on output switching.

The original Katana V1 USB HID protocol was documented by [therion23/KatanaHacking](https://github.com/therion23/KatanaHacking).

## License

MIT — same as the `v2x` crate.
