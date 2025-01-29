use crate::{t_task_rs::*, s_task::*, si_task::*, s_task_body::*, si_notification_handler::*};
use itron::abi::*;

impl STask for ETaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn migrate_and_activate(&'static self, prcid: &ID) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn cancel_activate(&'static self) -> ER_UINT{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn migrate(&'static self, prcid: &ID) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn get_task_state(&'static self, p_tskstat: &mut STAT) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn change_priority(&'static self, priority: &PRI) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn change_sub_priority(&'static self, subPriority: &uint_t) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn get_priority(&'static self, p_priority: &mut PRI) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn refer(&'static self, pk_taskStatus: &mut T_RTSK) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn wakeup(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn cancel_wakeup(&'static self) -> ER_UINT{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn release_wait(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn suspend(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn resume(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn raise_terminate(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn terminate(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
}

impl SiTask for EiTaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn wakeup(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
	#[inline]
	fn release_wait(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();
		1

	}
}

impl SiNotificationHandler for EiActivateNotificationHandlerForTTaskRs<'_>{

}

impl SiNotificationHandler for EiWakeUpNotificationHandlerForTTaskRs<'_>{

}

