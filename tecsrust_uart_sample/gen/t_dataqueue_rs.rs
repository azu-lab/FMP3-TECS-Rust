use itron::dataqueue::DataqueueRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
pub struct TDataqueueRs<'a>{
	dataqueue_ref: DataqueueRef<'a>,
}

pub struct EDataqueueForTDataqueueRs<'a>{
	pub cell: &'a TDataqueueRs<'a>,
}

pub struct EiDataqueueForTDataqueueRs<'a>{
	pub cell: &'a TDataqueueRs<'a>,
}

pub struct EiNotificationHandlerForTDataqueueRs<'a>{
	pub cell: &'a TDataqueueRs<'a>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_DATAQUEUE: TDataqueueRs = TDataqueueRs {
	dataqueue_ref: unsafe{DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_UART).unwrap())},
};

#[link_section = ".rodata"]
pub static EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EDataqueueForTDataqueueRs = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EiDataqueueForTDataqueueRs = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static EINOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EiNotificationHandlerForTDataqueueRs = EiNotificationHandlerForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

impl<> TDataqueueRs<'_> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> &'static DataqueueRef {
		&self.dataqueue_ref
	}
}
