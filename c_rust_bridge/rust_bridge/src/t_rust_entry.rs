pub struct TRustEntry<'a>{
	id: u32,
}

pub struct ERustEntryForTRustEntry<'a>{
	pub cell: &'a TRustEntry<'a>,
}

pub struct LockGuardForTRustEntry<'a>{
	pub id: &'a u32,
}

#[link_section = ".rodata"]
pub static RUSTENTRY: TRustEntry = TRustEntry {
	id: 1,
};

#[link_section = ".rodata"]
pub static ERUSTENTRYFORRUSTENTRY: ERustEntryForTRustEntry = ERustEntryForTRustEntry {
	cell: &RUSTENTRY,
};

impl<> TRustEntry<'_> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTRustEntry<'_> {
		LockGuardForTRustEntry {
			id: &self.id,
		}
	}
}
