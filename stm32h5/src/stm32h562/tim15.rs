#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM15 control register 1"]
    pub tim15_cr1: TIM15_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM15 control register 2"]
    pub tim15_cr2: TIM15_CR2,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - TIM15 slave mode control register"]
    pub tim15_smcr: TIM15_SMCR,
    #[doc = "0x0c - TIM15 DMA/interrupt enable register"]
    pub tim15_dier: TIM15_DIER,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM15 status register"]
    pub tim15_sr: TIM15_SR,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - TIM15 event generation register"]
    pub tim15_egr: TIM15_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim15_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - TIM15 capture/compare enable register"]
    pub tim15_ccer: TIM15_CCER,
    _reserved8: [u8; 0x02],
    #[doc = "0x24 - TIM15 counter"]
    pub tim15_cnt: TIM15_CNT,
    #[doc = "0x28 - TIM15 prescaler"]
    pub tim15_psc: TIM15_PSC,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - TIM15 auto-reload register"]
    pub tim15_arr: TIM15_ARR,
    #[doc = "0x30 - TIM15 repetition counter register"]
    pub tim15_rcr: TIM15_RCR,
    _reserved12: [u8; 0x02],
    #[doc = "0x34 - TIM15 capture/compare register 1"]
    pub tim15_ccr1: TIM15_CCR1,
    #[doc = "0x38 - TIM15 capture/compare register 2"]
    pub tim15_ccr2: TIM15_CCR2,
    _reserved14: [u8; 0x08],
    #[doc = "0x44 - TIM15 break and dead-time register"]
    pub tim15_bdtr: TIM15_BDTR,
    _reserved15: [u8; 0x0c],
    #[doc = "0x54 - TIM15 timer deadtime register 2"]
    pub tim15_dtr2: TIM15_DTR2,
    _reserved16: [u8; 0x04],
    #[doc = "0x5c - TIM15 input selection register"]
    pub tim15_tisel: TIM15_TISEL,
    #[doc = "0x60 - TIM15 alternate function register 1"]
    pub tim15_af1: TIM15_AF1,
    #[doc = "0x64 - TIM15 alternate function register 2"]
    pub tim15_af2: TIM15_AF2,
    _reserved19: [u8; 0x0374],
    #[doc = "0x3dc - TIM15 DMA control register"]
    pub tim15_dcr: TIM15_DCR,
    #[doc = "0x3e0 - TIM15 DMA address for full transfer"]
    pub tim15_dmar: TIM15_DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM15 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim15_ccmr1_output(&self) -> &TIM15_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM15 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim15_ccmr1_input(&self) -> &TIM15_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "TIM15_CR1 (rw) register accessor: an alias for `Reg<TIM15_CR1_SPEC>`"]
pub type TIM15_CR1 = crate::Reg<tim15_cr1::TIM15_CR1_SPEC>;
#[doc = "TIM15 control register 1"]
pub mod tim15_cr1;
#[doc = "TIM15_CR2 (rw) register accessor: an alias for `Reg<TIM15_CR2_SPEC>`"]
pub type TIM15_CR2 = crate::Reg<tim15_cr2::TIM15_CR2_SPEC>;
#[doc = "TIM15 control register 2"]
pub mod tim15_cr2;
#[doc = "TIM15_SMCR (rw) register accessor: an alias for `Reg<TIM15_SMCR_SPEC>`"]
pub type TIM15_SMCR = crate::Reg<tim15_smcr::TIM15_SMCR_SPEC>;
#[doc = "TIM15 slave mode control register"]
pub mod tim15_smcr;
#[doc = "TIM15_DIER (rw) register accessor: an alias for `Reg<TIM15_DIER_SPEC>`"]
pub type TIM15_DIER = crate::Reg<tim15_dier::TIM15_DIER_SPEC>;
#[doc = "TIM15 DMA/interrupt enable register"]
pub mod tim15_dier;
#[doc = "TIM15_SR (rw) register accessor: an alias for `Reg<TIM15_SR_SPEC>`"]
pub type TIM15_SR = crate::Reg<tim15_sr::TIM15_SR_SPEC>;
#[doc = "TIM15 status register"]
pub mod tim15_sr;
#[doc = "TIM15_EGR (rw) register accessor: an alias for `Reg<TIM15_EGR_SPEC>`"]
pub type TIM15_EGR = crate::Reg<tim15_egr::TIM15_EGR_SPEC>;
#[doc = "TIM15 event generation register"]
pub mod tim15_egr;
#[doc = "TIM15_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM15_CCMR1_INPUT_SPEC>`"]
pub type TIM15_CCMR1_INPUT = crate::Reg<tim15_ccmr1_input::TIM15_CCMR1_INPUT_SPEC>;
#[doc = "TIM15 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim15_ccmr1_input;
#[doc = "TIM15_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM15_CCMR1_OUTPUT_SPEC>`"]
pub type TIM15_CCMR1_OUTPUT = crate::Reg<tim15_ccmr1_output::TIM15_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM15 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim15_ccmr1_output;
#[doc = "TIM15_CCER (rw) register accessor: an alias for `Reg<TIM15_CCER_SPEC>`"]
pub type TIM15_CCER = crate::Reg<tim15_ccer::TIM15_CCER_SPEC>;
#[doc = "TIM15 capture/compare enable register"]
pub mod tim15_ccer;
#[doc = "TIM15_CNT (rw) register accessor: an alias for `Reg<TIM15_CNT_SPEC>`"]
pub type TIM15_CNT = crate::Reg<tim15_cnt::TIM15_CNT_SPEC>;
#[doc = "TIM15 counter"]
pub mod tim15_cnt;
#[doc = "TIM15_PSC (rw) register accessor: an alias for `Reg<TIM15_PSC_SPEC>`"]
pub type TIM15_PSC = crate::Reg<tim15_psc::TIM15_PSC_SPEC>;
#[doc = "TIM15 prescaler"]
pub mod tim15_psc;
#[doc = "TIM15_ARR (rw) register accessor: an alias for `Reg<TIM15_ARR_SPEC>`"]
pub type TIM15_ARR = crate::Reg<tim15_arr::TIM15_ARR_SPEC>;
#[doc = "TIM15 auto-reload register"]
pub mod tim15_arr;
#[doc = "TIM15_RCR (rw) register accessor: an alias for `Reg<TIM15_RCR_SPEC>`"]
pub type TIM15_RCR = crate::Reg<tim15_rcr::TIM15_RCR_SPEC>;
#[doc = "TIM15 repetition counter register"]
pub mod tim15_rcr;
#[doc = "TIM15_CCR1 (rw) register accessor: an alias for `Reg<TIM15_CCR1_SPEC>`"]
pub type TIM15_CCR1 = crate::Reg<tim15_ccr1::TIM15_CCR1_SPEC>;
#[doc = "TIM15 capture/compare register 1"]
pub mod tim15_ccr1;
#[doc = "TIM15_CCR2 (rw) register accessor: an alias for `Reg<TIM15_CCR2_SPEC>`"]
pub type TIM15_CCR2 = crate::Reg<tim15_ccr2::TIM15_CCR2_SPEC>;
#[doc = "TIM15 capture/compare register 2"]
pub mod tim15_ccr2;
#[doc = "TIM15_BDTR (rw) register accessor: an alias for `Reg<TIM15_BDTR_SPEC>`"]
pub type TIM15_BDTR = crate::Reg<tim15_bdtr::TIM15_BDTR_SPEC>;
#[doc = "TIM15 break and dead-time register"]
pub mod tim15_bdtr;
#[doc = "TIM15_DTR2 (rw) register accessor: an alias for `Reg<TIM15_DTR2_SPEC>`"]
pub type TIM15_DTR2 = crate::Reg<tim15_dtr2::TIM15_DTR2_SPEC>;
#[doc = "TIM15 timer deadtime register 2"]
pub mod tim15_dtr2;
#[doc = "TIM15_TISEL (rw) register accessor: an alias for `Reg<TIM15_TISEL_SPEC>`"]
pub type TIM15_TISEL = crate::Reg<tim15_tisel::TIM15_TISEL_SPEC>;
#[doc = "TIM15 input selection register"]
pub mod tim15_tisel;
#[doc = "TIM15_AF1 (rw) register accessor: an alias for `Reg<TIM15_AF1_SPEC>`"]
pub type TIM15_AF1 = crate::Reg<tim15_af1::TIM15_AF1_SPEC>;
#[doc = "TIM15 alternate function register 1"]
pub mod tim15_af1;
#[doc = "TIM15_AF2 (rw) register accessor: an alias for `Reg<TIM15_AF2_SPEC>`"]
pub type TIM15_AF2 = crate::Reg<tim15_af2::TIM15_AF2_SPEC>;
#[doc = "TIM15 alternate function register 2"]
pub mod tim15_af2;
#[doc = "TIM15_DCR (rw) register accessor: an alias for `Reg<TIM15_DCR_SPEC>`"]
pub type TIM15_DCR = crate::Reg<tim15_dcr::TIM15_DCR_SPEC>;
#[doc = "TIM15 DMA control register"]
pub mod tim15_dcr;
#[doc = "TIM15_DMAR (rw) register accessor: an alias for `Reg<TIM15_DMAR_SPEC>`"]
pub type TIM15_DMAR = crate::Reg<tim15_dmar::TIM15_DMAR_SPEC>;
#[doc = "TIM15 DMA address for full transfer"]
pub mod tim15_dmar;
