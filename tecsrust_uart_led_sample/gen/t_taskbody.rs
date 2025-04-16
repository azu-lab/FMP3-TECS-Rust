use itron::mutex::MutexRef;
use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{s_xuart_measure::*, t_xuart::*, s_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TTaskbody<'a, T, U, V>where
	T: SXuartMeasure,
	U: SDataqueueRs,
	V: SDataqueueRs,
{
	c_xuart: &'a T,
	c_dataqueue: &'a U,
	c_dataqueue_led: &'a V,
	variable: &'a SyncTTaskbodyVar,
	ex_ctrl_ref: &'a TECSMutexRef<'a>,
}

pub struct TTaskbodyVar{
	pub buffer: [u32; 8],
	pub buffer_count: u32,
}

pub struct SyncTTaskbodyVar{
	unsafe_var: UnsafeCell<TTaskbodyVar>,
}

unsafe impl Sync for SyncTTaskbodyVar {}

pub struct ETaskbodyForTTaskbody<'a>{
	pub cell: &'a TTaskbody<'a, EXuartForTXuart<'a>, EDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTTaskbody<'a, T, U, V>where
	T: SXuartMeasure,
	U: SDataqueueRs,
	V: SDataqueueRs,
{
	pub c_xuart: &'a T,
	pub c_dataqueue: &'a U,
	pub c_dataqueue_led: &'a V,
	pub var: &'a mut TTaskbodyVar,
	ex_ctrl_ref: &'a TECSMutexRef<'a>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXuartForTXuart, EDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TTaskbody {
	c_xuart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODYVAR,
	ex_ctrl_ref: &RPROCESSOR2SYMMETRIC_TASKBODY_EX_CTRL_REF,
};

pub static RPROCESSOR2SYMMETRIC_TASKBODYVAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is accessed by multiple tasks, but is safe because it is operated exclusively by the mutex object.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
		buffer: [0, 0, 0, 0, 0, 0, 0, 0],
		buffer_count: 0,
	}),
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY_EX_CTRL_REF: TECSMutexRef = TECSMutexRef{
	inner: unsafe{MutexRef::from_raw_nonnull(NonZeroI32::new(TECS_RUST_EX_CTRL_1).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

impl<T: SXuartMeasure, U: SDataqueueRs, V: SDataqueueRs> Drop for LockGuardForTTaskbody<'_, T, U, V> {
	fn drop(&mut self){
		self.ex_ctrl_ref.unlock();
	}
}

impl<T: SXuartMeasure, U: SDataqueueRs, V: SDataqueueRs> TTaskbody<'_, T, U, V> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_, T, U, V> {
		self.ex_ctrl_ref.lock();
		LockGuardForTTaskbody {
			c_xuart: self.c_xuart,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
			ex_ctrl_ref: self.ex_ctrl_ref,
		}
	}
}
