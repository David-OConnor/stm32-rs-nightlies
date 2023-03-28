#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM17 control register 1"]
    pub tim17_cr1: TIM17_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM17 control register 2"]
    pub tim17_cr2: TIM17_CR2,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - TIM17 DMA/interrupt enable register"]
    pub tim17_dier: TIM17_DIER,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - TIM17 status register"]
    pub tim17_sr: TIM17_SR,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - TIM17 event generation register"]
    pub tim17_egr: TIM17_EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_tim17_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - TIM17 capture/compare enable register"]
    pub tim17_ccer: TIM17_CCER,
    _reserved7: [u8; 0x02],
    #[doc = "0x24 - TIM17 counter"]
    pub tim17_cnt: TIM17_CNT,
    #[doc = "0x28 - TIM17 prescaler"]
    pub tim17_psc: TIM17_PSC,
    _reserved9: [u8; 0x02],
    #[doc = "0x2c - TIM17 auto-reload register"]
    pub tim17_arr: TIM17_ARR,
    #[doc = "0x30 - TIM17 repetition counter register"]
    pub tim17_rcr: TIM17_RCR,
    _reserved11: [u8; 0x02],
    #[doc = "0x34 - TIM17 capture/compare register 1"]
    pub tim17_ccr1: TIM17_CCR1,
    _reserved12: [u8; 0x0c],
    #[doc = "0x44 - TIM17 break and dead-time register"]
    pub tim17_bdtr: TIM17_BDTR,
    _reserved13: [u8; 0x0c],
    #[doc = "0x54 - TIM17 timer deadtime register 2"]
    pub tim17_dtr2: TIM17_DTR2,
    _reserved14: [u8; 0x04],
    #[doc = "0x5c - TIM17 input selection register"]
    pub tim17_tisel: TIM17_TISEL,
    #[doc = "0x60 - TIM17 alternate function register 1"]
    pub tim17_af1: TIM17_AF1,
    #[doc = "0x64 - TIM17 alternate function register 2"]
    pub tim17_af2: TIM17_AF2,
    _reserved17: [u8; 0x0374],
    #[doc = "0x3dc - TIM17 DMA control register"]
    pub tim17_dcr: TIM17_DCR,
    #[doc = "0x3e0 - TIM16/TIM17 DMA address for full transfer"]
    pub tim17_dmar: TIM17_DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM17 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim17_ccmr1_output(&self) -> &TIM17_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM17 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim17_ccmr1_input(&self) -> &TIM17_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "TIM17_CR1 (rw) register accessor: an alias for `Reg<TIM17_CR1_SPEC>`"]
pub type TIM17_CR1 = crate::Reg<tim17_cr1::TIM17_CR1_SPEC>;
#[doc = "TIM17 control register 1"]
pub mod tim17_cr1;
#[doc = "TIM17_CR2 (rw) register accessor: an alias for `Reg<TIM17_CR2_SPEC>`"]
pub type TIM17_CR2 = crate::Reg<tim17_cr2::TIM17_CR2_SPEC>;
#[doc = "TIM17 control register 2"]
pub mod tim17_cr2;
#[doc = "TIM17_DIER (rw) register accessor: an alias for `Reg<TIM17_DIER_SPEC>`"]
pub type TIM17_DIER = crate::Reg<tim17_dier::TIM17_DIER_SPEC>;
#[doc = "TIM17 DMA/interrupt enable register"]
pub mod tim17_dier;
#[doc = "TIM17_SR (rw) register accessor: an alias for `Reg<TIM17_SR_SPEC>`"]
pub type TIM17_SR = crate::Reg<tim17_sr::TIM17_SR_SPEC>;
#[doc = "TIM17 status register"]
pub mod tim17_sr;
#[doc = "TIM17_EGR (w) register accessor: an alias for `Reg<TIM17_EGR_SPEC>`"]
pub type TIM17_EGR = crate::Reg<tim17_egr::TIM17_EGR_SPEC>;
#[doc = "TIM17 event generation register"]
pub mod tim17_egr;
#[doc = "TIM17_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM17_CCMR1_INPUT_SPEC>`"]
pub type TIM17_CCMR1_INPUT = crate::Reg<tim17_ccmr1_input::TIM17_CCMR1_INPUT_SPEC>;
#[doc = "TIM17 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim17_ccmr1_input;
#[doc = "TIM17_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM17_CCMR1_OUTPUT_SPEC>`"]
pub type TIM17_CCMR1_OUTPUT = crate::Reg<tim17_ccmr1_output::TIM17_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM17 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim17_ccmr1_output;
#[doc = "TIM17_CCER (rw) register accessor: an alias for `Reg<TIM17_CCER_SPEC>`"]
pub type TIM17_CCER = crate::Reg<tim17_ccer::TIM17_CCER_SPEC>;
#[doc = "TIM17 capture/compare enable register"]
pub mod tim17_ccer;
#[doc = "TIM17_CNT (rw) register accessor: an alias for `Reg<TIM17_CNT_SPEC>`"]
pub type TIM17_CNT = crate::Reg<tim17_cnt::TIM17_CNT_SPEC>;
#[doc = "TIM17 counter"]
pub mod tim17_cnt;
#[doc = "TIM17_PSC (rw) register accessor: an alias for `Reg<TIM17_PSC_SPEC>`"]
pub type TIM17_PSC = crate::Reg<tim17_psc::TIM17_PSC_SPEC>;
#[doc = "TIM17 prescaler"]
pub mod tim17_psc;
#[doc = "TIM17_ARR (rw) register accessor: an alias for `Reg<TIM17_ARR_SPEC>`"]
pub type TIM17_ARR = crate::Reg<tim17_arr::TIM17_ARR_SPEC>;
#[doc = "TIM17 auto-reload register"]
pub mod tim17_arr;
#[doc = "TIM17_RCR (rw) register accessor: an alias for `Reg<TIM17_RCR_SPEC>`"]
pub type TIM17_RCR = crate::Reg<tim17_rcr::TIM17_RCR_SPEC>;
#[doc = "TIM17 repetition counter register"]
pub mod tim17_rcr;
#[doc = "TIM17_CCR1 (rw) register accessor: an alias for `Reg<TIM17_CCR1_SPEC>`"]
pub type TIM17_CCR1 = crate::Reg<tim17_ccr1::TIM17_CCR1_SPEC>;
#[doc = "TIM17 capture/compare register 1"]
pub mod tim17_ccr1;
#[doc = "TIM17_BDTR (rw) register accessor: an alias for `Reg<TIM17_BDTR_SPEC>`"]
pub type TIM17_BDTR = crate::Reg<tim17_bdtr::TIM17_BDTR_SPEC>;
#[doc = "TIM17 break and dead-time register"]
pub mod tim17_bdtr;
#[doc = "TIM17_DTR2 (rw) register accessor: an alias for `Reg<TIM17_DTR2_SPEC>`"]
pub type TIM17_DTR2 = crate::Reg<tim17_dtr2::TIM17_DTR2_SPEC>;
#[doc = "TIM17 timer deadtime register 2"]
pub mod tim17_dtr2;
#[doc = "TIM17_TISEL (rw) register accessor: an alias for `Reg<TIM17_TISEL_SPEC>`"]
pub type TIM17_TISEL = crate::Reg<tim17_tisel::TIM17_TISEL_SPEC>;
#[doc = "TIM17 input selection register"]
pub mod tim17_tisel;
#[doc = "TIM17_AF1 (rw) register accessor: an alias for `Reg<TIM17_AF1_SPEC>`"]
pub type TIM17_AF1 = crate::Reg<tim17_af1::TIM17_AF1_SPEC>;
#[doc = "TIM17 alternate function register 1"]
pub mod tim17_af1;
#[doc = "TIM17_AF2 (rw) register accessor: an alias for `Reg<TIM17_AF2_SPEC>`"]
pub type TIM17_AF2 = crate::Reg<tim17_af2::TIM17_AF2_SPEC>;
#[doc = "TIM17 alternate function register 2"]
pub mod tim17_af2;
#[doc = "TIM17_DCR (rw) register accessor: an alias for `Reg<TIM17_DCR_SPEC>`"]
pub type TIM17_DCR = crate::Reg<tim17_dcr::TIM17_DCR_SPEC>;
#[doc = "TIM17 DMA control register"]
pub mod tim17_dcr;
#[doc = "TIM17_DMAR (rw) register accessor: an alias for `Reg<TIM17_DMAR_SPEC>`"]
pub type TIM17_DMAR = crate::Reg<tim17_dmar::TIM17_DMAR_SPEC>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod tim17_dmar;
