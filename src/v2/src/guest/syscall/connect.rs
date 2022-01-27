// SPDX-License-Identifier: Apache-2.0

use super::types::{Argv, SockaddrInput};
use crate::guest::alloc::{Allocator, Collector, Input, Stage, Syscall};
use crate::Result;

use libc::{c_int, c_long};

pub struct Connect<T> {
    pub sockfd: c_int,
    pub addr: T,
}

unsafe impl<'a, T: Into<SockaddrInput<'a>>> Syscall<'a> for Connect<T> {
    const NUM: c_long = libc::SYS_connect;

    type Argv = Argv<3>;
    type Ret = ();

    type Staged = Input<'a, [u8], &'a [u8]>;
    type Committed = ();
    type Collected = Result<()>;

    fn stage(self, alloc: &mut impl Allocator) -> Result<(Self::Argv, Self::Staged)> {
        let addr = self.addr.into().stage(alloc)?;
        Ok((Argv([self.sockfd as _, addr.offset(), addr.len()]), addr))
    }

    fn collect(_: Self::Committed, ret: Result<Self::Ret>, _: &impl Collector) -> Self::Collected {
        ret
    }
}
