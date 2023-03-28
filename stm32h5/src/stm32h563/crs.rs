#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRS control register"]
    pub crs_cr: CRS_CR,
    #[doc = "0x04 - CRS configuration register"]
    pub crs_cfgr: CRS_CFGR,
    #[doc = "0x08 - CRS interrupt and status register"]
    pub crs_isr: CRS_ISR,
    #[doc = "0x0c - CRS interrupt flag clear register"]
    pub crs_icr: CRS_ICR,
}
#[doc = "CRS_CR (rw) register accessor: an alias for `Reg<CRS_CR_SPEC>`"]
pub type CRS_CR = crate::Reg<crs_cr::CRS_CR_SPEC>;
#[doc = "CRS control register"]
pub mod crs_cr;
#[doc = "CRS_CFGR (rw) register accessor: an alias for `Reg<CRS_CFGR_SPEC>`"]
pub type CRS_CFGR = crate::Reg<crs_cfgr::CRS_CFGR_SPEC>;
#[doc = "CRS configuration register"]
pub mod crs_cfgr;
#[doc = "CRS_ISR (r) register accessor: an alias for `Reg<CRS_ISR_SPEC>`"]
pub type CRS_ISR = crate::Reg<crs_isr::CRS_ISR_SPEC>;
#[doc = "CRS interrupt and status register"]
pub mod crs_isr;
#[doc = "CRS_ICR (rw) register accessor: an alias for `Reg<CRS_ICR_SPEC>`"]
pub type CRS_ICR = crate::Reg<crs_icr::CRS_ICR_SPEC>;
#[doc = "CRS interrupt flag clear register"]
pub mod crs_icr;
