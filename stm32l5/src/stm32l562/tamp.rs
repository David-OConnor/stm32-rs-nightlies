#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: FLTCR,
    #[doc = "0x10 - TAMP active tamper control register 1"]
    pub atcr1: ATCR1,
    #[doc = "0x14 - TAMP active tamper seed register"]
    pub atseedr: ATSEEDR,
    #[doc = "0x18 - TAMP active tamper output register"]
    pub ator: ATOR,
    #[doc = "0x1c - TAMP active tamper control register 2"]
    pub atcr2: ATCR2,
    #[doc = "0x20 - TAMP secure mode register"]
    pub smcr: SMCR,
    #[doc = "0x24 - TAMP privilege mode control register"]
    pub privcr: PRIVCR,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: IER,
    #[doc = "0x30 - TAMP status register"]
    pub sr: SR,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub smisr: SMISR,
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: SCR,
    #[doc = "0x40 - TAMP monotonic counter register"]
    pub countr: COUNTR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x50 - TAMP configuration register"]
    pub cfgr: CFGR,
    _reserved17: [u8; 0xac],
    #[doc = "0x100..0x180 - TAMP backup register"]
    pub bkpr: [BKPR; 32],
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "ATCR1 (rw) register accessor: an alias for `Reg<ATCR1_SPEC>`"]
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
#[doc = "TAMP active tamper control register 1"]
pub mod atcr1;
#[doc = "ATSEEDR (w) register accessor: an alias for `Reg<ATSEEDR_SPEC>`"]
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
#[doc = "TAMP active tamper seed register"]
pub mod atseedr;
#[doc = "ATOR (r) register accessor: an alias for `Reg<ATOR_SPEC>`"]
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
#[doc = "TAMP active tamper output register"]
pub mod ator;
#[doc = "ATCR2 (rw) register accessor: an alias for `Reg<ATCR2_SPEC>`"]
pub type ATCR2 = crate::Reg<atcr2::ATCR2_SPEC>;
#[doc = "TAMP active tamper control register 2"]
pub mod atcr2;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "TAMP secure mode register"]
pub mod smcr;
#[doc = "PRIVCR (rw) register accessor: an alias for `Reg<PRIVCR_SPEC>`"]
pub type PRIVCR = crate::Reg<privcr::PRIVCR_SPEC>;
#[doc = "TAMP privilege mode control register"]
pub mod privcr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`"]
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
#[doc = "TAMP secure masked interrupt status register"]
pub mod smisr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "COUNTR (r) register accessor: an alias for `Reg<COUNTR_SPEC>`"]
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
#[doc = "TAMP monotonic counter register"]
pub mod countr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "TAMP configuration register"]
pub mod cfgr;
#[doc = "BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`"]
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkpr;
