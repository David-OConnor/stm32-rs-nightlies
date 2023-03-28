#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA control register"]
    pub pka_cr: PKA_CR,
    #[doc = "0x04 - PKA status register"]
    pub pka_sr: PKA_SR,
    #[doc = "0x08 - PKA clear flag register"]
    pub pka_clrfr: PKA_CLRFR,
}
#[doc = "PKA_CR (rw) register accessor: an alias for `Reg<PKA_CR_SPEC>`"]
pub type PKA_CR = crate::Reg<pka_cr::PKA_CR_SPEC>;
#[doc = "PKA control register"]
pub mod pka_cr;
#[doc = "PKA_SR (r) register accessor: an alias for `Reg<PKA_SR_SPEC>`"]
pub type PKA_SR = crate::Reg<pka_sr::PKA_SR_SPEC>;
#[doc = "PKA status register"]
pub mod pka_sr;
#[doc = "PKA_CLRFR (w) register accessor: an alias for `Reg<PKA_CLRFR_SPEC>`"]
pub type PKA_CLRFR = crate::Reg<pka_clrfr::PKA_CLRFR_SPEC>;
#[doc = "PKA clear flag register"]
pub mod pka_clrfr;
