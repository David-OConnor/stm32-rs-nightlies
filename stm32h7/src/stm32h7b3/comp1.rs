#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    pub sr: SR,
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    pub icfr: ICFR,
    #[doc = "0x08 - Comparator option register"]
    pub or: OR,
    #[doc = "0x0c - Comparator configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x10 - Comparator configuration register 2"]
    pub cfgr2: CFGR2,
}
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Comparator status register"]
pub mod sr;
#[doc = "ICFR (w) register accessor: an alias for `Reg<ICFR_SPEC>`"]
pub type ICFR = crate::Reg<icfr::ICFR_SPEC>;
#[doc = "Comparator interrupt clear flag register"]
pub mod icfr;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "Comparator option register"]
pub mod or;
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "Comparator configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Comparator configuration register 2"]
pub mod cfgr2;
