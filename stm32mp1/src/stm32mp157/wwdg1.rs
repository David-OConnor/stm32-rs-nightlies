#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub wwdg_cr: WWDG_CR,
    #[doc = "0x04 - Configuration register"]
    pub wwdg_cfr: WWDG_CFR,
    #[doc = "0x08 - Status register"]
    pub wwdg_sr: WWDG_SR,
    _reserved3: [u8; 0x03e4],
    #[doc = "0x3f0 - WWDG hardware configuration register"]
    pub wwdg_hwcfgr: WWDG_HWCFGR,
    #[doc = "0x3f4 - WWDG version register"]
    pub wwdg_verr: WWDG_VERR,
    #[doc = "0x3f8 - WWDG ID register"]
    pub wwdg_ipidr: WWDG_IPIDR,
    #[doc = "0x3fc - WWDG size ID register"]
    pub wwdg_sidr: WWDG_SIDR,
}
#[doc = "WWDG_CR (rw) register accessor: an alias for `Reg<WWDG_CR_SPEC>`"]
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CR_SPEC>;
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "WWDG_CFR (rw) register accessor: an alias for `Reg<WWDG_CFR_SPEC>`"]
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFR_SPEC>;
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "WWDG_SR (rw) register accessor: an alias for `Reg<WWDG_SR_SPEC>`"]
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SR_SPEC>;
#[doc = "Status register"]
pub mod wwdg_sr;
#[doc = "WWDG_HWCFGR (r) register accessor: an alias for `Reg<WWDG_HWCFGR_SPEC>`"]
pub type WWDG_HWCFGR = crate::Reg<wwdg_hwcfgr::WWDG_HWCFGR_SPEC>;
#[doc = "WWDG hardware configuration register"]
pub mod wwdg_hwcfgr;
#[doc = "WWDG_VERR (r) register accessor: an alias for `Reg<WWDG_VERR_SPEC>`"]
pub type WWDG_VERR = crate::Reg<wwdg_verr::WWDG_VERR_SPEC>;
#[doc = "WWDG version register"]
pub mod wwdg_verr;
#[doc = "WWDG_IPIDR (r) register accessor: an alias for `Reg<WWDG_IPIDR_SPEC>`"]
pub type WWDG_IPIDR = crate::Reg<wwdg_ipidr::WWDG_IPIDR_SPEC>;
#[doc = "WWDG ID register"]
pub mod wwdg_ipidr;
#[doc = "WWDG_SIDR (r) register accessor: an alias for `Reg<WWDG_SIDR_SPEC>`"]
pub type WWDG_SIDR = crate::Reg<wwdg_sidr::WWDG_SIDR_SPEC>;
#[doc = "WWDG size ID register"]
pub mod wwdg_sidr;
