#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - HSEM register HSEM_R%s HSEM_R31"]
    pub r: [R; 16],
    _reserved1: [u8; 0x40],
    #[doc = "0x80..0xc0 - HSEM Read lock register"]
    pub rlr: [RLR; 16],
    _reserved2: [u8; 0x40],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub c1ier: C1IER,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub c1icr: C1ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub c1isr: C1ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub c1misr: C1MISR,
    #[doc = "0x110 - HSEM Interrupt enable register"]
    pub c2ier: C2IER,
    #[doc = "0x114 - HSEM Interrupt clear register"]
    pub c2icr: C2ICR,
    #[doc = "0x118 - HSEM Interrupt status register"]
    pub c2isr: C2ISR,
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    pub c2misr: C2MISR,
    _reserved10: [u8; 0x20],
    #[doc = "0x140 - HSEM Clear register"]
    pub cr: CR,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub keyr: KEYR,
}
#[doc = "R (rw) register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "HSEM register HSEM_R%s HSEM_R31"]
pub mod r;
#[doc = "RLR (r) register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr;
#[doc = "C1IER (rw) register accessor: an alias for `Reg<C1IER_SPEC>`"]
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier;
#[doc = "C1ICR (rw) register accessor: an alias for `Reg<C1ICR_SPEC>`"]
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "C1ISR (r) register accessor: an alias for `Reg<C1ISR_SPEC>`"]
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "C1MISR (r) register accessor: an alias for `Reg<C1MISR_SPEC>`"]
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "C2IER (rw) register accessor: an alias for `Reg<C2IER_SPEC>`"]
pub type C2IER = crate::Reg<c2ier::C2IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod c2ier;
#[doc = "C2ICR (rw) register accessor: an alias for `Reg<C2ICR_SPEC>`"]
pub type C2ICR = crate::Reg<c2icr::C2ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod c2icr;
#[doc = "C2ISR (r) register accessor: an alias for `Reg<C2ISR_SPEC>`"]
pub type C2ISR = crate::Reg<c2isr::C2ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod c2isr;
#[doc = "C2MISR (r) register accessor: an alias for `Reg<C2MISR_SPEC>`"]
pub type C2MISR = crate::Reg<c2misr::C2MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c2misr;
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR (rw) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
