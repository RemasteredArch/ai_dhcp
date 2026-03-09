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

use defmt::{debug, error, info};
use embassy_executor::Spawner;
use embassy_time::Timer;

// Register `defmt` as the real-time transfer protocol handler and `panic_probe` as the panic
// handler.
use defmt_rtt as _;
use panic_probe as _;

use crate::wifi::UdpBuffers;

#[macro_use]
mod macros;
mod wifi;

struct OnBoardLed<'ctl> {
    net_control: &'ctl mut cyw43::Control<'static>,
}

impl OnBoardLed<'_> {
    const GPIO_PIN_NUM: u8 = 0;
    const LED_ENABLE: bool = true;
    const LED_DISABLE: bool = false;

    async fn enable(&mut self) {
        self.net_control
            .gpio_set(Self::GPIO_PIN_NUM, Self::LED_ENABLE)
            .await
    }

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
    // blink_forever(&mut net_control).await
}

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

        print_stack_pointer();

        led.disable().await;
    }
}

/// Print the stack pointer of the caller, the stack address of a byte variable (to hint at the
/// actual end of the stack), and the absolute difference of these two addresses.
// Must be inlined to make sure it reports the stack pointer of the caller, not of this function.
#[inline(always)]
fn print_stack_pointer() {
    // SAFETY: all this does is copy `sp` (actually `r13`) into a general-purpose register.
    //
    // TO-DO: does `mov Rd, sp` _actually_ have no side effects?
    let sp = unsafe {
        let sp: u32;
        core::arch::asm!("mov {}, sp", out(reg) sp, options(pure, nomem, nostack, preserves_flags));
        sp
    };

    // Get the location of a byte placed on the stack, to provide an alternate data point from the
    // actual stack pointer.
    let b: u8 = 0;
    let ptr = &raw const b as u32;

    debug!("sp (r13):  0x{:x}", sp);
    debug!("u8 ptr:    0x{:x}", ptr);
    debug!("distance:  {}", ptr.abs_diff(sp));
}

#[allow(unused)]
async fn blink_forever(net_control: &mut cyw43::Control<'static>) -> ! {
    info!("Blinking forever");

    let mut led = OnBoardLed { net_control };

    loop {
        info!("enabling GPIO_0");
        led.enable().await;
        info!("pausing...");
        Timer::after_millis(500).await;

        info!("disabling GPIO_0");
        led.disable().await;
        info!("pausing...");
        Timer::after_millis(500).await;
    }
}
