#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM14 control register 1"]
    pub cr1: CR1,
    _reserved1: [u8; 0x0a],
    #[doc = "0x0c - TIM14 Interrupt enable register"]
    pub dier: DIER,
    _reserved2: [u8; 0x02],
    #[doc = "0x10 - TIM14 status register"]
    pub sr: SR,
    _reserved3: [u8; 0x02],
    #[doc = "0x14 - TIM14 event generation register"]
    pub egr: EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - TIM14 capture/compare enable register"]
    pub ccer: CCER,
    _reserved6: [u8; 0x02],
    #[doc = "0x24 - TIM14 counter"]
    pub cnt: CNT,
    #[doc = "0x28 - TIM14 prescaler"]
    pub psc: PSC,
    _reserved8: [u8; 0x02],
    #[doc = "0x2c - TIM14 auto-reload register"]
    pub arr: ARR,
    _reserved9: [u8; 0x06],
    #[doc = "0x34 - TIM14 capture/compare register 1"]
    pub ccr1: CCR1,
    _reserved10: [u8; 0x32],
    #[doc = "0x68 - TIM14 timer input selection register"]
    pub tisel: TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM14 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIM14 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM14 control register 1"]
pub mod cr1;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM14 Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM14 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM14 event generation register"]
pub mod egr;
#[doc = "CCMR1_input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "TIM14 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "TIM14 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "TIM14 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM14 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM14 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM14 auto-reload register"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "TIM14 capture/compare register 1"]
pub mod ccr1;
#[doc = "TISEL (rw) register accessor: an alias for `Reg<TISEL_SPEC>`"]
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
#[doc = "TIM14 timer input selection register"]
pub mod tisel;
