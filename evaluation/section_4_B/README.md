# Section IV-B: EFFECTIVENESS OF OPTIMIZATION IN EXECUTION TIME
This directory contains the samples used in Section IV-B of the paper.

### tecs_can_sample
- A sample that measures the execution time of CAN `send` and `receive`.
	- You can measure by adding `send` or `receive` to the `default` in `features` of `tecs_can_sample/tecsrustCAN/Cargo.toml`.
- To enable optimization of mutual exclusion, set `ENABLE_TECS_RUST_OPT` to `1` in the `Makefile`.
    - After optimization, if you want to apply semaphores, uncomment the `CanTask2` cell at the bottom of `tecsrustCAN.cdl`.

### tecs_led_sample
- A sample that measures the execution time of LED `light_on` and `light_off`.
	- You can measure by adding `light_on` or `light_off` to the `default` in `features` of `tecs_can_sample/tecsrustCAN/Cargo.toml`.
- To enable optimization of mutual exclusion, set `ENABLE_TECS_RUST_OPT` to `1` in the `Makefile`.
    - After optimization, if you want to apply semaphores, uncomment the `LedTask2` cell at the bottom of `tecsrustLED.cdl`.

### tecs_uart_sample
- A sample that measures the execution time of UART `put_char`.
- To enable optimization of mutual exclusion, set `ENABLE_TECS_RUST_OPT` to `1` in the `Makefile`.
    - After optimization, if you want to apply semaphores, uncomment the `Task2` cell and the `TaskBody` cell at the bottom of `tecsrustCAN.cdl`.