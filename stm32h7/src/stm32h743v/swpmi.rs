#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    pub cr: CR,
    #[doc = "0x04 - SWPMI Bitrate register"]
    pub brr: BRR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    pub icr: ICR,
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    pub ier: IER,
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    pub rfl: RFL,
    #[doc = "0x1c - SWPMI Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x20 - SWPMI Receive data register"]
    pub rdr: RDR,
    #[doc = "0x24 - SWPMI Option register"]
    pub or: OR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "RFL (r) register accessor: an alias for `Reg<RFL_SPEC>`"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "SWPMI Receive data register"]
pub mod rdr;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "SWPMI Option register"]
pub mod or;
