# Section IV-F: COMPARISON WITH R2R, THE RUST BINDING FOR ROS2
This directory contains the samples used in Section IV-F of the paper.

### r2r_sample
- A sample that measures on WSL2 the execution time of `imu_corrector` and `vehicle_velocity_converter` implemented with R2R.
	- Move to the `r2r_sample` directory and run the following commands:
    ```
    cargo build --release --bin imu_corrector
    taskset -c 0 ./target/release/imu_corrector
    ```
    ```
    cargo build --release --bin vehicle_velocity_converter
    taskset -c 0 ./target/release/vehicle_velocity_converter
    ```

### tecs_sample
- A sample that measures on WSL2 the execution time of `imu_corrector` and `vehicle_velocity_converter` implemented with the proposed framework's data structures.
	- Move to the `tecs_sample/tecs_rust` directory and run the following commands:
    ```
    cargo build --release --bin imu_corrector
    taskset -c 0 ./target/release/imu_corrector
    ```
    ```
    cargo build --release --bin vehicle_velocity_converter
    taskset -c 0 ./target/release/vehicle_velocity_converter
    ```