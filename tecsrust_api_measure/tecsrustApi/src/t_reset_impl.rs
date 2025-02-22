use crate::{t_reset::*, s_task_rs::*, s_semaphore_rs::*, s_task_body::*};

use core::num::NonZeroI32;
use itron::task::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

impl STaskBody for EResetForTReset<'_>{

	fn main(&'static self) {
		let (c_task, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

		let mut mig_result :Result<(), Error<MigrateError>> = Ok(());
		let mut ter_result :Result<(), Error<TerminateError>> = Ok(());

		let mut sig_result :Result<(), Error<SignalError>> = Ok(());
		let mut wait_result :Result<(), Error<WaitError>> = Ok(());

		let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());

		loop{
			sig_result = c_semaphore.signal();

			// c_task.activate() ↓
			ter_result = c_task.terminate();

			// c_taskmig.migrate(&processor2) ↓
			// mig_result = c_taskmig.migrate(&processor1);

			wait_result = c_semaphore.wait();
		}

	}
}

