pub struct TTaskbodyRs2
{
}

pub struct ETaskbodyForTTaskbodyRs2<'a>{
	pub cell: &'a TTaskbodyRs2,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_TASKBODY1_2: TTaskbodyRs2 = TTaskbodyRs2 {
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY1_2: ETaskbodyForTTaskbodyRs2 = ETaskbodyForTTaskbodyRs2 {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY1_2,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY2_2: TTaskbodyRs2 = TTaskbodyRs2 {
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY2_2: ETaskbodyForTTaskbodyRs2 = ETaskbodyForTTaskbodyRs2 {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY2_2,
};

