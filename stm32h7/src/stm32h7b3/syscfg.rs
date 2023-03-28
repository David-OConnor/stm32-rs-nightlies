#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmcr: PMCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - SYSCFG timer break lockup register"]
    pub cfgr: CFGR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - compensation cell control/status register"]
    pub cccsr: CCCSR,
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    pub ccvr: CCVR,
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    pub cccr: CCCR,
}
#[doc = "PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CCCSR (rw) register accessor: an alias for `Reg<CCCSR_SPEC>`"]
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR (r) register accessor: an alias for `Reg<CCVR_SPEC>`"]
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "SYSCFG timer break lockup register"]
pub mod cfgr;
