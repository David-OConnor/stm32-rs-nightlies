#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM16 control register 1"]
    pub tim16_cr1: TIM16_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM16 control register 2"]
    pub tim16_cr2: TIM16_CR2,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - TIM16 DMA/interrupt enable register"]
    pub tim16_dier: TIM16_DIER,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - TIM16 status register"]
    pub tim16_sr: TIM16_SR,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - TIM16 event generation register"]
    pub tim16_egr: TIM16_EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_tim16_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - TIM16 capture/compare enable register"]
    pub tim16_ccer: TIM16_CCER,
    _reserved7: [u8; 0x02],
    #[doc = "0x24 - TIM16 counter"]
    pub tim16_cnt: TIM16_CNT,
    #[doc = "0x28 - TIM16 prescaler"]
    pub tim16_psc: TIM16_PSC,
    _reserved9: [u8; 0x02],
    #[doc = "0x2c - TIM16 auto-reload register"]
    pub tim16_arr: TIM16_ARR,
    #[doc = "0x30 - TIM16 repetition counter register"]
    pub tim16_rcr: TIM16_RCR,
    _reserved11: [u8; 0x02],
    #[doc = "0x34 - TIM16 capture/compare register 1"]
    pub tim16_ccr1: TIM16_CCR1,
    _reserved12: [u8; 0x0c],
    #[doc = "0x44 - TIM16 break and dead-time register"]
    pub tim16_bdtr: TIM16_BDTR,
    _reserved13: [u8; 0x0c],
    #[doc = "0x54 - TIM16 timer deadtime register 2"]
    pub tim16_dtr2: TIM16_DTR2,
    _reserved14: [u8; 0x04],
    #[doc = "0x5c - TIM16 input selection register"]
    pub tim16_tisel: TIM16_TISEL,
    #[doc = "0x60 - TIM16 alternate function register 1"]
    pub tim16_af1: TIM16_AF1,
    #[doc = "0x64 - TIM16 alternate function register 2"]
    pub tim16_af2: TIM16_AF2,
    _reserved17: [u8; 0x0374],
    #[doc = "0x3dc - TIM16 DMA control register"]
    pub tim16_dcr: TIM16_DCR,
    #[doc = "0x3e0 - TIM16/TIM17 DMA address for full transfer"]
    pub tim16_dmar: TIM16_DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM16 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim16_ccmr1_output(&self) -> &TIM16_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM16 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim16_ccmr1_input(&self) -> &TIM16_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "TIM16_CR1 (rw) register accessor: an alias for `Reg<TIM16_CR1_SPEC>`"]
pub type TIM16_CR1 = crate::Reg<tim16_cr1::TIM16_CR1_SPEC>;
#[doc = "TIM16 control register 1"]
pub mod tim16_cr1;
#[doc = "TIM16_CR2 (rw) register accessor: an alias for `Reg<TIM16_CR2_SPEC>`"]
pub type TIM16_CR2 = crate::Reg<tim16_cr2::TIM16_CR2_SPEC>;
#[doc = "TIM16 control register 2"]
pub mod tim16_cr2;
#[doc = "TIM16_DIER (rw) register accessor: an alias for `Reg<TIM16_DIER_SPEC>`"]
pub type TIM16_DIER = crate::Reg<tim16_dier::TIM16_DIER_SPEC>;
#[doc = "TIM16 DMA/interrupt enable register"]
pub mod tim16_dier;
#[doc = "TIM16_SR (rw) register accessor: an alias for `Reg<TIM16_SR_SPEC>`"]
pub type TIM16_SR = crate::Reg<tim16_sr::TIM16_SR_SPEC>;
#[doc = "TIM16 status register"]
pub mod tim16_sr;
#[doc = "TIM16_EGR (w) register accessor: an alias for `Reg<TIM16_EGR_SPEC>`"]
pub type TIM16_EGR = crate::Reg<tim16_egr::TIM16_EGR_SPEC>;
#[doc = "TIM16 event generation register"]
pub mod tim16_egr;
#[doc = "TIM16_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM16_CCMR1_INPUT_SPEC>`"]
pub type TIM16_CCMR1_INPUT = crate::Reg<tim16_ccmr1_input::TIM16_CCMR1_INPUT_SPEC>;
#[doc = "TIM16 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim16_ccmr1_input;
#[doc = "TIM16_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM16_CCMR1_OUTPUT_SPEC>`"]
pub type TIM16_CCMR1_OUTPUT = crate::Reg<tim16_ccmr1_output::TIM16_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM16 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim16_ccmr1_output;
#[doc = "TIM16_CCER (rw) register accessor: an alias for `Reg<TIM16_CCER_SPEC>`"]
pub type TIM16_CCER = crate::Reg<tim16_ccer::TIM16_CCER_SPEC>;
#[doc = "TIM16 capture/compare enable register"]
pub mod tim16_ccer;
#[doc = "TIM16_CNT (rw) register accessor: an alias for `Reg<TIM16_CNT_SPEC>`"]
pub type TIM16_CNT = crate::Reg<tim16_cnt::TIM16_CNT_SPEC>;
#[doc = "TIM16 counter"]
pub mod tim16_cnt;
#[doc = "TIM16_PSC (rw) register accessor: an alias for `Reg<TIM16_PSC_SPEC>`"]
pub type TIM16_PSC = crate::Reg<tim16_psc::TIM16_PSC_SPEC>;
#[doc = "TIM16 prescaler"]
pub mod tim16_psc;
#[doc = "TIM16_ARR (rw) register accessor: an alias for `Reg<TIM16_ARR_SPEC>`"]
pub type TIM16_ARR = crate::Reg<tim16_arr::TIM16_ARR_SPEC>;
#[doc = "TIM16 auto-reload register"]
pub mod tim16_arr;
#[doc = "TIM16_RCR (rw) register accessor: an alias for `Reg<TIM16_RCR_SPEC>`"]
pub type TIM16_RCR = crate::Reg<tim16_rcr::TIM16_RCR_SPEC>;
#[doc = "TIM16 repetition counter register"]
pub mod tim16_rcr;
#[doc = "TIM16_CCR1 (rw) register accessor: an alias for `Reg<TIM16_CCR1_SPEC>`"]
pub type TIM16_CCR1 = crate::Reg<tim16_ccr1::TIM16_CCR1_SPEC>;
#[doc = "TIM16 capture/compare register 1"]
pub mod tim16_ccr1;
#[doc = "TIM16_BDTR (rw) register accessor: an alias for `Reg<TIM16_BDTR_SPEC>`"]
pub type TIM16_BDTR = crate::Reg<tim16_bdtr::TIM16_BDTR_SPEC>;
#[doc = "TIM16 break and dead-time register"]
pub mod tim16_bdtr;
#[doc = "TIM16_DTR2 (rw) register accessor: an alias for `Reg<TIM16_DTR2_SPEC>`"]
pub type TIM16_DTR2 = crate::Reg<tim16_dtr2::TIM16_DTR2_SPEC>;
#[doc = "TIM16 timer deadtime register 2"]
pub mod tim16_dtr2;
#[doc = "TIM16_TISEL (rw) register accessor: an alias for `Reg<TIM16_TISEL_SPEC>`"]
pub type TIM16_TISEL = crate::Reg<tim16_tisel::TIM16_TISEL_SPEC>;
#[doc = "TIM16 input selection register"]
pub mod tim16_tisel;
#[doc = "TIM16_AF1 (rw) register accessor: an alias for `Reg<TIM16_AF1_SPEC>`"]
pub type TIM16_AF1 = crate::Reg<tim16_af1::TIM16_AF1_SPEC>;
#[doc = "TIM16 alternate function register 1"]
pub mod tim16_af1;
#[doc = "TIM16_AF2 (rw) register accessor: an alias for `Reg<TIM16_AF2_SPEC>`"]
pub type TIM16_AF2 = crate::Reg<tim16_af2::TIM16_AF2_SPEC>;
#[doc = "TIM16 alternate function register 2"]
pub mod tim16_af2;
#[doc = "TIM16_DCR (rw) register accessor: an alias for `Reg<TIM16_DCR_SPEC>`"]
pub type TIM16_DCR = crate::Reg<tim16_dcr::TIM16_DCR_SPEC>;
#[doc = "TIM16 DMA control register"]
pub mod tim16_dcr;
#[doc = "TIM16_DMAR (rw) register accessor: an alias for `Reg<TIM16_DMAR_SPEC>`"]
pub type TIM16_DMAR = crate::Reg<tim16_dmar::TIM16_DMAR_SPEC>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod tim16_dmar;
