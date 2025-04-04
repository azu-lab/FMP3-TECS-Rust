use crate::{t_taskbody::*, s_task_body::*, s_xuart_measure::*, s_dataqueue_rs::*};

impl STaskBody for ETaskbodyForTTaskbody<'_>{

	fn main(&'static self) {
		let (c_xuart, c_dataqueue) = self.cell.get_cell_ref();

	}
}

