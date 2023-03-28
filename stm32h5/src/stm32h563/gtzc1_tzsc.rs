#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GTZC1 TZSC control register"]
    pub gtzc1_tzsc_cr: GTZC1_TZSC_CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - GTZC1 TZSC secure configuration register 1"]
    pub gtzc1_tzsc_seccfgr1: GTZC1_TZSC_SECCFGR1,
    #[doc = "0x14 - GTZC1 TZSC secure configuration register 2"]
    pub gtzc1_tzsc_seccfgr2: GTZC1_TZSC_SECCFGR2,
    #[doc = "0x18 - GTZC1 TZSC secure configuration register 3"]
    pub gtzc1_tzsc_seccfgr3: GTZC1_TZSC_SECCFGR3,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    pub gtzc1_tzsc_privcfgr1: GTZC1_TZSC_PRIVCFGR1,
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    pub gtzc1_tzsc_privcfgr2: GTZC1_TZSC_PRIVCFGR2,
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    pub gtzc1_tzsc_privcfgr3: GTZC1_TZSC_PRIVCFGR3,
    _reserved7: [u8; 0x14],
    #[doc = "0x40 - GTZC1 TZSC memory 1 sub-region A watermark configuration register"]
    pub gtzc1_tzsc_mpcwm1acfgr: GTZC1_TZSC_MPCWM1ACFGR,
    #[doc = "0x44 - GTZC1 TZSC memory 1 sub-region A watermark register"]
    pub gtzc1_tzsc_mpcwm1ar: GTZC1_TZSC_MPCWM1AR,
    #[doc = "0x48 - GTZC1 TZSC memory 1 sub-region B watermark configuration register"]
    pub gtzc1_tzsc_mpcwm1bcfgr: GTZC1_TZSC_MPCWM1BCFGR,
    #[doc = "0x4c - GTZC1 TZSC memory 1 sub-region B watermark register"]
    pub gtzc1_tzsc_mpcwm1br: GTZC1_TZSC_MPCWM1BR,
    #[doc = "0x50 - GTZC1 TZSC memory 2 sub-region A watermark configuration register"]
    pub gtzc1_tzsc_mpcwm2acfgr: GTZC1_TZSC_MPCWM2ACFGR,
    #[doc = "0x54 - GTZC1 TZSC memory 2 sub-region A watermark register"]
    pub gtzc1_tzsc_mpcwm2ar: GTZC1_TZSC_MPCWM2AR,
    #[doc = "0x58 - GTZC1 TZSC memory 2 sub-region B watermark configuration register"]
    pub gtzc1_tzsc_mpcwm2bcfgr: GTZC1_TZSC_MPCWM2BCFGR,
    #[doc = "0x5c - GTZC1 TZSC memory 2 sub-region B watermark register"]
    pub gtzc1_tzsc_mpcwm2br: GTZC1_TZSC_MPCWM2BR,
    #[doc = "0x60 - GTZC1 TZSC memory 3 sub-region A watermark configuration register"]
    pub gtzc1_tzsc_mpcwm3acfgr: GTZC1_TZSC_MPCWM3ACFGR,
    #[doc = "0x64 - GTZC1 TZSC memory 3 sub-region A watermark register"]
    pub gtzc1_tzsc_mpcwm3ar: GTZC1_TZSC_MPCWM3AR,
    #[doc = "0x68 - GTZC1 TZSC memory 3 sub-region B watermark configuration register"]
    pub gtzc1_tzsc_mpcwm3bcfgr: GTZC1_TZSC_MPCWM3BCFGR,
    #[doc = "0x6c - GTZC1 TZSC memory 3 sub-region B watermark register"]
    pub gtzc1_tzsc_mpcwm3br: GTZC1_TZSC_MPCWM3BR,
    #[doc = "0x70 - GTZC1 TZSC memory 4 sub-region A watermark configuration register"]
    pub gtzc1_tzsc_mpcwm4acfgr: GTZC1_TZSC_MPCWM4ACFGR,
    #[doc = "0x74 - GTZC1 TZSC memory 4 sub-region A watermark register"]
    pub gtzc1_tzsc_mpcwm4ar: GTZC1_TZSC_MPCWM4AR,
    #[doc = "0x78 - GTZC1 TZSC memory 4 sub-region B watermark configuration register"]
    pub gtzc1_tzsc_mpcwm4bcfgr: GTZC1_TZSC_MPCWM4BCFGR,
    #[doc = "0x7c - GTZC1 TZSC memory 4 sub-region B watermark register"]
    pub gtzc1_tzsc_mpcwm4br: GTZC1_TZSC_MPCWM4BR,
}
#[doc = "GTZC1_TZSC_CR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_CR_SPEC>`"]
pub type GTZC1_TZSC_CR = crate::Reg<gtzc1_tzsc_cr::GTZC1_TZSC_CR_SPEC>;
#[doc = "GTZC1 TZSC control register"]
pub mod gtzc1_tzsc_cr;
#[doc = "GTZC1_TZSC_SECCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_SECCFGR1_SPEC>`"]
pub type GTZC1_TZSC_SECCFGR1 = crate::Reg<gtzc1_tzsc_seccfgr1::GTZC1_TZSC_SECCFGR1_SPEC>;
#[doc = "GTZC1 TZSC secure configuration register 1"]
pub mod gtzc1_tzsc_seccfgr1;
#[doc = "GTZC1_TZSC_SECCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_SECCFGR2_SPEC>`"]
pub type GTZC1_TZSC_SECCFGR2 = crate::Reg<gtzc1_tzsc_seccfgr2::GTZC1_TZSC_SECCFGR2_SPEC>;
#[doc = "GTZC1 TZSC secure configuration register 2"]
pub mod gtzc1_tzsc_seccfgr2;
#[doc = "GTZC1_TZSC_SECCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_SECCFGR3_SPEC>`"]
pub type GTZC1_TZSC_SECCFGR3 = crate::Reg<gtzc1_tzsc_seccfgr3::GTZC1_TZSC_SECCFGR3_SPEC>;
#[doc = "GTZC1 TZSC secure configuration register 3"]
pub mod gtzc1_tzsc_seccfgr3;
#[doc = "GTZC1_TZSC_PRIVCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR1_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR1 = crate::Reg<gtzc1_tzsc_privcfgr1::GTZC1_TZSC_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod gtzc1_tzsc_privcfgr1;
#[doc = "GTZC1_TZSC_PRIVCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR2_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR2 = crate::Reg<gtzc1_tzsc_privcfgr2::GTZC1_TZSC_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod gtzc1_tzsc_privcfgr2;
#[doc = "GTZC1_TZSC_PRIVCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR3_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR3 = crate::Reg<gtzc1_tzsc_privcfgr3::GTZC1_TZSC_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod gtzc1_tzsc_privcfgr3;
#[doc = "GTZC1_TZSC_MPCWM1ACFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM1ACFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM1ACFGR = crate::Reg<gtzc1_tzsc_mpcwm1acfgr::GTZC1_TZSC_MPCWM1ACFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 1 sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm1acfgr;
#[doc = "GTZC1_TZSC_MPCWM1AR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM1AR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM1AR = crate::Reg<gtzc1_tzsc_mpcwm1ar::GTZC1_TZSC_MPCWM1AR_SPEC>;
#[doc = "GTZC1 TZSC memory 1 sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm1ar;
#[doc = "GTZC1_TZSC_MPCWM1BCFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM1BCFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM1BCFGR = crate::Reg<gtzc1_tzsc_mpcwm1bcfgr::GTZC1_TZSC_MPCWM1BCFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 1 sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm1bcfgr;
#[doc = "GTZC1_TZSC_MPCWM1BR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM1BR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM1BR = crate::Reg<gtzc1_tzsc_mpcwm1br::GTZC1_TZSC_MPCWM1BR_SPEC>;
#[doc = "GTZC1 TZSC memory 1 sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm1br;
#[doc = "GTZC1_TZSC_MPCWM2ACFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM2ACFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM2ACFGR = crate::Reg<gtzc1_tzsc_mpcwm2acfgr::GTZC1_TZSC_MPCWM2ACFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 2 sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm2acfgr;
#[doc = "GTZC1_TZSC_MPCWM2AR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM2AR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM2AR = crate::Reg<gtzc1_tzsc_mpcwm2ar::GTZC1_TZSC_MPCWM2AR_SPEC>;
#[doc = "GTZC1 TZSC memory 2 sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm2ar;
#[doc = "GTZC1_TZSC_MPCWM2BCFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM2BCFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM2BCFGR = crate::Reg<gtzc1_tzsc_mpcwm2bcfgr::GTZC1_TZSC_MPCWM2BCFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 2 sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm2bcfgr;
#[doc = "GTZC1_TZSC_MPCWM2BR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM2BR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM2BR = crate::Reg<gtzc1_tzsc_mpcwm2br::GTZC1_TZSC_MPCWM2BR_SPEC>;
#[doc = "GTZC1 TZSC memory 2 sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm2br;
#[doc = "GTZC1_TZSC_MPCWM3ACFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM3ACFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM3ACFGR = crate::Reg<gtzc1_tzsc_mpcwm3acfgr::GTZC1_TZSC_MPCWM3ACFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 3 sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm3acfgr;
#[doc = "GTZC1_TZSC_MPCWM3AR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM3AR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM3AR = crate::Reg<gtzc1_tzsc_mpcwm3ar::GTZC1_TZSC_MPCWM3AR_SPEC>;
#[doc = "GTZC1 TZSC memory 3 sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm3ar;
#[doc = "GTZC1_TZSC_MPCWM3BCFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM3BCFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM3BCFGR = crate::Reg<gtzc1_tzsc_mpcwm3bcfgr::GTZC1_TZSC_MPCWM3BCFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 3 sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm3bcfgr;
#[doc = "GTZC1_TZSC_MPCWM3BR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM3BR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM3BR = crate::Reg<gtzc1_tzsc_mpcwm3br::GTZC1_TZSC_MPCWM3BR_SPEC>;
#[doc = "GTZC1 TZSC memory 3 sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm3br;
#[doc = "GTZC1_TZSC_MPCWM4ACFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4ACFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4ACFGR = crate::Reg<gtzc1_tzsc_mpcwm4acfgr::GTZC1_TZSC_MPCWM4ACFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4acfgr;
#[doc = "GTZC1_TZSC_MPCWM4AR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4AR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4AR = crate::Reg<gtzc1_tzsc_mpcwm4ar::GTZC1_TZSC_MPCWM4AR_SPEC>;
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm4ar;
#[doc = "GTZC1_TZSC_MPCWM4BCFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4BCFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4BCFGR = crate::Reg<gtzc1_tzsc_mpcwm4bcfgr::GTZC1_TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4bcfgr;
#[doc = "GTZC1_TZSC_MPCWM4BR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4BR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4BR = crate::Reg<gtzc1_tzsc_mpcwm4br::GTZC1_TZSC_MPCWM4BR_SPEC>;
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm4br;
