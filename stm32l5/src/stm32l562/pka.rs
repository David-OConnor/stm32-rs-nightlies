#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA control register"]
    pub cr: CR,
    #[doc = "0x04 - PKA status register"]
    pub sr: SR,
    #[doc = "0x08 - PKA clear flag register"]
    pub clrfr: CLRFR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "PKA control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PKA status register"]
pub mod sr;
#[doc = "CLRFR (w) register accessor: an alias for `Reg<CLRFR_SPEC>`"]
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
#[doc = "PKA clear flag register"]
pub mod clrfr;
