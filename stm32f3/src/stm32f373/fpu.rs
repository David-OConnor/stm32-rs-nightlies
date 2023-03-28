#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Floating-point context control register"]
    pub fpccr: FPCCR,
    #[doc = "0x04 - Floating-point context address register"]
    pub fpcar: FPCAR,
    #[doc = "0x08 - Floating-point status control register"]
    pub fpscr: FPSCR,
}
#[doc = "FPCCR (rw) register accessor: an alias for `Reg<FPCCR_SPEC>`"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Floating-point context control register"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: an alias for `Reg<FPCAR_SPEC>`"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Floating-point context address register"]
pub mod fpcar;
#[doc = "FPSCR (rw) register accessor: an alias for `Reg<FPSCR_SPEC>`"]
pub type FPSCR = crate::Reg<fpscr::FPSCR_SPEC>;
#[doc = "Floating-point status control register"]
pub mod fpscr;
