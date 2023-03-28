#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - PKA status register"]
    pub sr: SR,
    #[doc = "0x08 - PKA clear flag register"]
    pub clrfr: CLRFR,
    _reserved3: [u8; 0x1fe8],
    #[doc = "0x1ff4 - PKA version register"]
    pub verr: VERR,
    #[doc = "0x1ff8 - PKA identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x1ffc - PKA size ID register"]
    pub sidr: SIDR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PKA status register"]
pub mod sr;
#[doc = "CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`"]
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
#[doc = "PKA clear flag register"]
pub mod clrfr;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "PKA version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "PKA identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "PKA size ID register"]
pub mod sidr;
