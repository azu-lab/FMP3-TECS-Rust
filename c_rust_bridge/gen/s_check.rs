pub trait SCheck {
	fn add(&'static self, i: &u32, j: &u32)-> u32;
	fn sub(&'static self, i: &i32, j: &i32, result: &mut i32);
	fn sum(&'static self, array: &[i32; 8])-> i32;
	fn copy(&'static self, input: &[i32; 8], output: &mut [i32; 8]);
}
