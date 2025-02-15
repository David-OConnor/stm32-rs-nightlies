#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    pub csr: CSR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - ADC common control register"]
    pub ccr: CCR,
    #[doc = "0x0c - Common regular data register for dual mode"]
    pub cdr: CDR,
    #[doc = "0x10 - Common regular data register for dual mode"]
    pub cdr2: CDR2,
}
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "CDR (r) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Common regular data register for dual mode"]
pub mod cdr;
#[doc = "CDR2 (r) register accessor: an alias for `Reg<CDR2_SPEC>`"]
pub type CDR2 = crate::Reg<cdr2::CDR2_SPEC>;
#[doc = "Common regular data register for dual mode"]
pub mod cdr2;
