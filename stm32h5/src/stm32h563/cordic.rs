#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CORDIC control/status register"]
    pub cordic_csr: CORDIC_CSR,
    #[doc = "0x04 - CORDIC argument register"]
    pub cordic_wdata: CORDIC_WDATA,
    #[doc = "0x08 - CORDIC result register"]
    pub cordic_rdata: CORDIC_RDATA,
}
#[doc = "CORDIC_CSR (rw) register accessor: an alias for `Reg<CORDIC_CSR_SPEC>`"]
pub type CORDIC_CSR = crate::Reg<cordic_csr::CORDIC_CSR_SPEC>;
#[doc = "CORDIC control/status register"]
pub mod cordic_csr;
#[doc = "CORDIC_WDATA (w) register accessor: an alias for `Reg<CORDIC_WDATA_SPEC>`"]
pub type CORDIC_WDATA = crate::Reg<cordic_wdata::CORDIC_WDATA_SPEC>;
#[doc = "CORDIC argument register"]
pub mod cordic_wdata;
#[doc = "CORDIC_RDATA (r) register accessor: an alias for `Reg<CORDIC_RDATA_SPEC>`"]
pub type CORDIC_RDATA = crate::Reg<cordic_rdata::CORDIC_RDATA_SPEC>;
#[doc = "CORDIC result register"]
pub mod cordic_rdata;
