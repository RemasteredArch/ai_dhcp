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

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

// Register `defmt` as the real-time transfer protocol handler and `panic_probe` as the panic
// handler.
use defmt_rtt as _;
use panic_probe as _;

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

    let (net_stack, mut net_control) = wifi::init_wifi(spawner, p, Default::default())
        .await
        .unwrap();

    let mut led = OnBoardLed {
        net_control: &mut net_control,
    };

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
