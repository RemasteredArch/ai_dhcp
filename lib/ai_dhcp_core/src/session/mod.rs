// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2026 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use core::net::Ipv4Addr;

/// A DHCP message sent from a client to a server or servers.
pub enum ClientMessage {
    Discover(DhcpDiscover),
    Request(DhcpRequest),
}

pub struct DhcpDiscover {
    pub requested_addr: Option<Ipv4Addr>,
}

pub struct DhcpRequest;

/// A DHCP message sent from a server to a client.
pub enum ServerMessage {
    Offer(DhcpOffer),
    Acknowledge(DhcpAcknowledge),
}

pub struct DhcpOffer {
    pub lease: crate::DhcpLease,
}

pub struct DhcpAcknowledge;
