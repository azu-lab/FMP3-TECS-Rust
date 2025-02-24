use crate::{t_led_taskbody::*, s_led::*, s_task_body::*};

impl STaskBody for ETaskbodyForTLedTaskbody<'_>{

	fn main(&'static self) {
		let c_led = self.cell.get_cell_ref();

	}
}

