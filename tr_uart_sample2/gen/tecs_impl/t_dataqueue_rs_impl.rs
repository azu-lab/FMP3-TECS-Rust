use crate::tecs_global::*;
use crate::tecs_celltype::t_dataqueue_rs::*;
use crate::tecs_signature::{s_dataqueue_rs::*, si_dataqueue_rs::*};
impl SDataqueueRs for EDataqueueForTDataqueueRs{

	#[inline]
	fn send(&'static self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_polling(&'static self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_timeout(&'static self, data: &itron::dataqueue::DataElement, timeout: &itron::time::Timeout) -> Result<(), itron::error::Error<itron::dataqueue::SendTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&'static self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive(&'static self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_polling(&'static self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::TryRecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_timeout(&'static self, timeout: &itron::time::Timeout) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn initialize(&'static self) -> Result<(), itron::error::Error<itron::dataqueue::InitializeError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&'static self) -> Result<itron::dataqueue::Info, itron::error::Error<itron::dataqueue::InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs{

	#[inline]
	fn send_polling(&'static self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&'static self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
}

