use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{s_task_body::*, t_led_taskbody::*};

pub struct TTaskRs<'a, T>
where
	T: STaskBody,
{
	pub c_task_body: &'a T,
	task_ref: TaskRef<'a>,
}

pub struct ETaskForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a, ETaskbodyForTLedTaskbody<'a>>,
}

pub struct EiTaskForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a, ETaskbodyForTLedTaskbody<'a>>,
}

pub struct EiActivateNotificationHandlerForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a, ETaskbodyForTLedTaskbody<'a>>,
}

pub struct EiWakeUpNotificationHandlerForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a, ETaskbodyForTLedTaskbody<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_LEDTASK: TTaskRs<ETaskbodyForTLedTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_LEDTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LED).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR1SYMMETRIC_LEDTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASK,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR1SYMMETRIC_LEDTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASK,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_LEDTASK: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASK,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_LEDTASK: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASK,
};

impl<T: STaskBody> TTaskRs<'_, T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static TaskRef) {
		(
			self.c_task_body,
			&self.task_ref
		)
	}
}
