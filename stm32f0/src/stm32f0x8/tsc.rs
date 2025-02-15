#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x0c - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10 - I/O hysteresis control register"]
    pub iohcr: IOHCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - I/O analog switch control register"]
    pub ioascr: IOASCR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - I/O sampling control register"]
    pub ioscr: IOSCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - I/O channel control register"]
    pub ioccr: IOCCR,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - I/O group control status register"]
    pub iogcsr: IOGCSR,
    #[doc = "0x34..0x4c - I/O group x counter register"]
    pub iogcr: [IOGCR; 6],
}
impl RegisterBlock {
    #[doc = "0x34 - I/O group x counter register"]
    #[inline(always)]
    pub fn iog1cr(&self) -> &IOGCR {
        &self.iogcr[0]
    }
    #[doc = "0x38 - I/O group x counter register"]
    #[inline(always)]
    pub fn iog2cr(&self) -> &IOGCR {
        &self.iogcr[1]
    }
    #[doc = "0x3c - I/O group x counter register"]
    #[inline(always)]
    pub fn iog3cr(&self) -> &IOGCR {
        &self.iogcr[2]
    }
    #[doc = "0x40 - I/O group x counter register"]
    #[inline(always)]
    pub fn iog4cr(&self) -> &IOGCR {
        &self.iogcr[3]
    }
    #[doc = "0x44 - I/O group x counter register"]
    #[inline(always)]
    pub fn iog5cr(&self) -> &IOGCR {
        &self.iogcr[4]
    }
    #[doc = "0x48 - I/O group x counter register"]
    #[inline(always)]
    pub fn iog6cr(&self) -> &IOGCR {
        &self.iogcr[5]
    }
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IOHCR (rw) register accessor: an alias for `Reg<IOHCR_SPEC>`"]
pub type IOHCR = crate::Reg<iohcr::IOHCR_SPEC>;
#[doc = "I/O hysteresis control register"]
pub mod iohcr;
#[doc = "IOASCR (rw) register accessor: an alias for `Reg<IOASCR_SPEC>`"]
pub type IOASCR = crate::Reg<ioascr::IOASCR_SPEC>;
#[doc = "I/O analog switch control register"]
pub mod ioascr;
#[doc = "IOSCR (rw) register accessor: an alias for `Reg<IOSCR_SPEC>`"]
pub type IOSCR = crate::Reg<ioscr::IOSCR_SPEC>;
#[doc = "I/O sampling control register"]
pub mod ioscr;
#[doc = "IOCCR (rw) register accessor: an alias for `Reg<IOCCR_SPEC>`"]
pub type IOCCR = crate::Reg<ioccr::IOCCR_SPEC>;
#[doc = "I/O channel control register"]
pub mod ioccr;
#[doc = "IOGCSR (rw) register accessor: an alias for `Reg<IOGCSR_SPEC>`"]
pub type IOGCSR = crate::Reg<iogcsr::IOGCSR_SPEC>;
#[doc = "I/O group control status register"]
pub mod iogcsr;
#[doc = "IOGCR (r) register accessor: an alias for `Reg<IOGCR_SPEC>`"]
pub type IOGCR = crate::Reg<iogcr::IOGCR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iogcr;
