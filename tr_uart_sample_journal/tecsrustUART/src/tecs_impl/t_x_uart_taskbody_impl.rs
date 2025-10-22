use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart::*, s_dataqueue_rs::*, s_task_body::*};
impl STaskBody for ETaskbodyForTXUartTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();
		itron::task::delay(itron::time::duration!(s: 1)).expect("delay failed");

		lg.c_x_uart.open();

		loop {
			let result = lg.c_x_uart.get_char();
			match result {
				Ok(c) => {
					lg.var.buffer[lg.var.buffer_count as usize] = c;
					if lg.var.buffer_count + 1 >= *lg.buffer_size {
						lg.var.buffer_count = 0;
					} else {
						lg.var.buffer_count += 1;
					}
					let data: itron::dataqueue::DataElement = c.into();
					lg.c_dataqueue.send_force(&data);
				}
				Err(_) => {
					// No data
				}
			}
		}
	}
}

