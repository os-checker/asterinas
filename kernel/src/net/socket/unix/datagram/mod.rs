// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(net)]

mod message;
mod socket;

pub(in net) use message::UNIX_DATAGRAM_DEFAULT_BUF_SIZE;
pub use socket::UnixDatagramSocket;
