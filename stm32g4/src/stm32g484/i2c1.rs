#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Own address register 1"]
    pub oar1: OAR1,
    #[doc = "0x0c - Own address register 2"]
    pub oar2: OAR2,
    #[doc = "0x10 - Timing register"]
    pub timingr: TIMINGR,
    #[doc = "0x14 - Status register 1"]
    pub timeoutr: TIMEOUTR,
    #[doc = "0x18 - Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x1c - Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x20 - PEC register"]
    pub pecr: PECR,
    #[doc = "0x24 - Receive data register"]
    pub rxdr: RXDR,
    #[doc = "0x28 - Transmit data register"]
    pub txdr: TXDR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "OAR1 (rw) register accessor: an alias for `Reg<OAR1_SPEC>`"]
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "OAR2 (rw) register accessor: an alias for `Reg<OAR2_SPEC>`"]
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "TIMINGR (rw) register accessor: an alias for `Reg<TIMINGR_SPEC>`"]
pub type TIMINGR = crate::Reg<timingr::TIMINGR_SPEC>;
#[doc = "Timing register"]
pub mod timingr;
#[doc = "TIMEOUTR (rw) register accessor: an alias for `Reg<TIMEOUTR_SPEC>`"]
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTR_SPEC>;
#[doc = "Status register 1"]
pub mod timeoutr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt and Status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "PECR (r) register accessor: an alias for `Reg<PECR_SPEC>`"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "PEC register"]
pub mod pecr;
#[doc = "RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "Receive data register"]
pub mod rxdr;
#[doc = "TXDR (rw) register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "Transmit data register"]
pub mod txdr;
