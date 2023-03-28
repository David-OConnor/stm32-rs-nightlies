#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: FLTCR,
    _reserved3: [u8; 0x1c],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: IER,
    #[doc = "0x30 - TAMP status register"]
    pub sr: SR,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: MISR,
    _reserved6: [u8; 0x04],
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: SCR,
    _reserved7: [u8; 0xc0],
    #[doc = "0x100..0x114 - TAMP backup register"]
    pub bkpr: [BKPR; 5],
    _reserved8: [u8; 0x02d8],
    #[doc = "0x3ec - TAMP hardware configuration register 2"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - TAMP hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - EXTI Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - EXTI Size ID register"]
    pub sidr: SIDR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
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
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`"]
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
#[doc = "TAMP backup register"]
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
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
