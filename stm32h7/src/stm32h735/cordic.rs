#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - Argument register"]
    pub wdata: WDATA,
    #[doc = "0x08 - Result register"]
    pub rdata: RDATA,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "Argument register"]
pub mod wdata;
#[doc = "RDATA (rw) register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Result register"]
pub mod rdata;
