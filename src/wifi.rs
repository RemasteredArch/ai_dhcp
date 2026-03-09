// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2026 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.
//
// This module's implementation is heavily modified from
// <https://github.com/embassy-rs/embassy/blob/5c1ca25/examples/rp/src/bin/wifi_tcp_server.rs>.
// Original Embassy source code copyright Dario Nieuwenhuis et al., licensed under the
// [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the
// [MIT License](https://opensource.org/license/MIT).

//! `wifi`: set up and use the [CYW43439](https://www.infineon.com/part/CYW43439) Wi-Fi chip.
//!
//! See [`init_wifi`] and [`UdpBinding`].
//!
//! Note that this module is partially based off of Embassy code and that it uses firmware under a
//! proprietary license. See the [crate-level docs][`crate`].

use core::net::Ipv4Addr;

use defmt::{debug, info, trace};
use embassy_net::{Ipv4Cidr, StaticConfigV4, udp};
use embassy_rp::{gpio, peripherals};
use heapless::Vec;

embassy_rp::bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<peripherals::PIO0>;
});

/// A configuration that describes how a network device should connect to an IP network.
pub struct NetworkConfig<'s> {
    /// The service set identifier (SSID) of the target network.
    pub ssid: &'s str,
    /// The password used to log onto the network.
    pub password: &'s str,
    /// The IPv4 configuration with which to connect to the network with.
    pub config: embassy_net::Config,
}

impl<'s> NetworkConfig<'s> {
    /// An IPv4 configuration that matches my network.
    // TO-DO: make this configurable without editing source code (probably using DHCP).
    fn default_ipv4_config() -> embassy_net::Config {
        embassy_net::Config::ipv4_static(StaticConfigV4 {
            address: Ipv4Cidr::new(Ipv4Addr::new(192, 168, 68, 213), 22),
            gateway: Some(Ipv4Addr::new(192, 168, 68, 1)),
            dns_servers: Vec::from_slice(&[Ipv4Addr::new(1, 1, 1, 1), Ipv4Addr::new(1, 0, 0, 1)])
                .unwrap(),
        })
    }

    /// Connect to a network matching mine with the given credentials.
    pub fn new_with_default_ipv4(ssid: &'s str, password: &'s str) -> Self {
        Self {
            ssid,
            password,
            config: Self::default_ipv4_config(),
        }
    }
}

impl Default for NetworkConfig<'static> {
    fn default() -> Self {
        NetworkConfig::new_with_default_ipv4(env!("AI_DHCP_NET_SSID"), env!("AI_DHCP_NET_PASSWORD"))
    }
}

/// The buffers needed by a [`UdpBinding`] to process incoming and outgoing datagrams.
pub struct UdpBuffers<const BUF_LEN: usize, const MAX_DATAGRAMS: usize> {
    rx_meta: [udp::PacketMetadata; MAX_DATAGRAMS],
    rx_buf: [u8; BUF_LEN],
    tx_meta: [udp::PacketMetadata; MAX_DATAGRAMS],
    tx_buf: [u8; BUF_LEN],
}

impl<const BUF_LEN: usize, const MAX_DATAGRAMS: usize> UdpBuffers<BUF_LEN, MAX_DATAGRAMS> {
    /// Create an empty set of buffers.
    pub fn new() -> Self {
        Self {
            rx_meta: [udp::PacketMetadata::EMPTY; _],
            rx_buf: [0; _],
            tx_meta: [udp::PacketMetadata::EMPTY; _],
            tx_buf: [0; _],
        }
    }
}

impl<const BUF_LEN: usize, const MAX_DATAGRAMS: usize> Default
    for UdpBuffers<BUF_LEN, MAX_DATAGRAMS>
{
    fn default() -> Self {
        Self::new()
    }
}

/// A UDP socket bound to a given endpoint.
///
/// Provides an opinionated wrapper around [`embassy_net::udp::UdpSocket`].
pub struct UdpBinding<'stack, const BUF_LEN: usize, const MAX_DATAGRAMS: usize> {
    socket: udp::UdpSocket<'stack>,
}

impl<'stack, const BUF_LEN: usize, const MAX_DATAGRAMS: usize>
    UdpBinding<'stack, BUF_LEN, MAX_DATAGRAMS>
{
    /// Use the given network stack to create a socket bound to an endpoint.
    pub fn new(
        stack: embassy_net::Stack<'stack>,
        endpoint: embassy_net::IpListenEndpoint,
        buffers: &'static mut UdpBuffers<BUF_LEN, MAX_DATAGRAMS>,
    ) -> Self {
        let UdpBuffers {
            rx_meta,
            rx_buf,
            tx_meta,
            tx_buf,
        } = buffers;

        let mut socket = udp::UdpSocket::new(stack, rx_meta, rx_buf, tx_meta, tx_buf);
        socket.bind(endpoint).unwrap();

        Self { socket }
    }

    /// Returns the endpoint that this socket is bound to.
    pub async fn endpoint(&self) -> embassy_net::IpListenEndpoint {
        self.socket.endpoint()
    }

    /// Wait to receive a new UDP datagram, returning the subslice of `output_buf` that the new
    /// datagram's contents were written to (or the error encountered while receiving a new datagram).
    pub async fn receive<'out>(
        &mut self,
        output_buf: &'out mut [u8],
    ) -> Result<(&'out mut [u8], udp::UdpMetadata), udp::RecvError> {
        let (bytes, metadata) = self.socket.recv_from(output_buf).await?;
        Ok((&mut output_buf[..bytes], metadata))
    }

    /// Send a UDP datagram with the given contents to the given endpoint, returning an error if one
    /// was encountered.
    pub async fn send(
        &mut self,
        message: &[u8],
        to: embassy_net::IpEndpoint,
    ) -> Result<(), udp::SendError> {
        self.socket.send_to(message, to).await
    }
}

#[embassy_executor::task]
async fn wifi_event_loop(
    runner: cyw43::Runner<
        'static,
        gpio::Output<'static>,
        cyw43_pio::PioSpi<'static, peripherals::PIO0, 0, peripherals::DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn network_event_loop(
    mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>,
) -> ! {
    runner.run().await
}

/// Initialize the CYW43439 and connects to the given network, returning the network stack and
/// control over the device (or an error encountered when attempting to spawn their background
/// tasks). Must only be called once.
///
/// # Panics
///
/// - Panics if it cannot connect to the given network.
/// - Panics if this function has been called more than one time.
pub async fn init_wifi(
    spawner: embassy_executor::Spawner,
    p: embassy_rp::Peripherals,
    net_config: NetworkConfig<'_>,
) -> Result<(embassy_net::Stack<'static>, cyw43::Control<'static>), embassy_executor::SpawnError> {
    info!("Initalizing Wi-Fi");

    let pio = embassy_rp::pio::Pio::new(p.PIO0, Irqs);
    let (driver, mut control, runner) = cyw43::new(
        singleton!(cyw43::State::new(), cyw43::State),
        gpio::Output::new(p.PIN_23, gpio::Level::Low),
        cyw43_pio::PioSpi::new(
            // This isn't elevated into a static singleton in the Embassy example, but I found that
            // I could not get the onboard status LED to blink without doing so.
            singleton!(
                pio.common,
                embassy_rp::pio::Common<'static, peripherals::PIO0>
            ),
            pio.sm0,
            cyw43_pio::DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            gpio::Output::new(p.PIN_25, gpio::Level::High),
            p.PIN_24,
            p.PIN_29,
            p.DMA_CH0,
        ),
        cyw43_firmware::CYW43_43439A0,
    )
    .await;

    trace!("Spawning Wi-Fi event loop");
    spawner.spawn(wifi_event_loop(runner))?;

    trace!("Initializing Wi-Fi chip");
    control.init(cyw43_firmware::CYW43_43439A0_CLM).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let rng_seed = embassy_rp::clocks::RoscRng.next_u64();
    trace!("Spawning network event loop with random seed {}", rng_seed);
    let (stack, runner) = embassy_net::new(
        driver,
        net_config.config,
        singleton!(
            embassy_net::StackResources::new(),
            embassy_net::StackResources<3>
        ),
        rng_seed,
    );
    spawner.spawn(network_event_loop(runner))?;

    debug!("Joining Wi-Fi network {}", net_config.ssid);
    control
        .join(
            net_config.ssid,
            cyw43::JoinOptions::new(net_config.password.as_bytes()),
        )
        .await
        .expect("failed to join network!");

    debug!("Waiting for link signal");
    stack.wait_link_up().await;

    info!("Finished initializing Wi-Fi");
    Ok((stack, control))
}
