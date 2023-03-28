#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    pub tim1_cr1: TIM1_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM1 control register 2"]
    pub tim1_cr2: TIM1_CR2,
    #[doc = "0x08 - TIM1 slave mode control register"]
    pub tim1_smcr: TIM1_SMCR,
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    pub tim1_dier: TIM1_DIER,
    #[doc = "0x10 - TIM1 status register"]
    pub tim1_sr: TIM1_SR,
    #[doc = "0x14 - TIM1 event generation register"]
    pub tim1_egr: TIM1_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim1_ccmr1: [u8; 0x04],
    _reserved_7_tim1_ccmr2: [u8; 0x04],
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    pub tim1_ccer: TIM1_CCER,
    #[doc = "0x24 - TIM1 counter"]
    pub tim1_cnt: TIM1_CNT,
    #[doc = "0x28 - TIM1 prescaler"]
    pub tim1_psc: TIM1_PSC,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM1 auto-reload register"]
    pub tim1_arr: TIM1_ARR,
    #[doc = "0x30 - TIM1 repetition counter register"]
    pub tim1_rcr: TIM1_RCR,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    pub tim1_ccr1: TIM1_CCR1,
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    pub tim1_ccr2: TIM1_CCR2,
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    pub tim1_ccr3: TIM1_CCR3,
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    pub tim1_ccr4: TIM1_CCR4,
    #[doc = "0x44 - TIM1 break and dead-time register"]
    pub tim1_bdtr: TIM1_BDTR,
    #[doc = "0x48 - TIM1 capture/compare register 5"]
    pub tim1_ccr5: TIM1_CCR5,
    #[doc = "0x4c - TIM1 capture/compare register 6"]
    pub tim1_ccr6: TIM1_CCR6,
    #[doc = "0x50 - TIM1 capture/compare mode register 3"]
    pub tim1_ccmr3: TIM1_CCMR3,
    #[doc = "0x54 - TIM1 timer deadtime register 2"]
    pub tim1_dtr2: TIM1_DTR2,
    #[doc = "0x58 - TIM1 timer encoder control register"]
    pub tim1_ecr: TIM1_ECR,
    #[doc = "0x5c - TIM1 timer input selection register"]
    pub tim1_tisel: TIM1_TISEL,
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    pub tim1_af1: TIM1_AF1,
    #[doc = "0x64 - TIM1 alternate function register 2"]
    pub tim1_af2: TIM1_AF2,
    _reserved26: [u8; 0x0374],
    #[doc = "0x3dc - TIM1 DMA control register"]
    pub tim1_dcr: TIM1_DCR,
    #[doc = "0x3e0 - TIM1 DMA address for full transfer"]
    pub tim1_dmar: TIM1_DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr1_output(&self) -> &TIM1_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr1_input(&self) -> &TIM1_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr2_output(&self) -> &TIM1_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim1_ccmr2_input(&self) -> &TIM1_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "TIM1_CR1 (rw) register accessor: an alias for `Reg<TIM1_CR1_SPEC>`"]
pub type TIM1_CR1 = crate::Reg<tim1_cr1::TIM1_CR1_SPEC>;
#[doc = "TIM1 control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1_CR2 (rw) register accessor: an alias for `Reg<TIM1_CR2_SPEC>`"]
pub type TIM1_CR2 = crate::Reg<tim1_cr2::TIM1_CR2_SPEC>;
#[doc = "TIM1 control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1_SMCR (rw) register accessor: an alias for `Reg<TIM1_SMCR_SPEC>`"]
pub type TIM1_SMCR = crate::Reg<tim1_smcr::TIM1_SMCR_SPEC>;
#[doc = "TIM1 slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1_DIER (rw) register accessor: an alias for `Reg<TIM1_DIER_SPEC>`"]
pub type TIM1_DIER = crate::Reg<tim1_dier::TIM1_DIER_SPEC>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1_SR (rw) register accessor: an alias for `Reg<TIM1_SR_SPEC>`"]
pub type TIM1_SR = crate::Reg<tim1_sr::TIM1_SR_SPEC>;
#[doc = "TIM1 status register"]
pub mod tim1_sr;
#[doc = "TIM1_EGR (w) register accessor: an alias for `Reg<TIM1_EGR_SPEC>`"]
pub type TIM1_EGR = crate::Reg<tim1_egr::TIM1_EGR_SPEC>;
#[doc = "TIM1 event generation register"]
pub mod tim1_egr;
#[doc = "TIM1_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM1_CCMR1_INPUT_SPEC>`"]
pub type TIM1_CCMR1_INPUT = crate::Reg<tim1_ccmr1_input::TIM1_CCMR1_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim1_ccmr1_input;
#[doc = "TIM1_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM1_CCMR1_OUTPUT_SPEC>`"]
pub type TIM1_CCMR1_OUTPUT = crate::Reg<tim1_ccmr1_output::TIM1_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim1_ccmr1_output;
#[doc = "TIM1_CCMR2_Input (rw) register accessor: an alias for `Reg<TIM1_CCMR2_INPUT_SPEC>`"]
pub type TIM1_CCMR2_INPUT = crate::Reg<tim1_ccmr2_input::TIM1_CCMR2_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod tim1_ccmr2_input;
#[doc = "TIM1_CCMR2_Output (rw) register accessor: an alias for `Reg<TIM1_CCMR2_OUTPUT_SPEC>`"]
pub type TIM1_CCMR2_OUTPUT = crate::Reg<tim1_ccmr2_output::TIM1_CCMR2_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod tim1_ccmr2_output;
#[doc = "TIM1_CCER (rw) register accessor: an alias for `Reg<TIM1_CCER_SPEC>`"]
pub type TIM1_CCER = crate::Reg<tim1_ccer::TIM1_CCER_SPEC>;
#[doc = "TIM1 capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1_CNT (rw) register accessor: an alias for `Reg<TIM1_CNT_SPEC>`"]
pub type TIM1_CNT = crate::Reg<tim1_cnt::TIM1_CNT_SPEC>;
#[doc = "TIM1 counter"]
pub mod tim1_cnt;
#[doc = "TIM1_PSC (rw) register accessor: an alias for `Reg<TIM1_PSC_SPEC>`"]
pub type TIM1_PSC = crate::Reg<tim1_psc::TIM1_PSC_SPEC>;
#[doc = "TIM1 prescaler"]
pub mod tim1_psc;
#[doc = "TIM1_ARR (rw) register accessor: an alias for `Reg<TIM1_ARR_SPEC>`"]
pub type TIM1_ARR = crate::Reg<tim1_arr::TIM1_ARR_SPEC>;
#[doc = "TIM1 auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1_RCR (rw) register accessor: an alias for `Reg<TIM1_RCR_SPEC>`"]
pub type TIM1_RCR = crate::Reg<tim1_rcr::TIM1_RCR_SPEC>;
#[doc = "TIM1 repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1_CCR1 (rw) register accessor: an alias for `Reg<TIM1_CCR1_SPEC>`"]
pub type TIM1_CCR1 = crate::Reg<tim1_ccr1::TIM1_CCR1_SPEC>;
#[doc = "TIM1 capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1_CCR2 (rw) register accessor: an alias for `Reg<TIM1_CCR2_SPEC>`"]
pub type TIM1_CCR2 = crate::Reg<tim1_ccr2::TIM1_CCR2_SPEC>;
#[doc = "TIM1 capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1_CCR3 (rw) register accessor: an alias for `Reg<TIM1_CCR3_SPEC>`"]
pub type TIM1_CCR3 = crate::Reg<tim1_ccr3::TIM1_CCR3_SPEC>;
#[doc = "TIM1 capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1_CCR4 (rw) register accessor: an alias for `Reg<TIM1_CCR4_SPEC>`"]
pub type TIM1_CCR4 = crate::Reg<tim1_ccr4::TIM1_CCR4_SPEC>;
#[doc = "TIM1 capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "TIM1_BDTR (rw) register accessor: an alias for `Reg<TIM1_BDTR_SPEC>`"]
pub type TIM1_BDTR = crate::Reg<tim1_bdtr::TIM1_BDTR_SPEC>;
#[doc = "TIM1 break and dead-time register"]
pub mod tim1_bdtr;
#[doc = "TIM1_CCR5 (rw) register accessor: an alias for `Reg<TIM1_CCR5_SPEC>`"]
pub type TIM1_CCR5 = crate::Reg<tim1_ccr5::TIM1_CCR5_SPEC>;
#[doc = "TIM1 capture/compare register 5"]
pub mod tim1_ccr5;
#[doc = "TIM1_CCR6 (rw) register accessor: an alias for `Reg<TIM1_CCR6_SPEC>`"]
pub type TIM1_CCR6 = crate::Reg<tim1_ccr6::TIM1_CCR6_SPEC>;
#[doc = "TIM1 capture/compare register 6"]
pub mod tim1_ccr6;
#[doc = "TIM1_CCMR3 (rw) register accessor: an alias for `Reg<TIM1_CCMR3_SPEC>`"]
pub type TIM1_CCMR3 = crate::Reg<tim1_ccmr3::TIM1_CCMR3_SPEC>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod tim1_ccmr3;
#[doc = "TIM1_DTR2 (rw) register accessor: an alias for `Reg<TIM1_DTR2_SPEC>`"]
pub type TIM1_DTR2 = crate::Reg<tim1_dtr2::TIM1_DTR2_SPEC>;
#[doc = "TIM1 timer deadtime register 2"]
pub mod tim1_dtr2;
#[doc = "TIM1_ECR (rw) register accessor: an alias for `Reg<TIM1_ECR_SPEC>`"]
pub type TIM1_ECR = crate::Reg<tim1_ecr::TIM1_ECR_SPEC>;
#[doc = "TIM1 timer encoder control register"]
pub mod tim1_ecr;
#[doc = "TIM1_TISEL (rw) register accessor: an alias for `Reg<TIM1_TISEL_SPEC>`"]
pub type TIM1_TISEL = crate::Reg<tim1_tisel::TIM1_TISEL_SPEC>;
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
#[doc = "TIM1_AF1 (rw) register accessor: an alias for `Reg<TIM1_AF1_SPEC>`"]
pub type TIM1_AF1 = crate::Reg<tim1_af1::TIM1_AF1_SPEC>;
#[doc = "TIM1 alternate function option register 1"]
pub mod tim1_af1;
#[doc = "TIM1_AF2 (rw) register accessor: an alias for `Reg<TIM1_AF2_SPEC>`"]
pub type TIM1_AF2 = crate::Reg<tim1_af2::TIM1_AF2_SPEC>;
#[doc = "TIM1 alternate function register 2"]
pub mod tim1_af2;
#[doc = "TIM1_DCR (rw) register accessor: an alias for `Reg<TIM1_DCR_SPEC>`"]
pub type TIM1_DCR = crate::Reg<tim1_dcr::TIM1_DCR_SPEC>;
#[doc = "TIM1 DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1_DMAR (rw) register accessor: an alias for `Reg<TIM1_DMAR_SPEC>`"]
pub type TIM1_DMAR = crate::Reg<tim1_dmar::TIM1_DMAR_SPEC>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod tim1_dmar;
