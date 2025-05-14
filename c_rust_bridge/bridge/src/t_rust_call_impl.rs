use crate::{t_rust_call::*, s_check::*};

impl SCheck for ERustCallForTRustCall<'_>{

	fn add(&'static self, i: &u32, j: &u32) -> u32{
		let lg = self.cell.get_cell_ref();

	}
	fn sub(&'static self, i: &i32, j: &i32, result: &mut i32) {
		let lg = self.cell.get_cell_ref();

	}
	fn sum(&'static self, array: &[i32; 8]) -> i32{
		let lg = self.cell.get_cell_ref();

	}
	fn copy(&'static self, input: &[i32; 8], output: &mut [i32; 8]) {
		let lg = self.cell.get_cell_ref();

	}
}

