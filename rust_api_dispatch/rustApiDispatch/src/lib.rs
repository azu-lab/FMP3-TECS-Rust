#![no_std]
#![feature(const_option)]

mod tecs_print;
mod kernel_cfg;

use crate::tecs_print::*;
use core::num::NonZeroI32;
use itron::task::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;
use kernel_cfg::*;
use itron::abi::*;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::task::State::*;

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

const N: u32 = 1001;

pub static mut OVERHEAD: HrtCnt = 0;
pub static mut START: HrtCnt = 0;
pub static mut END: HrtCnt = 0;

static c_task :TaskRef = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_2).unwrap())};
static c_taskmig :TaskRef = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_MIG).unwrap())};
static c_semaphore :SemaphoreRef = unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())};

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__task1_1(_: usize) {
    print!("Processor1: Rust chg_pri,", );
    delay(duration!(ms: 1000)).expect("delay failed");

    let mut dispatch_time :HrtCnt = 0;
    let mut dispatch_end :HrtCnt = 0;
    let mut overhead :HrtCnt = 0;

    unsafe{ dispatch_time = fch_hrt();}
    for i in 0..N {
        unsafe{ dispatch_end = fch_hrt();}
    }

    overhead = (dispatch_end - dispatch_time) / N;

    unsafe{
        OVERHEAD = overhead;
    }

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

    let mut can_result :Result<usize, Error<CancelActivateAllError>> = Ok(0);

    delay(duration!(ms: 1000)).expect("delay failed");


    for i in 0..N{
        let set_priority :Priority = 5;
        let default_priority :Priority = 10;
        let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());
        let processor2 = Processor::from_raw_nonnull(NonZeroI32::new(2).unwrap());

        /* c_task.act_tsk(); ↓ TASK2_2(attr: TA_NULL, pri: 5) */
        // while c_task.info().unwrap().state() != Dormant {
        // 	delay(duration!(ms: 5)).expect("delay failed");
        // }

        /* c_task.change_priority(&set_priority); ↓ TASK2_2(attr: TA_ACT, pri: 10) */
        {
            while (c_task.info().unwrap().state() == Running || c_task.priority().unwrap() != default_priority){
                delay(duration!(ms: 5)).expect("delay failed");
            }
            c_task.set_priority(default_priority);
            c_task.activate();
        }

        /* c_taskmig.migrate(&processor2); ↓ TASK_MIG(attr: TA_NULL, pri: 6) */
        // {
        // 	while c_taskmig.info().unwrap().state() != Dormant {
        // 		delay(duration!(ms: 5)).expect("delay failed");
        // 	}
        // 	c_taskmig.activate();
        // }

        // wait_result = c_semaphore.wait();
        // print!("Processor1: act_tsk",);

        // let refer = c_task.info();
        // match refer {
        // 	Ok(info) => {
        // 		match info.state() {
        // 			Running => {
        // 				print!("Running", );
        // 			},
        // 			Ready => {
        // 				print!("Ready", );
        // 			},
        // 			Waiting => {
        // 				print!("Waiting", );
        // 			},
        // 			Suspended => {
        // 				print!("Suspended", );
        // 			},
        // 			WaitingSuspended => {
        // 				print!("WaitingSuspended", );
        // 			},
        // 			Dormant => {
        // 				print!("Dormant", );
        // 			},
        // 		}
        // 	},
        // 	Err(_) => {
        // 		print!("info error", );
        // 	},
        // }

        unsafe{ 
            // _ = loc_cpu();
            START = fch_hrt();
        }

        // act_result = c_task.activate();
        // acto_result = c_task.migrate_and_activate(processor2);
        // get_result = c_task.priority();
        chg_result = c_task.set_priority(set_priority);

        // wait_result = c_semaphore.wait();
        // mig_result = c_taskmig.migrate(processor2); // mig_tsk は 呼び出したタスクと同じプロセッサに割り付けられているタスクのみに適用可能
        
        // unsafe{ ter_result = c_taskmig.terminate();} // ter_tsk は 呼び出したタスクと同じプロセッサに割り付けられているタスクのみに適用可能

        // unsafe{ 
            // end = fch_hrt();
            // _ = unl_cpu();
        // }

        // duration = end - start - overhead;
        // print!("%tu,", duration );

        // let refer = c_task.info();
        // match refer {
        // 	Ok(info) => {
        // 		match info.state() {
        // 			Running => {
        // 				print!("Running", );
        // 			},
        // 			Ready => {
        // 				print!("Ready", );
        // 			},
        // 			Waiting => {
        // 				print!("Waiting", );
        // 			},
        // 			Suspended => {
        // 				print!("Suspended", );
        // 			},
        // 			WaitingSuspended => {
        // 				print!("WaitingSuspended", );
        // 			},
        // 			Dormant => {
        // 				print!("Dormant", );
        // 			},
        // 		}
        // 	},
        // 	Err(_) => {
        // 		print!("info error", );
        // 	},
        // }

        // c_task.activate() ↓
        // sig_result = c_semaphore.signal();
        // match act_result {
        // 	Ok(_) => {
        // 		print!("activation success",);
        // 	},
        // 	Err(error) => {
        // 		match error {
        // 			BadContext => {
        // 				print!("BadContext", );
        // 			},
        // 			BadId => {
        // 				print!("BadId", );
        // 			},
        // 			BadState => {
        // 				print!("BadState", );
        // 			},
        // 			AccessDenied => {
        // 				print!("AccessDenied", );
        // 			},
        // 		}
        // 	},
        // }

        // c_task.get_priority(); ↓
        // sig_result = c_semaphore.signal();
        // match get_result {
        // 	Ok(pri) => {
        // 		print!("get_pri succcess %tu", pri);
        // 	},
        // 	Err(_) => {
        // 		print!("get_pri error", );
        // 	},
        // }

        // c_task.migrate_and_activate(&processor2) ↓
        // unsafe{ c_task.terminate(); }
        // c_task.migrate(&processor1);

        // c_task.change_priority(set_priority) ↓
        // sig_result = c_semaphore.signal();
        // get_result = c_task.priority();
        // match get_result {
        // 	Ok(pri) => {
        // 		print!("get_pri succcess %tu", pri);
        // 	},
        // 	Err(_) => {
        // 		print!("get_pri error", );
        // 	},
        // }
        // match chg_result {
        // 	Ok(_) => {
        // 		print!("chg_pri succcess", );
        // 	},
        // 	Err(_) => {
        // 		print!("chg_pri error", );
        // 	},
        // }
        // chg_result = c_task.set_priority(default_priority);
        // get_result = c_task.priority();
        // match get_result {
        // 	Ok(pri) => {
        // 		print!("get_pri succcess %tu", pri);
        // 	},
        // 	Err(_) => {
        // 		print!("get_pri error", );
        // 	},
        // }

        // c_taskmig.migrate(processor2) ↓
        // sig_result = c_semaphore.signal();
        // match mig_result {
        // 	Ok(_) => {
        // 		print!("mig_tsk succcess", );
        // 	},
        // 	Err(error) => {
        // 		match error {
        // 			BadContext => {
        // 				print!("BadContext", );
        // 			},
        // 			BadId => {
        // 				print!("BadId", );
        // 			},
        // 			AccessDenied => {
        // 				print!("AccessDenied", );
        // 			},
        // 			BadParam => {
        // 				print!("BadParam", );
        // 			},
        // 		}
        // 	},
        // }
        
        // c_taskmig.terminate() ↓
        // sig_result = c_semaphore.signal();
        // match ter_result {
        // 	Ok(_) => {
        // 		print!("ter_tsk succcess", );
        // 	},
        // 	Err(error) => {
        // 		match error {
        // 			BadContext => {
        // 				print!("BadContext", );
        // 			},
        // 			BadId => {
        // 				print!("BadId", );
        // 			},
        // 			AccessDenied => {
        // 				print!("AccessDenied", );
        // 			},
        // 			BadState => {
        // 				print!("BadState", );
        // 			},
        // 			BadParam => {
        // 				print!("BadParam", );
        // 			},
        // 		}
        // 	},
        // }
        // let refer2 = c_taskmig.info();
        // match refer2 {
        // 	Ok(info) => {
        // 		match info.state() {
        // 			Running => {
        // 				print!("Running", );
        // 			},
        // 			Ready => {
        // 				print!("Ready", );
        // 			},
        // 			Waiting => {
        // 				print!("Waiting", );
        // 			},
        // 			Suspended => {
        // 				print!("Suspended", );
        // 			},
        // 			WaitingSuspended => {
        // 				print!("WaitingSuspended", );
        // 			},
        // 			Dormant => {
        // 				print!("Dormant", );
        // 			},
        // 		}
        // 	},
        // 	Err(_) => {
        // 		print!("info error", );
        // 	},
        // }
        // act_result = c_taskmig.activate();
        // match act_result {
        // 	Ok(_) => {
        // 		print!("act_tsk succcess", );
        // 	},
        // 	Err(_) => {
        // 		print!("act_tsk error", );
        // 	},
        // }

        // delay(duration!(ms: 10)).expect("delay failed");
    }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor_all_mig__taskmig(_: usize) {
    unsafe{
        END = fch_hrt();
    }

    unsafe{
        let duration = END - START - OVERHEAD;
        print!("%tu,", duration);
    }

    let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());

    c_taskmig.migrate(processor1);
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__task2_1(_: usize) {
    loop{}
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__task2_2(_: usize) {

    unsafe{
        END = fch_hrt();
    }

    unsafe{
        let duration = END - START - OVERHEAD;
        print!("%tu,", duration);
    }
}