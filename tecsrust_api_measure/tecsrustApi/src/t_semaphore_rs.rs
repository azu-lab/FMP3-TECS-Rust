use itron::semaphore::SemaphoreRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
pub struct TSemaphoreRs<'a>
{
	semaphore_ref: SemaphoreRef<'a>,
}

pub struct ESemaphoreForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

pub struct EiSemaphoreForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

pub struct EiNotificationHandlerForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_SEMAPHORE: TSemaphoreRs = TSemaphoreRs {
	semaphore_ref: unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())},
};

#[link_section = ".rodata"]
pub static ESEMAPHOREFORRPROCESSOR1SYMMETRIC_SEMAPHORE: ESemaphoreForTSemaphoreRs = ESemaphoreForTSemaphoreRs {
	cell: &RPROCESSOR1SYMMETRIC_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static EISEMAPHOREFORRPROCESSOR1SYMMETRIC_SEMAPHORE: EiSemaphoreForTSemaphoreRs = EiSemaphoreForTSemaphoreRs {
	cell: &RPROCESSOR1SYMMETRIC_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static EINOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_SEMAPHORE: EiNotificationHandlerForTSemaphoreRs = EiNotificationHandlerForTSemaphoreRs {
	cell: &RPROCESSOR1SYMMETRIC_SEMAPHORE,
};

impl<> TSemaphoreRs<'_> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> &'static SemaphoreRef {
		&self.semaphore_ref
	}
}
