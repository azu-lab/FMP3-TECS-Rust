use crate::{s_xuart_measure::*, t_xuart::*, si_dataqueue_rs::*, t_dataqueue_rs::*};

use crate::{s_xuart_measure::*, t_xuart::*, s_led::*, t_mio_led::*, si_dataqueue_rs::*, t_dataqueue_rs::*, s_dataqueue_rs::*};

pub struct TXuartTaskbody<'a, T, U, V, W>where
	T: SXuartMeasure,
	U: SLed,
	V: SiDataqueueRs,
	W: SDataqueueRs,
{
	c_xuart: &'a T,
	c_led: &'a U,
	c_dataqueue: &'a V,
	c_dataqueue_led: &'a W,
}

pub struct ETaskbodyForTXuartTaskbody<'a>{
	pub cell: &'a TXuartTaskbody<'a, EXuartForTXuart<'a>, ELedForTMioLed<'a>, EiDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct EXuartMainForTXuartTaskbody<'a>{
	pub cell: &'a TXuartTaskbody<'a, EXuartForTXuart<'a>, ELedForTMioLed<'a>, EiDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTXuartTaskbody<'a, T, U, V, W>where
	T: SXuartMeasure,
	U: SLed,
	V: SiDataqueueRs,
	W: SDataqueueRs,
{
	pub c_xuart: &'a T,
	pub c_led: &'a U,
	pub c_dataqueue: &'a V,
	pub c_dataqueue_led: &'a W,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXuartTaskbody<EXuartForTXuart, ELedForTMioLed, EiDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TXuartTaskbody {
	c_xuart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_led: &ELEDFORRPROCESSOR1SYMMETRIC_LED,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXuartTaskbody = ETaskbodyForTXuartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[link_section = ".rodata"]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXuartMainForTXuartTaskbody = EXuartMainForTXuartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXuartMeasure, U: SLed, V: SiDataqueueRs, W: SDataqueueRs> TXuartTaskbody<'_, T, U, V, W> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXuartTaskbody<'_, T, U, V, W> {
		LockGuardForTXuartTaskbody {
			c_xuart: self.c_xuart,
			c_led: self.c_led,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
		}
	}
}
