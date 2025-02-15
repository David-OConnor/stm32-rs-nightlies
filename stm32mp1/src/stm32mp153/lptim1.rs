#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPTIM interrupt and status register"]
    pub lptim_isr: LPTIM_ISR,
    #[doc = "0x04 - LPTIM interrupt clear register"]
    pub lptim_icr: LPTIM_ICR,
    #[doc = "0x08 - LPTIM interrupt enable register"]
    pub lptim_ier: LPTIM_IER,
    #[doc = "0x0c - LPTIM configuration register"]
    pub lptim_cfgr: LPTIM_CFGR,
    #[doc = "0x10 - LPTIM control register"]
    pub lptim_cr: LPTIM_CR,
    #[doc = "0x14 - LPTIM compare register"]
    pub lptim_cmp: LPTIM_CMP,
    #[doc = "0x18 - LPTIM autoreload register"]
    pub lptim_arr: LPTIM_ARR,
    #[doc = "0x1c - LPTIM counter register"]
    pub lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - LPTIM configuration register 2"]
    pub lptim_cfgr2: LPTIM_CFGR2,
    _reserved9: [u8; 0x03c8],
    #[doc = "0x3f0 - LPTIM 1 peripheral hardware configuration register"]
    pub lptim1_hwcfgr: LPTIM1_HWCFGR,
    #[doc = "0x3f4 - LPTIM peripheral version identification register"]
    pub lptim_verr: LPTIM_VERR,
    #[doc = "0x3f8 - LPTIM peripheral type identification register"]
    pub lptim_pidr: LPTIM_PIDR,
    #[doc = "0x3fc - LPTIM registers map size identification register"]
    pub lptim_sidr: LPTIM_SIDR,
}
#[doc = "LPTIM_ISR (r) register accessor: an alias for `Reg<LPTIM_ISR_SPEC>`"]
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISR_SPEC>;
#[doc = "LPTIM interrupt and status register"]
pub mod lptim_isr;
#[doc = "LPTIM_ICR (w) register accessor: an alias for `Reg<LPTIM_ICR_SPEC>`"]
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICR_SPEC>;
#[doc = "LPTIM interrupt clear register"]
pub mod lptim_icr;
#[doc = "LPTIM_IER (rw) register accessor: an alias for `Reg<LPTIM_IER_SPEC>`"]
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IER_SPEC>;
#[doc = "LPTIM interrupt enable register"]
pub mod lptim_ier;
#[doc = "LPTIM_CFGR (rw) register accessor: an alias for `Reg<LPTIM_CFGR_SPEC>`"]
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGR_SPEC>;
#[doc = "LPTIM configuration register"]
pub mod lptim_cfgr;
#[doc = "LPTIM_CR (rw) register accessor: an alias for `Reg<LPTIM_CR_SPEC>`"]
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CR_SPEC>;
#[doc = "LPTIM control register"]
pub mod lptim_cr;
#[doc = "LPTIM_CMP (rw) register accessor: an alias for `Reg<LPTIM_CMP_SPEC>`"]
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMP_SPEC>;
#[doc = "LPTIM compare register"]
pub mod lptim_cmp;
#[doc = "LPTIM_ARR (rw) register accessor: an alias for `Reg<LPTIM_ARR_SPEC>`"]
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARR_SPEC>;
#[doc = "LPTIM autoreload register"]
pub mod lptim_arr;
#[doc = "LPTIM_CNT (r) register accessor: an alias for `Reg<LPTIM_CNT_SPEC>`"]
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNT_SPEC>;
#[doc = "LPTIM counter register"]
pub mod lptim_cnt;
#[doc = "LPTIM_CFGR2 (rw) register accessor: an alias for `Reg<LPTIM_CFGR2_SPEC>`"]
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2_SPEC>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
#[doc = "LPTIM1_HWCFGR (r) register accessor: an alias for `Reg<LPTIM1_HWCFGR_SPEC>`"]
pub type LPTIM1_HWCFGR = crate::Reg<lptim1_hwcfgr::LPTIM1_HWCFGR_SPEC>;
#[doc = "LPTIM 1 peripheral hardware configuration register"]
pub mod lptim1_hwcfgr;
#[doc = "LPTIM_VERR (r) register accessor: an alias for `Reg<LPTIM_VERR_SPEC>`"]
pub type LPTIM_VERR = crate::Reg<lptim_verr::LPTIM_VERR_SPEC>;
#[doc = "LPTIM peripheral version identification register"]
pub mod lptim_verr;
#[doc = "LPTIM_PIDR (r) register accessor: an alias for `Reg<LPTIM_PIDR_SPEC>`"]
pub type LPTIM_PIDR = crate::Reg<lptim_pidr::LPTIM_PIDR_SPEC>;
#[doc = "LPTIM peripheral type identification register"]
pub mod lptim_pidr;
#[doc = "LPTIM_SIDR (r) register accessor: an alias for `Reg<LPTIM_SIDR_SPEC>`"]
pub type LPTIM_SIDR = crate::Reg<lptim_sidr::LPTIM_SIDR_SPEC>;
#[doc = "LPTIM registers map size identification register"]
pub mod lptim_sidr;
