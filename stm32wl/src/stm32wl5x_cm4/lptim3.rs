#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x08 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x0c - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - control register"]
    pub cr: CR,
    #[doc = "0x14 - compare register"]
    pub cmp: CMP,
    #[doc = "0x18 - autoreload register"]
    pub arr: ARR,
    #[doc = "0x1c - counter register"]
    pub cnt: CNT,
    #[doc = "0x20 - option register"]
    pub or: OR,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - repetition register"]
    pub rcr: RCR,
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "compare register"]
pub mod cmp;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "autoreload register"]
pub mod arr;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter register"]
pub mod cnt;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "option register"]
pub mod or;
#[doc = "RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "repetition register"]
pub mod rcr;
