use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody<T, U>
where
	T: SXUart + 'static,
	U: SDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_dataqueue: &'static U,
}

pub struct ETaskbodyForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U>
where
	T: SXUart + 'static,
	U: SDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY: TXUartTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY,
};

impl<T: SXUart, U: SDataqueueRs> TXUartTaskbody<T, U> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
