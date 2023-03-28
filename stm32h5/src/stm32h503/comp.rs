#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    pub comp_sr: COMP_SR,
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    pub comp_icfr: COMP_ICFR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Comparator configuration register 1"]
    pub comp_cfgr1: COMP_CFGR1,
    #[doc = "0x10 - Comparator configuration register 2"]
    pub comp_cfgr2: COMP_CFGR2,
}
#[doc = "COMP_SR (r) register accessor: an alias for `Reg<COMP_SR_SPEC>`"]
pub type COMP_SR = crate::Reg<comp_sr::COMP_SR_SPEC>;
#[doc = "Comparator status register"]
pub mod comp_sr;
#[doc = "COMP_ICFR (rw) register accessor: an alias for `Reg<COMP_ICFR_SPEC>`"]
pub type COMP_ICFR = crate::Reg<comp_icfr::COMP_ICFR_SPEC>;
#[doc = "Comparator interrupt clear flag register"]
pub mod comp_icfr;
#[doc = "COMP_CFGR1 (rw) register accessor: an alias for `Reg<COMP_CFGR1_SPEC>`"]
pub type COMP_CFGR1 = crate::Reg<comp_cfgr1::COMP_CFGR1_SPEC>;
#[doc = "Comparator configuration register 1"]
pub mod comp_cfgr1;
#[doc = "COMP_CFGR2 (rw) register accessor: an alias for `Reg<COMP_CFGR2_SPEC>`"]
pub type COMP_CFGR2 = crate::Reg<comp_cfgr2::COMP_CFGR2_SPEC>;
#[doc = "Comparator configuration register 2"]
pub mod comp_cfgr2;
