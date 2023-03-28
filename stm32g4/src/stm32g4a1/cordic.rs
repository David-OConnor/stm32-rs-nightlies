#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CORDIC Control Status register"]
    pub csr: CSR,
    #[doc = "0x04 - CORDIC argument register"]
    pub wdata: WDATA,
    #[doc = "0x08 - CORDIC result register"]
    pub rdata: RDATA,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CORDIC Control Status register"]
pub mod csr;
#[doc = "WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "CORDIC argument register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "CORDIC result register"]
pub mod rdata;
