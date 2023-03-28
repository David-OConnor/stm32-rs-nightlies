#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM7 control register 1"]
    pub tim7_cr1: TIM7_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM7 control register 2"]
    pub tim7_cr2: TIM7_CR2,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - TIM7 DMA/Interrupt enable register"]
    pub tim7_dier: TIM7_DIER,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - TIM7 status register"]
    pub tim7_sr: TIM7_SR,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - TIM7 event generation register"]
    pub tim7_egr: TIM7_EGR,
    _reserved5: [u8; 0x0e],
    #[doc = "0x24 - TIM7 counter"]
    pub tim7_cnt: TIM7_CNT,
    #[doc = "0x28 - TIM7 prescaler"]
    pub tim7_psc: TIM7_PSC,
    _reserved7: [u8; 0x02],
    #[doc = "0x2c - TIM7 auto-reload register"]
    pub tim7_arr: TIM7_ARR,
}
#[doc = "TIM7_CR1 (rw) register accessor: an alias for `Reg<TIM7_CR1_SPEC>`"]
pub type TIM7_CR1 = crate::Reg<tim7_cr1::TIM7_CR1_SPEC>;
#[doc = "TIM7 control register 1"]
pub mod tim7_cr1;
#[doc = "TIM7_CR2 (rw) register accessor: an alias for `Reg<TIM7_CR2_SPEC>`"]
pub type TIM7_CR2 = crate::Reg<tim7_cr2::TIM7_CR2_SPEC>;
#[doc = "TIM7 control register 2"]
pub mod tim7_cr2;
#[doc = "TIM7_DIER (rw) register accessor: an alias for `Reg<TIM7_DIER_SPEC>`"]
pub type TIM7_DIER = crate::Reg<tim7_dier::TIM7_DIER_SPEC>;
#[doc = "TIM7 DMA/Interrupt enable register"]
pub mod tim7_dier;
#[doc = "TIM7_SR (rw) register accessor: an alias for `Reg<TIM7_SR_SPEC>`"]
pub type TIM7_SR = crate::Reg<tim7_sr::TIM7_SR_SPEC>;
#[doc = "TIM7 status register"]
pub mod tim7_sr;
#[doc = "TIM7_EGR (w) register accessor: an alias for `Reg<TIM7_EGR_SPEC>`"]
pub type TIM7_EGR = crate::Reg<tim7_egr::TIM7_EGR_SPEC>;
#[doc = "TIM7 event generation register"]
pub mod tim7_egr;
#[doc = "TIM7_CNT (rw) register accessor: an alias for `Reg<TIM7_CNT_SPEC>`"]
pub type TIM7_CNT = crate::Reg<tim7_cnt::TIM7_CNT_SPEC>;
#[doc = "TIM7 counter"]
pub mod tim7_cnt;
#[doc = "TIM7_PSC (rw) register accessor: an alias for `Reg<TIM7_PSC_SPEC>`"]
pub type TIM7_PSC = crate::Reg<tim7_psc::TIM7_PSC_SPEC>;
#[doc = "TIM7 prescaler"]
pub mod tim7_psc;
#[doc = "TIM7_ARR (rw) register accessor: an alias for `Reg<TIM7_ARR_SPEC>`"]
pub type TIM7_ARR = crate::Reg<tim7_arr::TIM7_ARR_SPEC>;
#[doc = "TIM7 auto-reload register"]
pub mod tim7_arr;
