#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub itcmcr: ITCMCR,
    #[doc = "0x04 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub dtcmcr: DTCMCR,
    #[doc = "0x08 - AHBP Control register"]
    pub ahbpcr: AHBPCR,
    #[doc = "0x0c - Auxiliary Cache Control register"]
    pub cacr: CACR,
    #[doc = "0x10 - AHB Slave Control register"]
    pub ahbscr: AHBSCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Auxiliary Bus Fault Status register"]
    pub abfsr: ABFSR,
}
#[doc = "ITCMCR (rw) register accessor: an alias for `Reg<ITCMCR_SPEC>`"]
pub type ITCMCR = crate::Reg<itcmcr::ITCMCR_SPEC>;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod itcmcr;
#[doc = "DTCMCR (rw) register accessor: an alias for `Reg<DTCMCR_SPEC>`"]
pub type DTCMCR = crate::Reg<dtcmcr::DTCMCR_SPEC>;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod dtcmcr;
#[doc = "AHBPCR (rw) register accessor: an alias for `Reg<AHBPCR_SPEC>`"]
pub type AHBPCR = crate::Reg<ahbpcr::AHBPCR_SPEC>;
#[doc = "AHBP Control register"]
pub mod ahbpcr;
#[doc = "CACR (rw) register accessor: an alias for `Reg<CACR_SPEC>`"]
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
#[doc = "Auxiliary Cache Control register"]
pub mod cacr;
#[doc = "AHBSCR (rw) register accessor: an alias for `Reg<AHBSCR_SPEC>`"]
pub type AHBSCR = crate::Reg<ahbscr::AHBSCR_SPEC>;
#[doc = "AHB Slave Control register"]
pub mod ahbscr;
#[doc = "ABFSR (rw) register accessor: an alias for `Reg<ABFSR_SPEC>`"]
pub type ABFSR = crate::Reg<abfsr::ABFSR_SPEC>;
#[doc = "Auxiliary Bus Fault Status register"]
pub mod abfsr;
