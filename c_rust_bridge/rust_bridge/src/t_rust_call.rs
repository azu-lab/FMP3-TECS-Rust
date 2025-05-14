use crate::{s_check::*, t_centry::*, t_rust_entry::*};

pub struct TRustCall<'a>{
	c_entry: &'a (dyn SCheck + Sync + Send),
	id: u32,
}

pub struct ERustCallForTRustCall<'a>{
	pub cell: &'a TRustCall<'a>,
}

pub struct LockGuardForTRustCall<'a>{
	pub c_entry: &'a (dyn SCheck + Sync + Send),
	pub id: &'a u32,
}

#[link_section = ".rodata"]
pub static RUSTCALL: TRustCall = TRustCall {
	c_entry: &ECENTRYFORCENTRY,
	id: 0,
};

#[link_section = ".rodata"]
pub static ERUSTCALLFORRUSTCALL: ERustCallForTRustCall = ERustCallForTRustCall {
	cell: &RUSTCALL,
};

#[link_section = ".rodata"]
pub static RUSTCALL2: TRustCall = TRustCall {
	c_entry: &ERUSTENTRYFORRUSTENTRY,
	id: 0,
};

#[link_section = ".rodata"]
pub static ERUSTCALLFORRUSTCALL2: ERustCallForTRustCall = ERustCallForTRustCall {
	cell: &RUSTCALL2,
};

impl<> TRustCall<'_> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTRustCall<'_> {
		LockGuardForTRustCall {
			c_entry: self.c_entry,
			id: &self.id,
		}
	}
}
