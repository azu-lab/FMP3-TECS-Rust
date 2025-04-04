use crate::{t_taskbody::*, s_task_body::*, s_xuart_measure::*, s_dataqueue_rs::*};
use crate::print;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::task::*;


impl STaskBody for ETaskbodyForTTaskbody<'_>{

	fn main(&'static self) {
		let (c_xuart, c_dataqueue) = self.cell.get_cell_ref();

		delay(duration!(s: 1)).expect("delay failed");

		loop{
			let mut data = c_dataqueue.receive();
			match data {
				Ok(data) => {
					c_xuart.put_char(&(data as u8));
				}
				Err(e) => {
					c_xuart.put_char(&b'E');
				}
			}
		}
	}
}

