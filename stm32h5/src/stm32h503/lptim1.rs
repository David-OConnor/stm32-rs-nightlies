#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_lptim_isr: [u8; 0x04],
    _reserved_1_lptim_icr: [u8; 0x04],
    _reserved_2_lptim_dier: [u8; 0x04],
    #[doc = "0x0c - LPTIM configuration register"]
    pub lptim_cfgr: LPTIM_CFGR,
    #[doc = "0x10 - LPTIM control register"]
    pub lptim_cr: LPTIM_CR,
    #[doc = "0x14 - LPTIM compare register 1"]
    pub lptim_ccr1: LPTIM_CCR1,
    #[doc = "0x18 - LPTIM autoreload register"]
    pub lptim_arr: LPTIM_ARR,
    #[doc = "0x1c - LPTIM counter register"]
    pub lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - LPTIM repetition register"]
    pub lptim_rcr: LPTIM_RCR,
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    pub lptim_ccmr1: LPTIM_CCMR1,
    _reserved10: [u8; 0x04],
    #[doc = "0x34 - LPTIM compare register 2"]
    pub lptim_ccr2: LPTIM_CCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim_isr_intput(&self) -> &LPTIM_ISR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim_isr_output(&self) -> &LPTIM_ISR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn lptim_icr_intput(&self) -> &LPTIM_ICR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - LPTIM1 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim_icr_output(&self) -> &LPTIM_ICR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn lptim_dier_intput(&self) -> &LPTIM_DIER_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - LPTIM1 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lptim_dier_output(&self) -> &LPTIM_DIER_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "LPTIM_ISR_output (r) register accessor: an alias for `Reg<LPTIM_ISR_OUTPUT_SPEC>`"]
pub type LPTIM_ISR_OUTPUT = crate::Reg<lptim_isr_output::LPTIM_ISR_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod lptim_isr_output;
#[doc = "LPTIM_ISR_intput (r) register accessor: an alias for `Reg<LPTIM_ISR_INTPUT_SPEC>`"]
pub type LPTIM_ISR_INTPUT = crate::Reg<lptim_isr_intput::LPTIM_ISR_INTPUT_SPEC>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod lptim_isr_intput;
#[doc = "LPTIM_ICR_output (w) register accessor: an alias for `Reg<LPTIM_ICR_OUTPUT_SPEC>`"]
pub type LPTIM_ICR_OUTPUT = crate::Reg<lptim_icr_output::LPTIM_ICR_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt clear register \\[alternate\\]"]
pub mod lptim_icr_output;
#[doc = "LPTIM_ICR_intput (w) register accessor: an alias for `Reg<LPTIM_ICR_INTPUT_SPEC>`"]
pub type LPTIM_ICR_INTPUT = crate::Reg<lptim_icr_intput::LPTIM_ICR_INTPUT_SPEC>;
#[doc = "LPTIM interrupt clear register"]
pub mod lptim_icr_intput;
#[doc = "LPTIM_DIER_output (rw) register accessor: an alias for `Reg<LPTIM_DIER_OUTPUT_SPEC>`"]
pub type LPTIM_DIER_OUTPUT = crate::Reg<lptim_dier_output::LPTIM_DIER_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]"]
pub mod lptim_dier_output;
#[doc = "LPTIM_DIER_intput (rw) register accessor: an alias for `Reg<LPTIM_DIER_INTPUT_SPEC>`"]
pub type LPTIM_DIER_INTPUT = crate::Reg<lptim_dier_intput::LPTIM_DIER_INTPUT_SPEC>;
#[doc = "LPTIM interrupt enable register"]
pub mod lptim_dier_intput;
#[doc = "LPTIM_CFGR (rw) register accessor: an alias for `Reg<LPTIM_CFGR_SPEC>`"]
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGR_SPEC>;
#[doc = "LPTIM configuration register"]
pub mod lptim_cfgr;
#[doc = "LPTIM_CR (rw) register accessor: an alias for `Reg<LPTIM_CR_SPEC>`"]
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CR_SPEC>;
#[doc = "LPTIM control register"]
pub mod lptim_cr;
#[doc = "LPTIM_CCR1 (rw) register accessor: an alias for `Reg<LPTIM_CCR1_SPEC>`"]
pub type LPTIM_CCR1 = crate::Reg<lptim_ccr1::LPTIM_CCR1_SPEC>;
#[doc = "LPTIM compare register 1"]
pub mod lptim_ccr1;
#[doc = "LPTIM_ARR (rw) register accessor: an alias for `Reg<LPTIM_ARR_SPEC>`"]
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARR_SPEC>;
#[doc = "LPTIM autoreload register"]
pub mod lptim_arr;
#[doc = "LPTIM_CNT (r) register accessor: an alias for `Reg<LPTIM_CNT_SPEC>`"]
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNT_SPEC>;
#[doc = "LPTIM counter register"]
pub mod lptim_cnt;
#[doc = "LPTIM_RCR (rw) register accessor: an alias for `Reg<LPTIM_RCR_SPEC>`"]
pub type LPTIM_RCR = crate::Reg<lptim_rcr::LPTIM_RCR_SPEC>;
#[doc = "LPTIM repetition register"]
pub mod lptim_rcr;
#[doc = "LPTIM_CCMR1 (rw) register accessor: an alias for `Reg<LPTIM_CCMR1_SPEC>`"]
pub type LPTIM_CCMR1 = crate::Reg<lptim_ccmr1::LPTIM_CCMR1_SPEC>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod lptim_ccmr1;
#[doc = "LPTIM_CCR2 (rw) register accessor: an alias for `Reg<LPTIM_CCR2_SPEC>`"]
pub type LPTIM_CCR2 = crate::Reg<lptim_ccr2::LPTIM_CCR2_SPEC>;
#[doc = "LPTIM compare register 2"]
pub mod lptim_ccr2;
