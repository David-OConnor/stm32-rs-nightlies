#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM14 control register 1"]
    pub tim14_cr1: TIM14_CR1,
    _reserved1: [u8; 0x0a],
    #[doc = "0x0c - TIM14 Interrupt enable register"]
    pub tim14_dier: TIM14_DIER,
    _reserved2: [u8; 0x02],
    #[doc = "0x10 - TIM14 status register"]
    pub tim14_sr: TIM14_SR,
    _reserved3: [u8; 0x02],
    #[doc = "0x14 - TIM14 event generation register"]
    pub tim14_egr: TIM14_EGR,
    _reserved4: [u8; 0x02],
    #[doc = "0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
    pub tim14_ccmr1: TIM14_CCMR1,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - TIM14 capture/compare enable register"]
    pub tim14_ccer: TIM14_CCER,
    _reserved6: [u8; 0x02],
    #[doc = "0x24 - TIM14 counter"]
    pub tim14_cnt: TIM14_CNT,
    #[doc = "0x28 - TIM14 prescaler"]
    pub tim14_psc: TIM14_PSC,
    _reserved8: [u8; 0x02],
    #[doc = "0x2c - TIM14 auto-reload register"]
    pub tim14_arr: TIM14_ARR,
    _reserved9: [u8; 0x06],
    #[doc = "0x34 - TIM14 capture/compare register 1"]
    pub tim14_ccr1: TIM14_CCR1,
    _reserved10: [u8; 0x32],
    #[doc = "0x68 - TIM14 timer input selection register"]
    pub tim14_tisel: TIM14_TISEL,
}
#[doc = "TIM14_CR1 (rw) register accessor: an alias for `Reg<TIM14_CR1_SPEC>`"]
pub type TIM14_CR1 = crate::Reg<tim14_cr1::TIM14_CR1_SPEC>;
#[doc = "TIM14 control register 1"]
pub mod tim14_cr1;
#[doc = "TIM14_DIER (rw) register accessor: an alias for `Reg<TIM14_DIER_SPEC>`"]
pub type TIM14_DIER = crate::Reg<tim14_dier::TIM14_DIER_SPEC>;
#[doc = "TIM14 Interrupt enable register"]
pub mod tim14_dier;
#[doc = "TIM14_SR (rw) register accessor: an alias for `Reg<TIM14_SR_SPEC>`"]
pub type TIM14_SR = crate::Reg<tim14_sr::TIM14_SR_SPEC>;
#[doc = "TIM14 status register"]
pub mod tim14_sr;
#[doc = "TIM14_EGR (w) register accessor: an alias for `Reg<TIM14_EGR_SPEC>`"]
pub type TIM14_EGR = crate::Reg<tim14_egr::TIM14_EGR_SPEC>;
#[doc = "TIM14 event generation register"]
pub mod tim14_egr;
#[doc = "TIM14_CCMR1 (rw) register accessor: an alias for `Reg<TIM14_CCMR1_SPEC>`"]
pub type TIM14_CCMR1 = crate::Reg<tim14_ccmr1::TIM14_CCMR1_SPEC>;
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
pub mod tim14_ccmr1;
#[doc = "TIM14_CCER (rw) register accessor: an alias for `Reg<TIM14_CCER_SPEC>`"]
pub type TIM14_CCER = crate::Reg<tim14_ccer::TIM14_CCER_SPEC>;
#[doc = "TIM14 capture/compare enable register"]
pub mod tim14_ccer;
#[doc = "TIM14_CNT (rw) register accessor: an alias for `Reg<TIM14_CNT_SPEC>`"]
pub type TIM14_CNT = crate::Reg<tim14_cnt::TIM14_CNT_SPEC>;
#[doc = "TIM14 counter"]
pub mod tim14_cnt;
#[doc = "TIM14_PSC (rw) register accessor: an alias for `Reg<TIM14_PSC_SPEC>`"]
pub type TIM14_PSC = crate::Reg<tim14_psc::TIM14_PSC_SPEC>;
#[doc = "TIM14 prescaler"]
pub mod tim14_psc;
#[doc = "TIM14_ARR (rw) register accessor: an alias for `Reg<TIM14_ARR_SPEC>`"]
pub type TIM14_ARR = crate::Reg<tim14_arr::TIM14_ARR_SPEC>;
#[doc = "TIM14 auto-reload register"]
pub mod tim14_arr;
#[doc = "TIM14_CCR1 (rw) register accessor: an alias for `Reg<TIM14_CCR1_SPEC>`"]
pub type TIM14_CCR1 = crate::Reg<tim14_ccr1::TIM14_CCR1_SPEC>;
#[doc = "TIM14 capture/compare register 1"]
pub mod tim14_ccr1;
#[doc = "TIM14_TISEL (rw) register accessor: an alias for `Reg<TIM14_TISEL_SPEC>`"]
pub type TIM14_TISEL = crate::Reg<tim14_tisel::TIM14_TISEL_SPEC>;
#[doc = "TIM14 timer input selection register"]
pub mod tim14_tisel;
