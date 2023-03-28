#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub dlyb_cr: DLYB_CR,
    #[doc = "0x04 - configuration register"]
    pub dlyb_cfgr: DLYB_CFGR,
}
#[doc = "DLYB_CR (rw) register accessor: an alias for `Reg<DLYB_CR_SPEC>`"]
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CR_SPEC>;
#[doc = "control register"]
pub mod dlyb_cr;
#[doc = "DLYB_CFGR (rw) register accessor: an alias for `Reg<DLYB_CFGR_SPEC>`"]
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGR_SPEC>;
#[doc = "configuration register"]
pub mod dlyb_cfgr;
