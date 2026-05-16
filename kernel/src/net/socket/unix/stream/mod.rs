// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(net)]

mod connected;
mod init;
mod listener;
mod socket;

pub(in net) use connected::UNIX_STREAM_DEFAULT_BUF_SIZE;
pub use socket::UnixStreamSocket;
