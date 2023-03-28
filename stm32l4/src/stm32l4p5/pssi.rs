#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PSSI control register"]
    pub cr: CR,
    #[doc = "0x04 - PSSI status register"]
    pub sr: SR,
    #[doc = "0x08 - PSSI raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x0c - PSSI interrupt enable register"]
    pub ier: IER,
    #[doc = "0x10 - PSSI masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x14 - PSSI interrupt clear register"]
    pub icr: ICR,
    _reserved6: [u8; 0x10],
    #[doc = "0x28 - PSSI data register"]
    pub dr: DR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "PSSI control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PSSI status register"]
pub mod sr;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "PSSI raw interrupt status register"]
pub mod ris;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "PSSI interrupt enable register"]
pub mod ier;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "PSSI masked interrupt status register"]
pub mod mis;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "PSSI interrupt clear register"]
pub mod icr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "PSSI data register"]
pub mod dr;
