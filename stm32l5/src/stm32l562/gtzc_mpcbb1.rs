#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - MPCBB control register"]
    pub lckvtr1: LCKVTR1,
    #[doc = "0x14 - MPCBB control register"]
    pub lckvtr2: LCKVTR2,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100..0x200 - MPCBBx vector register"]
    pub vctr: [VCTR; 64],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "MPCBB control register"]
pub mod cr;
#[doc = "LCKVTR1 (rw) register accessor: an alias for `Reg<LCKVTR1_SPEC>`"]
pub type LCKVTR1 = crate::Reg<lckvtr1::LCKVTR1_SPEC>;
#[doc = "MPCBB control register"]
pub mod lckvtr1;
#[doc = "LCKVTR2 (rw) register accessor: an alias for `Reg<LCKVTR2_SPEC>`"]
pub type LCKVTR2 = crate::Reg<lckvtr2::LCKVTR2_SPEC>;
#[doc = "MPCBB control register"]
pub mod lckvtr2;
#[doc = "VCTR (rw) register accessor: an alias for `Reg<VCTR_SPEC>`"]
pub type VCTR = crate::Reg<vctr::VCTR_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod vctr;
