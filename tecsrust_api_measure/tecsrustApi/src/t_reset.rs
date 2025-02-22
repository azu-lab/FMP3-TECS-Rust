use crate::{s_task_rs::*, t_task_rs::*, s_semaphore_rs::*, t_semaphore_rs::*};

pub struct TReset<'a, T, U, V>
where
	T: STaskRs,
	U: STaskRs,
	V: SSemaphoreRs,
{
	c_task: &'a T,
	c_taskmig: &'a U,
	c_semaphore: &'a V,
}

pub struct EResetForTReset<'a>{
	pub cell: &'a TReset<'a, ETaskForTTaskRs<'a>, ETaskForTTaskRs<'a>, ESemaphoreForTSemaphoreRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_RESET: TReset<ETaskForTTaskRs, ETaskForTTaskRs, ESemaphoreForTSemaphoreRs> = TReset {
	c_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	c_taskmig: &ETASKFORRPROCESSOR1SYMMETRIC_TASKMIG,
	c_semaphore: &ESEMAPHOREFORRPROCESSOR1SYMMETRIC_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static ERESETFORRPROCESSOR2SYMMETRIC_RESET: EResetForTReset = EResetForTReset {
	cell: &RPROCESSOR2SYMMETRIC_RESET,
};

impl<T: STaskRs, U: STaskRs, V: SSemaphoreRs> TReset<'_, T, U, V> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U, &'static V) {
		(
			self.c_task,
			self.c_taskmig,
			self.c_semaphore
		)
	}
}
