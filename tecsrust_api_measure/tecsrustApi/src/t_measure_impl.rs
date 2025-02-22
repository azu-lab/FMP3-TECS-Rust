use crate::{t_measure::*, s_task_rs::*, s_task_body::*, s_semaphore_rs::*};
use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;

use itron::abi::*;
use itron::task::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use itron::time::{duration, Duration, timeout, Timeout};

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

const N: u32 = 1000;

impl STaskBody for ETaskbodyForTMeasure<'_>{

	fn main(&'static self) {
		let (c_task, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

		print!("TECS/Rust dispatch: act_tsk,", );
		delay(duration!(ms: 100)).expect("delay failed");

		let mut dispatch_time :HrtCnt = 0;
		let mut dispatch_end :HrtCnt = 0;
		let mut overhead :HrtCnt = 0;

		unsafe{ dispatch_time = fch_hrt();}
		for i in 0..N {
			unsafe{ dispatch_end = fch_hrt();}
		}

		overhead = (dispatch_end - dispatch_time) / N;

		let mut start :HrtCnt = 0;
		let mut end :HrtCnt = 0;
		let mut duration :HrtCnt = 0;

		let mut act_result :Result<(), Error<ActivateError>> = Ok(());
		let mut acto_result :Result<(), Error<ActivateOnError>> = Ok(());
		let mut get_result :Result<Priority, Error<PriorityError>> = Ok(0);
		let mut chg_result :Result<(), Error<SetPriorityError>> = Ok(());
		let mut mig_result :Result<(), Error<MigrateError>> = Ok(());
		let mut ter_result :Result<(), Error<TerminateError>> = Ok(());

		let mut sig_result :Result<(), Error<SignalError>> = Ok(());
		let mut wait_result :Result<(), Error<WaitError>> = Ok(());

		let set_priority :Priority = 6;
		let default_priority :Priority = 7;

		let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());
		let processor2 = Processor::from_raw_nonnull(NonZeroI32::new(2).unwrap());

		let mut can_result :Result<usize, Error<CancelActivateAllError>> = Ok(0);


		for i in 0..N{

			unsafe{ 
				_ = loc_cpu();
				start = fch_hrt();
			}

			act_result = c_task.activate();
			// acto_result = c_task.migrate_and_activate(&processor2);
			// get_result = c_task.get_priority();
			// chg_result = c_task.change_priority(&set_priority);

			// sig_result = c_semaphore.signal();
			// mig_result = c_taskmig.migrate(&processor2); // mig_tsk は 呼び出したタスクと同じプロセッサに割り付けられているタスクのみに適用可能
			
			// ter_result = c_taskmig.terminate(); // ter_tsk は 呼び出したタスクと同じプロセッサに割り付けられているタスクのみに適用可能

			unsafe{ 
				end = fch_hrt();
				_ = unl_cpu();
			}

			duration = end - start;
			print!("%tu,", duration );

			// c_task.activate() ↓
			wait_result = c_semaphore.wait();

			// c_task.migrate_and_activate(&processor2) ↓
			// c_task.terminate();
			// c_task.migrate(&processor1);

			// c_task.change_priority(&set_priority) ↓
			// chg_result = c_task.change_priority(&default_priority);

			// c_taskmig.migrate(&processor2) ↓
			// wait_result = c_semaphore.wait();
			
			// c_taskmig.terminate() ↓
			// act_result = c_taskmig.activate();

			delay(duration!(ms: 10)).expect("delay failed");
		}
		
	}
}

