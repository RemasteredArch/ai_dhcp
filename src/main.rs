// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2026 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

#![doc = include_str!("../README.md")]
#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::{error, info};
use embassy_executor::Spawner;

// Register `defmt` as the real-time transfer protocol handler and `panic_probe` as the panic
// handler.
use defmt_rtt as _;
use panic_probe as _;

use crate::wifi::UdpBuffers;

#[macro_use]
mod macros;
mod wifi;

/// The onboard status LED controlled by the CYW43439.
struct OnBoardLed<'ctl> {
    /// The control driver for the CYW43439.
    net_control: &'ctl mut cyw43::Control<'static>,
}

impl OnBoardLed<'_> {
    const GPIO_PIN_NUM: u8 = 0;
    const LED_ENABLE: bool = true;
    const LED_DISABLE: bool = false;

    /// Enable the LED, causing it to glow.
    async fn enable(&mut self) {
        self.net_control
            .gpio_set(Self::GPIO_PIN_NUM, Self::LED_ENABLE)
            .await
    }

    /// Disable the LED, causing it to no longer glow.
    async fn disable(&mut self) {
        self.net_control
            .gpio_set(Self::GPIO_PIN_NUM, Self::LED_DISABLE)
            .await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    handle_udp(spawner, p).await.unwrap();
}

/// Initialize Wi-Fi and start a UDP echo server.
async fn handle_udp(
    spawner: Spawner,
    p: embassy_rp::Peripherals,
) -> Result<Infallible, embassy_net::udp::BindError> {
    /// The byte length for the buffers used to send, receive, and inspect UDP datagrams.
    const BUF_SIZE: usize = 1024;
    /// The byte length of the largest UDP datagram commonly received.
    const LARGE_DATAGRAM_SIZE: usize = 512;
    /// The UDP port that a DHCP server listens on for DHCP discovery messages.
    const DHCP_SERVER_PORT: u16 = 67;

    let (net_stack, mut net_control) = wifi::init_wifi(spawner, p, Default::default())
        .await
        .unwrap();

    info!("Initializing buffers");
    let buffers =
        singleton!(UdpBuffers::new(), UdpBuffers<BUF_SIZE, {BUF_SIZE / LARGE_DATAGRAM_SIZE}>);
    info!("Binding to socket");
    let socket = wifi::UdpBinding::new(net_stack, DHCP_SERVER_PORT.into(), buffers);

    udp_echo(
        socket,
        OnBoardLed {
            net_control: &mut net_control,
        },
    )
    .await
    .unwrap();
    unreachable!()
}

/// Start a UDP echo server that responds to requests with the contents of the request and blinks
/// while it does so.
async fn udp_echo<'stack, const BUF_LEN: usize, const MAX_DATAGRAMS: usize>(
    mut socket: wifi::UdpBinding<'stack, BUF_LEN, MAX_DATAGRAMS>,
    mut led: OnBoardLed<'_>,
) -> Result<Infallible, embassy_net::udp::RecvError> {
    info!("Starting UDP echo on port 67");

    let mut output_buf = [0; BUF_LEN];

    loop {
        let (datagram, metadata) = socket.receive(&mut output_buf).await?;
        led.enable().await;
        info!("Received a new datagram: {}\n", metadata);

        match str::from_utf8(datagram) {
            Ok(as_str) => info!(
                "Replying with contents (str, {} bytes): {}\n",
                as_str.len(),
                as_str
            ),
            Err(_) => info!(
                "Replying with contents (bytes, {} bytes): {:?}\n",
                datagram.len(),
                datagram
            ),
        }

        if let Err(err) = socket.send(datagram, metadata.endpoint).await {
            error!("Failed to send datagram: {}", err);
        }

        led.disable().await;
    }
}
