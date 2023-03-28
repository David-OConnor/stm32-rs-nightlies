#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s HSEM_R31"]
    pub r: [R; 32],
    #[doc = "0x80..0x100 - HSEM Read lock register"]
    pub rlr: [RLR; 32],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub misr: MISR,
    _reserved6: [u8; 0x30],
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
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod ier;
#[doc = "ICR (r) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod icr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod isr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod misr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR (rw) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
