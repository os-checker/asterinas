// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(net)]

mod addr;
mod common;
mod datagram;
pub mod options;
mod stream;

pub use datagram::DatagramSocket;
pub(in net) use datagram::observer::DatagramObserver;
pub(in net) use stream::observer::StreamObserver;
pub use stream::{StreamSocket, options as stream_options};
