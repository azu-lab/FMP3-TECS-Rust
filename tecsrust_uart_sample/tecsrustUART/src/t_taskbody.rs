use crate::{s_xuart_measure::*, t_xuart::*, s_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TTaskbody<'a, T, U>where
	T: SXuartMeasure,
	U: SDataqueueRs,
{
	c_xuart: &'a T,
	c_dataqueue: &'a U,
}

pub struct ETaskbodyForTTaskbody<'a>{
	pub cell: &'a TTaskbody<'a, EXuartForTXuart<'a>, EDataqueueForTDataqueueRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXuartForTXuart, EDataqueueForTDataqueueRs> = TTaskbody {
	c_xuart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

impl<T: SXuartMeasure, U: SDataqueueRs> TTaskbody<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U) {
		(
			self.c_xuart,
			self.c_dataqueue
		)
	}
}
