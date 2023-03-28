#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM5 control register 1"]
    pub tim5_cr1: TIM5_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM5 control register 2"]
    pub tim5_cr2: TIM5_CR2,
    #[doc = "0x08 - TIM5 slave mode control register"]
    pub tim5_smcr: TIM5_SMCR,
    #[doc = "0x0c - TIM5 DMA/Interrupt enable register"]
    pub tim5_dier: TIM5_DIER,
    #[doc = "0x10 - TIM5 status register"]
    pub tim5_sr: TIM5_SR,
    #[doc = "0x14 - TIM5 event generation register"]
    pub tim5_egr: TIM5_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim5_ccmr1: [u8; 0x04],
    _reserved_7_tim5_ccmr2: [u8; 0x04],
    #[doc = "0x20 - TIM5 capture/compare enable register"]
    pub tim5_ccer: TIM5_CCER,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - TIM5 counter"]
    pub tim5_cnt: TIM5_CNT,
    #[doc = "0x28 - TIM5 prescaler"]
    pub tim5_psc: TIM5_PSC,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM5 auto-reload register"]
    pub tim5_arr: TIM5_ARR,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - TIM5 capture/compare register 1"]
    pub tim5_ccr1: TIM5_CCR1,
    #[doc = "0x38 - TIM5 capture/compare register 2"]
    pub tim5_ccr2: TIM5_CCR2,
    #[doc = "0x3c - TIM5 capture/compare register 3"]
    pub tim5_ccr3: TIM5_CCR3,
    #[doc = "0x40 - TIM5 capture/compare register 4"]
    pub tim5_ccr4: TIM5_CCR4,
    _reserved16: [u8; 0x14],
    #[doc = "0x58 - TIM5 timer encoder control register"]
    pub tim5_ecr: TIM5_ECR,
    #[doc = "0x5c - TIM5 timer input selection register"]
    pub tim5_tisel: TIM5_TISEL,
    #[doc = "0x60 - TIM5 alternate function register 1"]
    pub tim5_af1: TIM5_AF1,
    #[doc = "0x64 - TIM5 alternate function register 2"]
    pub tim5_af2: TIM5_AF2,
    _reserved20: [u8; 0x0374],
    #[doc = "0x3dc - TIM5 DMA control register"]
    pub tim5_dcr: TIM5_DCR,
    #[doc = "0x3e0 - TIM5 DMA address for full transfer"]
    pub tim5_dmar: TIM5_DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM5 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim5_ccmr1_output(&self) -> &TIM5_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM5 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim5_ccmr1_input(&self) -> &TIM5_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - TIM5 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim5_ccmr2_output(&self) -> &TIM5_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - TIM5 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn tim5_ccmr2_input(&self) -> &TIM5_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "TIM5_CR1 (rw) register accessor: an alias for `Reg<TIM5_CR1_SPEC>`"]
pub type TIM5_CR1 = crate::Reg<tim5_cr1::TIM5_CR1_SPEC>;
#[doc = "TIM5 control register 1"]
pub mod tim5_cr1;
#[doc = "TIM5_CR2 (rw) register accessor: an alias for `Reg<TIM5_CR2_SPEC>`"]
pub type TIM5_CR2 = crate::Reg<tim5_cr2::TIM5_CR2_SPEC>;
#[doc = "TIM5 control register 2"]
pub mod tim5_cr2;
#[doc = "TIM5_SMCR (rw) register accessor: an alias for `Reg<TIM5_SMCR_SPEC>`"]
pub type TIM5_SMCR = crate::Reg<tim5_smcr::TIM5_SMCR_SPEC>;
#[doc = "TIM5 slave mode control register"]
pub mod tim5_smcr;
#[doc = "TIM5_DIER (rw) register accessor: an alias for `Reg<TIM5_DIER_SPEC>`"]
pub type TIM5_DIER = crate::Reg<tim5_dier::TIM5_DIER_SPEC>;
#[doc = "TIM5 DMA/Interrupt enable register"]
pub mod tim5_dier;
#[doc = "TIM5_SR (rw) register accessor: an alias for `Reg<TIM5_SR_SPEC>`"]
pub type TIM5_SR = crate::Reg<tim5_sr::TIM5_SR_SPEC>;
#[doc = "TIM5 status register"]
pub mod tim5_sr;
#[doc = "TIM5_EGR (w) register accessor: an alias for `Reg<TIM5_EGR_SPEC>`"]
pub type TIM5_EGR = crate::Reg<tim5_egr::TIM5_EGR_SPEC>;
#[doc = "TIM5 event generation register"]
pub mod tim5_egr;
#[doc = "TIM5_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM5_CCMR1_INPUT_SPEC>`"]
pub type TIM5_CCMR1_INPUT = crate::Reg<tim5_ccmr1_input::TIM5_CCMR1_INPUT_SPEC>;
#[doc = "TIM5 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim5_ccmr1_input;
#[doc = "TIM5_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM5_CCMR1_OUTPUT_SPEC>`"]
pub type TIM5_CCMR1_OUTPUT = crate::Reg<tim5_ccmr1_output::TIM5_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM5 capture/compare mode register 1 \\[alternate\\]"]
pub mod tim5_ccmr1_output;
#[doc = "TIM5_CCMR2_Input (rw) register accessor: an alias for `Reg<TIM5_CCMR2_INPUT_SPEC>`"]
pub type TIM5_CCMR2_INPUT = crate::Reg<tim5_ccmr2_input::TIM5_CCMR2_INPUT_SPEC>;
#[doc = "TIM5 capture/compare mode register 2 \\[alternate\\]"]
pub mod tim5_ccmr2_input;
#[doc = "TIM5_CCMR2_Output (rw) register accessor: an alias for `Reg<TIM5_CCMR2_OUTPUT_SPEC>`"]
pub type TIM5_CCMR2_OUTPUT = crate::Reg<tim5_ccmr2_output::TIM5_CCMR2_OUTPUT_SPEC>;
#[doc = "TIM5 capture/compare mode register 2 \\[alternate\\]"]
pub mod tim5_ccmr2_output;
#[doc = "TIM5_CCER (rw) register accessor: an alias for `Reg<TIM5_CCER_SPEC>`"]
pub type TIM5_CCER = crate::Reg<tim5_ccer::TIM5_CCER_SPEC>;
#[doc = "TIM5 capture/compare enable register"]
pub mod tim5_ccer;
#[doc = "TIM5_CNT (rw) register accessor: an alias for `Reg<TIM5_CNT_SPEC>`"]
pub type TIM5_CNT = crate::Reg<tim5_cnt::TIM5_CNT_SPEC>;
#[doc = "TIM5 counter"]
pub mod tim5_cnt;
#[doc = "TIM5_PSC (rw) register accessor: an alias for `Reg<TIM5_PSC_SPEC>`"]
pub type TIM5_PSC = crate::Reg<tim5_psc::TIM5_PSC_SPEC>;
#[doc = "TIM5 prescaler"]
pub mod tim5_psc;
#[doc = "TIM5_ARR (rw) register accessor: an alias for `Reg<TIM5_ARR_SPEC>`"]
pub type TIM5_ARR = crate::Reg<tim5_arr::TIM5_ARR_SPEC>;
#[doc = "TIM5 auto-reload register"]
pub mod tim5_arr;
#[doc = "TIM5_CCR1 (rw) register accessor: an alias for `Reg<TIM5_CCR1_SPEC>`"]
pub type TIM5_CCR1 = crate::Reg<tim5_ccr1::TIM5_CCR1_SPEC>;
#[doc = "TIM5 capture/compare register 1"]
pub mod tim5_ccr1;
#[doc = "TIM5_CCR2 (rw) register accessor: an alias for `Reg<TIM5_CCR2_SPEC>`"]
pub type TIM5_CCR2 = crate::Reg<tim5_ccr2::TIM5_CCR2_SPEC>;
#[doc = "TIM5 capture/compare register 2"]
pub mod tim5_ccr2;
#[doc = "TIM5_CCR3 (rw) register accessor: an alias for `Reg<TIM5_CCR3_SPEC>`"]
pub type TIM5_CCR3 = crate::Reg<tim5_ccr3::TIM5_CCR3_SPEC>;
#[doc = "TIM5 capture/compare register 3"]
pub mod tim5_ccr3;
#[doc = "TIM5_CCR4 (rw) register accessor: an alias for `Reg<TIM5_CCR4_SPEC>`"]
pub type TIM5_CCR4 = crate::Reg<tim5_ccr4::TIM5_CCR4_SPEC>;
#[doc = "TIM5 capture/compare register 4"]
pub mod tim5_ccr4;
#[doc = "TIM5_ECR (rw) register accessor: an alias for `Reg<TIM5_ECR_SPEC>`"]
pub type TIM5_ECR = crate::Reg<tim5_ecr::TIM5_ECR_SPEC>;
#[doc = "TIM5 timer encoder control register"]
pub mod tim5_ecr;
#[doc = "TIM5_TISEL (rw) register accessor: an alias for `Reg<TIM5_TISEL_SPEC>`"]
pub type TIM5_TISEL = crate::Reg<tim5_tisel::TIM5_TISEL_SPEC>;
#[doc = "TIM5 timer input selection register"]
pub mod tim5_tisel;
#[doc = "TIM5_AF1 (rw) register accessor: an alias for `Reg<TIM5_AF1_SPEC>`"]
pub type TIM5_AF1 = crate::Reg<tim5_af1::TIM5_AF1_SPEC>;
#[doc = "TIM5 alternate function register 1"]
pub mod tim5_af1;
#[doc = "TIM5_AF2 (rw) register accessor: an alias for `Reg<TIM5_AF2_SPEC>`"]
pub type TIM5_AF2 = crate::Reg<tim5_af2::TIM5_AF2_SPEC>;
#[doc = "TIM5 alternate function register 2"]
pub mod tim5_af2;
#[doc = "TIM5_DCR (rw) register accessor: an alias for `Reg<TIM5_DCR_SPEC>`"]
pub type TIM5_DCR = crate::Reg<tim5_dcr::TIM5_DCR_SPEC>;
#[doc = "TIM5 DMA control register"]
pub mod tim5_dcr;
#[doc = "TIM5_DMAR (rw) register accessor: an alias for `Reg<TIM5_DMAR_SPEC>`"]
pub type TIM5_DMAR = crate::Reg<tim5_dmar::TIM5_DMAR_SPEC>;
#[doc = "TIM5 DMA address for full transfer"]
pub mod tim5_dmar;
