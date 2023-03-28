#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    pub cr1: CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM1 control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - TIM1 slave mode control register"]
    pub smcr: SMCR,
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    pub dier: DIER,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM1 status register"]
    pub sr: SR,
    #[doc = "0x14 - TIM1 event generation register"]
    pub egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - TIM1 counter"]
    pub cnt: CNT,
    #[doc = "0x28 - TIM1 prescaler"]
    pub psc: PSC,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM1 auto-reload register"]
    pub arr: ARR,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM1 repetition counter register"]
    pub rcr: RCR,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    pub ccr1: CCR1,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    pub ccr2: CCR2,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    pub ccr3: CCR3,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    pub ccr4: CCR4,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - TIM1 break and dead-time register"]
    pub bdtr: BDTR,
    #[doc = "0x48 - TIM1 DMA control register"]
    pub dcr: DCR,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM1 DMA address for full transfer"]
    pub dmar: DMAR,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - TIM1 capture/compare mode register 3"]
    pub ccmr3: CCMR3,
    #[doc = "0x58 - TIM1 capture/compare register 5"]
    pub ccr5: CCR5,
    #[doc = "0x5c - TIM1 capture/compare register 6"]
    pub ccr6: CCR6,
    _reserved23: [u8; 0x02],
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    pub af1: AF1,
    #[doc = "0x64 - TIM1 Alternate function register 2"]
    pub af2: AF2,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub tisel: TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM1 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM1 control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "TIM1 slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM1 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM1 event generation register"]
pub mod egr;
#[doc = "CCMR1_input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCMR2_input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`"]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_input;
#[doc = "CCMR2_output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`"]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_output;
#[doc = "CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "TIM1 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM1 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM1 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM1 auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "TIM1 repetition counter register"]
pub mod rcr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "TIM1 capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "TIM1 capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "TIM1 capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "TIM1 capture/compare register 4"]
pub mod ccr4;
#[doc = "BDTR (rw) register accessor: an alias for `Reg<BDTR_SPEC>`"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "TIM1 break and dead-time register"]
pub mod bdtr;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "TIM1 DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod dmar;
#[doc = "CCMR3 (rw) register accessor: an alias for `Reg<CCMR3_SPEC>`"]
pub type CCMR3 = crate::Reg<ccmr3::CCMR3_SPEC>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod ccmr3;
#[doc = "CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "TIM1 capture/compare register 5"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "TIM1 capture/compare register 6"]
pub mod ccr6;
#[doc = "AF1 (rw) register accessor: an alias for `Reg<AF1_SPEC>`"]
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
#[doc = "TIM1 alternate function option register 1"]
pub mod af1;
#[doc = "AF2 (rw) register accessor: an alias for `Reg<AF2_SPEC>`"]
pub type AF2 = crate::Reg<af2::AF2_SPEC>;
#[doc = "TIM1 Alternate function register 2"]
pub mod af2;
#[doc = "TISEL (rw) register accessor: an alias for `Reg<TISEL_SPEC>`"]
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
#[doc = "TIM1 timer input selection register"]
pub mod tisel;
