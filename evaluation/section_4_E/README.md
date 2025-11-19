# Section IV-E: COMPARISON OF EXECUTION TIMES ON THE RTOS
This directory contains the samples used in Section IV-E of the paper.

### rust_api_dispatch
- A sample that measures the execution time of Rust task APIs `act_tsk`, `chg_pri`, and `mig_tsk`.
	- You can measure by adding `act_tsk` or `chg_pri` or `mig_tsk` to the `default` in `features` of `Cargo.toml`.
	- At the same time, also update the contents of `rustApiDispatch.cfg`.
		- For `act_tsk`, copy the contents of `rust_api_dispatch/act_tsk/rustApiDispatch.cfg` to `rust_api_dispatch/rustApiDispatch.cfg`.

### rust_api_measure
- A sample that measures the execution time of the Rust task API `get_pri`.
	- You can measure by adding `get_pri` to the `default` in `features` of `Cargo.toml`.

### tecs_api_dispatch
- A sample that measures the execution time of the proposed framework's task APIs `act_tsk`, `chg_pri`, and `mig_tsk`.
	- You can measure by adding `act_tsk` or `chg_pri` or `mig_tsk` to the `default` in `features` of `Cargo.toml`.
	- At the same time, also update the contents of `tr_api_dispatch.cdl`.
		- For `act_tsk`, copy the contents of `tecs_api_dispatch/act_tsk/tr_api_dispatch.cdl` to `tecs_api_dispatch/tr_api_dispatch.cdl`.

### tecs_api_measure
- A sample that measures the execution time of the proposed framework's task API `get_pri`.
	- You can measure by adding `get_pri` to the `default` in `features` of `Cargo.toml`.