#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF_BUF Control and Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - VREF_BUF Calibration Control Register"]
    pub ccr: CCR,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "VREF_BUF Control and Status Register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "VREF_BUF Calibration Control Register"]
pub mod ccr;
