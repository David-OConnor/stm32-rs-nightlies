#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub cr1: CR1,
    #[doc = "0x04 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub cr2: CR2,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub fltcr: FLTCR,
    #[doc = "0x10 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub atcr1: ATCR1,
    #[doc = "0x14 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub atseedr: ATSEEDR,
    #[doc = "0x18 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub ator: ATOR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub smcr: SMCR,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub ier: IER,
    #[doc = "0x30 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub sr: SR,
    #[doc = "0x34 - TAMP non-secure masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub smisr: SMISR,
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: SCR,
    #[doc = "0x40 - TAMP monotonic counter register"]
    pub countr: COUNTR,
    _reserved13: [u8; 0x0c],
    #[doc = "0x50 - TAMP configuration register"]
    pub cfgr: CFGR,
    _reserved14: [u8; 0xac],
    #[doc = "0x100..0x180 - TAMP backup %s register"]
    pub bkpr: [BKPR; 32],
    _reserved15: [u8; 0x026c],
    #[doc = "0x3ec - TAMP hardware configuration register 2"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - TAMP hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - TAMP version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - TAMP identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - TAMP size identification register"]
    pub sidr: SIDR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod cr2;
#[doc = "FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod fltcr;
#[doc = "ATCR1 (rw) register accessor: an alias for `Reg<ATCR1_SPEC>`"]
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod atcr1;
#[doc = "ATSEEDR (w) register accessor: an alias for `Reg<ATSEEDR_SPEC>`"]
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod atseedr;
#[doc = "ATOR (r) register accessor: an alias for `Reg<ATOR_SPEC>`"]
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod ator;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "This register can be written only when the APB access is secure."]
pub mod smcr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod ier;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "TAMP non-secure masked interrupt status register"]
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
#[doc = "TAMP backup %s register"]
pub mod bkpr;
#[doc = "HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
#[doc = "TAMP hardware configuration register 2"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
#[doc = "TAMP hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "TAMP version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "TAMP identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "TAMP size identification register"]
pub mod sidr;
