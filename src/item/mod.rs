// SPDX-License-Identifier: Apache-2.0

//! Shared `sallyport` item definitions.

mod block;
pub mod enarxcall;
pub mod gdbcall;
pub mod syscall;

pub use block::*;
pub use enarxcall::Payload as Enarxcall;
pub use gdbcall::Payload as Gdbcall;
pub use syscall::Payload as Syscall;

use crate::libc::EINVAL;
use crate::Error;

use core::convert::{TryFrom, TryInto};
use core::mem::size_of;

/// The maximum size of a UDP packet
///
/// The maximum UDP message size is 65507, as determined by the following formula:
/// 0xffff - (sizeof(minimal IP Header) + sizeof(UDP Header)) = 65535-(20+8) = 65507
pub const MAX_UDP_PACKET_SIZE: usize = 65507;

pub(super) const LARGEST_PAYLOAD_SIZE: usize = {
    let mut max = size_of::<Gdbcall>();
    if size_of::<Syscall>() > max {
        max = size_of::<Syscall>();
    }
    if size_of::<Enarxcall>() > max {
        max = size_of::<Enarxcall>();
    }
    max
};

pub(super) const LARGEST_ITEM_SIZE: usize = size_of::<Header>() + LARGEST_PAYLOAD_SIZE;

/// `sallyport` item kind.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
pub enum Kind {
    End = 0x00,

    Syscall = 0x01,
    Gdbcall = 0x02,
    Enarxcall = 0x03,
}

impl TryFrom<usize> for Kind {
    type Error = Error;

    #[inline]
    fn try_from(kind: usize) -> Result<Self, Self::Error> {
        match kind {
            kind if kind == Kind::End as _ => Ok(Kind::End),
            kind if kind == Kind::Syscall as _ => Ok(Kind::Syscall),
            kind if kind == Kind::Gdbcall as _ => Ok(Kind::Gdbcall),
            kind if kind == Kind::Enarxcall as _ => Ok(Kind::Enarxcall),
            _ => Err(EINVAL),
        }
    }
}

pub(crate) const HEADER_USIZE_COUNT: usize = size_of::<Header>() / size_of::<usize>();

/// `sallyport` item header.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, align(8))]
pub struct Header {
    pub size: usize,
    pub kind: Kind,
}

impl TryFrom<[usize; HEADER_USIZE_COUNT]> for Header {
    type Error = Error;

    #[inline]
    fn try_from(header: [usize; HEADER_USIZE_COUNT]) -> Result<Self, Self::Error> {
        let [size, kind] = header;
        let kind = kind.try_into()?;
        Ok(Self { size, kind })
    }
}

/// `sallyport` item.
#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    Syscall(&'a mut Syscall, &'a mut [u8]),
    Gdbcall(&'a mut Gdbcall, &'a mut [u8]),
    Enarxcall(&'a mut Enarxcall, &'a mut [u8]),
}
