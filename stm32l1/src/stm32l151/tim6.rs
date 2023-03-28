#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM6 control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - TIM6 control register 2"]
    pub cr2: CR2,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - TIM6 DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - TIM6 status register"]
    pub sr: SR,
    #[doc = "0x14 - TIM6 event generation register"]
    pub egr: EGR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - TIM6 counter"]
    pub cnt: CNT,
    #[doc = "0x28 - TIM6 prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - TIM6 auto-reload register"]
    pub arr: ARR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM6 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM6 control register 2"]
pub mod cr2;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM6 DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM6 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM6 event generation register"]
pub mod egr;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM6 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM6 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM6 auto-reload register"]
pub mod arr;
