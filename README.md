# `ai_dhcp`

`ai_dhcp`: a blazingly fast 🚀 DHCP server 🌐 that's ✨ powered by AI ✨
to make your network stable like never before.

## Motivations and Architecture

`ai_dhcp` is a **satire piece**. Please do not actually unleash it onto a real network.
Or do, I can't stop you. Chaos _is_ the point, after all.

`ai_dhcp` is not yet complete, but the rough architecture is as follows:

- A Raspberry Pi Pico W running `ai_dhcp` acts as a [DHCPv4] server,
  passing basic client information off to an LLM that decides the response sent back to the client.
  - This does not require any extra hardware,
    just the Pico and, ideally, a debug probe.
- Another computer on the network provides a simple enough LLM API for `ai_dhcp` to make DHCP decisions from.
  I tried to find an option that would let me run this locally for convenience,
  but I am afraid to say that 264 KiB of memory would appear to be too little to run any LLM.
  - This will probably be something like [Ollama] or [`llama.cpp`] on an ordinary PC.

This could also be ported to run on other hardware without too much difficulty.
If you're interested, let me know.

## Setup

- Install the RP2040 target with `rustup target add thumbv6m-none-eabi`.
- [Install `probe-rs`](https://probe.rs/docs/getting-started/installation/).
- Connect your Raspberry Pi W to power and a [Debug Probe].
  - You _can_ set this up without a Debug Probe.
    Refer to the `probe-rs` documentation for how to do this.
  - WSL users should refer to [USBIPD] to pass the Debug Probe into WSL virtual machines.
    See <https://learn.microsoft.com/en-us/windows/wsl/connect-usb> for more.
- With the environment variables `AI_DHCP_NET_{SSID,PASSWORD}` defined
  to the appropriate credentials for your Wi-Fi ~~target~~ network,
  simply `cargo run`.

## License

`ai_dhcp` is licensed under the Mozilla Public License,
version 2.0 or (as the license stipulates) any later version.
A copy of the license should be distributed with `ai_dhcp`,
located at [`LICENSE`](./LICENSE),
or you can obtain one at <https://mozilla.org/MPL/2.0/>.

`ai_dhcp` contains code from other software.
Besides the usual licenses of dependencies, two are of note:

- Parts of `src/wifi.rs` are heavily modified from <https://github.com/embassy-rs/embassy/blob/5c1ca25/examples/rp/src/bin/wifi_tcp_server.rs>
  and `build.rs` is heavily modified from <https://github.com/embassy-rs/embassy/blob/5c1ca25/examples/rp/build.rs>.
  Original Embassy source code copyright Dario Nieuwenhuis et al., licensed under either the
  [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  or the [MIT License](https://opensource.org/license/MIT).
- This crate depends on [`cyw43-firmware`](https://crates.io/crates/cyw43-firmware),
  which includes firmware copyright George Robotics Pty Ltd,
  distributed under either a general noncommercial license
  or a more permissive license that may only be used with the RP2040 or other Raspberry Pi Ltd devices;
  see [`LICENSE-RPI`](./LICENSE-RPI) for the latter license.
  The Rust wrapper is written by Keziah Biermann and distributed under the [Unlicense](https://unlicense.org/).

[debug probe]: https://www.raspberrypi.com/products/debug-probe/
[dhcpv4]: https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol
[ollama]: https://ollama.com/
[usbipd]: https://github.com/dorssel/usbipd-win
[`llama.cpp`]: https://github.com/ggml-org/llama.cpp
