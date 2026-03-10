// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2026 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

#![doc = include_str!("../README.md")]
#![no_std]

use core::net::Ipv4Addr;

use heapless::Vec;

pub mod session;

pub struct MacAddress {
    addr: [u8; 6],
}

impl MacAddress {
    pub const fn new(addr: [u8; 6]) -> Self {
        Self { addr }
    }

    pub const fn addr(&self) -> [u8; 6] {
        self.addr
    }

    #[expect(clippy::missing_panics_doc, reason = "3 < 6")]
    pub const fn oui_part(&self) -> [u8; 3] {
        *self.addr.first_chunk().unwrap()
    }

    #[expect(clippy::missing_panics_doc, reason = "3 < 6")]
    pub const fn nic_part(&self) -> [u8; 3] {
        *self.addr.last_chunk().unwrap()
    }
}

#[non_exhaustive]
pub struct Client {
    pub mac_addr: MacAddress,
}

pub struct DhcpLease {
    pub ip_addr: Ipv4Addr,
    pub dns_servers: Vec<Ipv4Addr, 3>,
}

pub trait LeaseProvider {
    type Error;

    async fn assign_lease(client: Client) -> Result<DhcpLease, Self::Error>;
}
