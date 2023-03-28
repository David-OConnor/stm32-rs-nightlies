#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PSSI control register"]
    pub pssi_cr: PSSI_CR,
    #[doc = "0x04 - PSSI status register"]
    pub pssi_sr: PSSI_SR,
    #[doc = "0x08 - PSSI raw interrupt status register"]
    pub pssi_ris: PSSI_RIS,
    #[doc = "0x0c - PSSI interrupt enable register"]
    pub pssi_ier: PSSI_IER,
    #[doc = "0x10 - PSSI masked interrupt status register"]
    pub pssi_mis: PSSI_MIS,
    #[doc = "0x14 - PSSI interrupt clear register"]
    pub pssi_icr: PSSI_ICR,
    _reserved6: [u8; 0x10],
    #[doc = "0x28 - PSSI data register"]
    pub pssi_dr: PSSI_DR,
}
#[doc = "PSSI_CR (rw) register accessor: an alias for `Reg<PSSI_CR_SPEC>`"]
pub type PSSI_CR = crate::Reg<pssi_cr::PSSI_CR_SPEC>;
#[doc = "PSSI control register"]
pub mod pssi_cr;
#[doc = "PSSI_SR (r) register accessor: an alias for `Reg<PSSI_SR_SPEC>`"]
pub type PSSI_SR = crate::Reg<pssi_sr::PSSI_SR_SPEC>;
#[doc = "PSSI status register"]
pub mod pssi_sr;
#[doc = "PSSI_RIS (r) register accessor: an alias for `Reg<PSSI_RIS_SPEC>`"]
pub type PSSI_RIS = crate::Reg<pssi_ris::PSSI_RIS_SPEC>;
#[doc = "PSSI raw interrupt status register"]
pub mod pssi_ris;
#[doc = "PSSI_IER (rw) register accessor: an alias for `Reg<PSSI_IER_SPEC>`"]
pub type PSSI_IER = crate::Reg<pssi_ier::PSSI_IER_SPEC>;
#[doc = "PSSI interrupt enable register"]
pub mod pssi_ier;
#[doc = "PSSI_MIS (r) register accessor: an alias for `Reg<PSSI_MIS_SPEC>`"]
pub type PSSI_MIS = crate::Reg<pssi_mis::PSSI_MIS_SPEC>;
#[doc = "PSSI masked interrupt status register"]
pub mod pssi_mis;
#[doc = "PSSI_ICR (w) register accessor: an alias for `Reg<PSSI_ICR_SPEC>`"]
pub type PSSI_ICR = crate::Reg<pssi_icr::PSSI_ICR_SPEC>;
#[doc = "PSSI interrupt clear register"]
pub mod pssi_icr;
#[doc = "PSSI_DR (rw) register accessor: an alias for `Reg<PSSI_DR_SPEC>`"]
pub type PSSI_DR = crate::Reg<pssi_dr::PSSI_DR_SPEC>;
#[doc = "PSSI data register"]
pub mod pssi_dr;
