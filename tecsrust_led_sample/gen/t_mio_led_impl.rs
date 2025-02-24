use crate::{t_mio_led::*, s_led::*};

impl SLed for ELedForTMioLed<'_>{

	fn set_up(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();

	}
	fn light_on(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();

	}
	fn light_off(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();

	}
}

