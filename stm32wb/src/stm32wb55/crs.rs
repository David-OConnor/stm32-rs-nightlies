#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRS control register"]
    pub cr: CR,
    #[doc = "0x04 - CRS configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - CRS interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x0c - CRS interrupt flag clear register"]
    pub icr: ICR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRS control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "CRS configuration register"]
pub mod cfgr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "CRS interrupt and status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "CRS interrupt flag clear register"]
pub mod icr;
