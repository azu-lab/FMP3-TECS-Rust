# Section IV-C: EFFECTIVENESS OF OPTIMIZATION IN PROGRAM SIZE
This directory contains the samples used in Section IV-C of the paper.

### rust_sample
- A sample for measuring Rust binary size.
- The `ex*` directories already contain the code used for each evaluation and the built binaries.
    - To build the sample where one mutual exclusion is eliminated, copy the following files from the `ex1` directory:
        - Copy the contents of `rust_sample/ex1/lib.rs` to `rust_sample/rust_opt/src/lib.rs`.
        - Copy the contents of `rust_sample/ex1/rust_opt.cfg` to `rust_sample/rust_opt.cfg`.
        - Copy the contents of `rust_sample/ex1/rust_opt.h` to `rust_sample/rust_opt.h`.

### tecs_sample
- A sample for measuring the binary size of the proposed framework.
- To enable optimization of mutual exclusion, set `ENABLE_TECS_RUST_OPT` to `1` in the `Makefile`.
- To vary the number of mutual exclusions eliminated by the optimization, you will need to edit the code.
    - The `ex*_no_opt` and `ex*_opt` directories already contain the code used for each evaluation and the built binaries.
    - To build the sample where one mutual exclusion is eliminated, copy the following file from the `ex1` directory:
        - Copy the contents of `tecs_sample/ex1_opt/tecsrust_opt.cdl` to `tecs_sample/tecsrust_opt.cdl`.
