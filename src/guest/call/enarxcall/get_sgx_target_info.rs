// SPDX-License-Identifier: Apache-2.0

use super::super::types::Argv;
use super::Alloc;
use crate::guest::alloc::{Allocator, Collect, Collector, Output};
use crate::item::enarxcall::sgx::TargetInfo;
use crate::item::enarxcall::Number;
use crate::Result;

use core::mem::size_of;

// GetSgxTargetInfo call, which writes the [SGX `TargetInfo`](TargetInfo) in `info` field.
pub struct GetSgxTargetInfo<'a> {
    pub info: &'a mut TargetInfo,
}

impl<'a> Alloc<'a> for GetSgxTargetInfo<'a> {
    const NUM: Number = Number::GetSgxTargetInfo;

    type Argv = Argv<1>;
    type Ret = ();

    type Staged = Output<'a, [u8; size_of::<TargetInfo>()], &'a mut [u8; size_of::<TargetInfo>()]>;
    type Committed = Self::Staged;
    type Collected = Result<()>;

    fn stage(self, alloc: &mut impl Allocator) -> Result<(Self::Argv, Self::Staged)> {
        let info = Output::stage(alloc, self.info.as_mut())?;
        Ok((Argv([info.offset()]), info))
    }

    fn collect(
        info: Self::Committed,
        ret: Result<Self::Ret>,
        col: &impl Collector,
    ) -> Self::Collected {
        if ret.is_ok() {
            info.collect(col);
        }
        ret
    }
}
