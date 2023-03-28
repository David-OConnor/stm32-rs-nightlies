#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - SBS temporal isolation control register"]
    pub sbs_hdplcr: SBS_HDPLCR,
    #[doc = "0x14 - SBS temporal isolation status register"]
    pub sbs_hdplsr: SBS_HDPLSR,
    #[doc = "0x18 - SBS next HDPL control register"]
    pub sbs_nexthdplcr: SBS_NEXTHDPLCR,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - SBS debug control register"]
    pub sbs_dbgcr: SBS_DBGCR,
    #[doc = "0x24 - SBS debug lock register"]
    pub sbs_dbglockr: SBS_DBGLOCKR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x34 - SBS RSS command register"]
    pub sbs_rsscmdr: SBS_RSSCMDR,
    _reserved6: [u8; 0x68],
    #[doc = "0xa0 - SBS EPOCH selection control register"]
    pub sbs_epochselcr: SBS_EPOCHSELCR,
    _reserved7: [u8; 0x1c],
    #[doc = "0xc0 - SBS security mode configuration control register"]
    pub sbs_seccfgr: SBS_SECCFGR,
    _reserved8: [u8; 0x3c],
    #[doc = "0x100 - SBS product mode and configuration register"]
    pub sbs_pmcr: SBS_PMCR,
    #[doc = "0x104 - SBS FPU interrupt mask register"]
    pub sbs_fpuimr: SBS_FPUIMR,
    #[doc = "0x108 - SBS memory erase status register"]
    pub sbs_mesr: SBS_MESR,
    _reserved11: [u8; 0x04],
    #[doc = "0x110 - SBS compensation cell for I/Os control and status register"]
    pub sbs_cccsr: SBS_CCCSR,
    #[doc = "0x114 - SBS compensation cell for I/Os value register"]
    pub sbs_ccvalr: SBS_CCVALR,
    #[doc = "0x118 - SBS compensation cell for I/Os software code register"]
    pub sbs_ccswcr: SBS_CCSWCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x120 - SBS Class B register"]
    pub sbs_cfgr2: SBS_CFGR2,
    _reserved15: [u8; 0x20],
    #[doc = "0x144 - SBS CPU non-secure lock register"]
    pub sbs_cnslckr: SBS_CNSLCKR,
    #[doc = "0x148 - SBS CPU secure lock register"]
    pub sbs_cslckr: SBS_CSLCKR,
    #[doc = "0x14c - SBS flift ECC NMI mask register"]
    pub sbs_eccnmir: SBS_ECCNMIR,
}
#[doc = "SBS_HDPLCR (rw) register accessor: an alias for `Reg<SBS_HDPLCR_SPEC>`"]
pub type SBS_HDPLCR = crate::Reg<sbs_hdplcr::SBS_HDPLCR_SPEC>;
#[doc = "SBS temporal isolation control register"]
pub mod sbs_hdplcr;
#[doc = "SBS_HDPLSR (r) register accessor: an alias for `Reg<SBS_HDPLSR_SPEC>`"]
pub type SBS_HDPLSR = crate::Reg<sbs_hdplsr::SBS_HDPLSR_SPEC>;
#[doc = "SBS temporal isolation status register"]
pub mod sbs_hdplsr;
#[doc = "SBS_NEXTHDPLCR (rw) register accessor: an alias for `Reg<SBS_NEXTHDPLCR_SPEC>`"]
pub type SBS_NEXTHDPLCR = crate::Reg<sbs_nexthdplcr::SBS_NEXTHDPLCR_SPEC>;
#[doc = "SBS next HDPL control register"]
pub mod sbs_nexthdplcr;
#[doc = "SBS_DBGCR (rw) register accessor: an alias for `Reg<SBS_DBGCR_SPEC>`"]
pub type SBS_DBGCR = crate::Reg<sbs_dbgcr::SBS_DBGCR_SPEC>;
#[doc = "SBS debug control register"]
pub mod sbs_dbgcr;
#[doc = "SBS_DBGLOCKR (rw) register accessor: an alias for `Reg<SBS_DBGLOCKR_SPEC>`"]
pub type SBS_DBGLOCKR = crate::Reg<sbs_dbglockr::SBS_DBGLOCKR_SPEC>;
#[doc = "SBS debug lock register"]
pub mod sbs_dbglockr;
#[doc = "SBS_RSSCMDR (rw) register accessor: an alias for `Reg<SBS_RSSCMDR_SPEC>`"]
pub type SBS_RSSCMDR = crate::Reg<sbs_rsscmdr::SBS_RSSCMDR_SPEC>;
#[doc = "SBS RSS command register"]
pub mod sbs_rsscmdr;
#[doc = "SBS_EPOCHSELCR (rw) register accessor: an alias for `Reg<SBS_EPOCHSELCR_SPEC>`"]
pub type SBS_EPOCHSELCR = crate::Reg<sbs_epochselcr::SBS_EPOCHSELCR_SPEC>;
#[doc = "SBS EPOCH selection control register"]
pub mod sbs_epochselcr;
#[doc = "SBS_SECCFGR (rw) register accessor: an alias for `Reg<SBS_SECCFGR_SPEC>`"]
pub type SBS_SECCFGR = crate::Reg<sbs_seccfgr::SBS_SECCFGR_SPEC>;
#[doc = "SBS security mode configuration control register"]
pub mod sbs_seccfgr;
#[doc = "SBS_PMCR (rw) register accessor: an alias for `Reg<SBS_PMCR_SPEC>`"]
pub type SBS_PMCR = crate::Reg<sbs_pmcr::SBS_PMCR_SPEC>;
#[doc = "SBS product mode and configuration register"]
pub mod sbs_pmcr;
#[doc = "SBS_FPUIMR (rw) register accessor: an alias for `Reg<SBS_FPUIMR_SPEC>`"]
pub type SBS_FPUIMR = crate::Reg<sbs_fpuimr::SBS_FPUIMR_SPEC>;
#[doc = "SBS FPU interrupt mask register"]
pub mod sbs_fpuimr;
#[doc = "SBS_MESR (rw) register accessor: an alias for `Reg<SBS_MESR_SPEC>`"]
pub type SBS_MESR = crate::Reg<sbs_mesr::SBS_MESR_SPEC>;
#[doc = "SBS memory erase status register"]
pub mod sbs_mesr;
#[doc = "SBS_CCCSR (rw) register accessor: an alias for `Reg<SBS_CCCSR_SPEC>`"]
pub type SBS_CCCSR = crate::Reg<sbs_cccsr::SBS_CCCSR_SPEC>;
#[doc = "SBS compensation cell for I/Os control and status register"]
pub mod sbs_cccsr;
#[doc = "SBS_CCVALR (r) register accessor: an alias for `Reg<SBS_CCVALR_SPEC>`"]
pub type SBS_CCVALR = crate::Reg<sbs_ccvalr::SBS_CCVALR_SPEC>;
#[doc = "SBS compensation cell for I/Os value register"]
pub mod sbs_ccvalr;
#[doc = "SBS_CCSWCR (rw) register accessor: an alias for `Reg<SBS_CCSWCR_SPEC>`"]
pub type SBS_CCSWCR = crate::Reg<sbs_ccswcr::SBS_CCSWCR_SPEC>;
#[doc = "SBS compensation cell for I/Os software code register"]
pub mod sbs_ccswcr;
#[doc = "SBS_CFGR2 (rw) register accessor: an alias for `Reg<SBS_CFGR2_SPEC>`"]
pub type SBS_CFGR2 = crate::Reg<sbs_cfgr2::SBS_CFGR2_SPEC>;
#[doc = "SBS Class B register"]
pub mod sbs_cfgr2;
#[doc = "SBS_CNSLCKR (rw) register accessor: an alias for `Reg<SBS_CNSLCKR_SPEC>`"]
pub type SBS_CNSLCKR = crate::Reg<sbs_cnslckr::SBS_CNSLCKR_SPEC>;
#[doc = "SBS CPU non-secure lock register"]
pub mod sbs_cnslckr;
#[doc = "SBS_CSLCKR (rw) register accessor: an alias for `Reg<SBS_CSLCKR_SPEC>`"]
pub type SBS_CSLCKR = crate::Reg<sbs_cslckr::SBS_CSLCKR_SPEC>;
#[doc = "SBS CPU secure lock register"]
pub mod sbs_cslckr;
#[doc = "SBS_ECCNMIR (rw) register accessor: an alias for `Reg<SBS_ECCNMIR_SPEC>`"]
pub type SBS_ECCNMIR = crate::Reg<sbs_eccnmir::SBS_ECCNMIR_SPEC>;
#[doc = "SBS flift ECC NMI mask register"]
pub mod sbs_eccnmir;
