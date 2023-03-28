#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    pub sai_gcr: SAI_GCR,
    #[doc = "0x04 - Configuration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - Configuration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - This register has no meaning in and SPDIF audio protocol"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - This register has no meaning in and SPDIF audio protocol"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - Interrupt mask register"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - Status register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - Clear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - Data register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - Configuration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - Configuration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - This register has no meaning in and SPDIF audio protocol"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - This register has no meaning in and SPDIF audio protocol"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - Interrupt mask register"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - Status register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - Clear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - Data register"]
    pub sai_bdr: SAI_BDR,
    #[doc = "0x44 - PDM control register"]
    pub sai_pdmcr: SAI_PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub sai_pdmdly: SAI_PDMDLY,
    _reserved19: [u8; 0x03a4],
    #[doc = "0x3f0 - SAI hardware configuration register"]
    pub sai_hwcfgr: SAI_HWCFGR,
    #[doc = "0x3f4 - SAI version register"]
    pub sai_verr: SAI_VERR,
    #[doc = "0x3f8 - SAI identification register"]
    pub sai_ipidr: SAI_IPIDR,
    #[doc = "0x3fc - SAI size identification register"]
    pub sai_sidr: SAI_SIDR,
}
#[doc = "SAI_GCR (rw) register accessor: an alias for `Reg<SAI_GCR_SPEC>`"]
pub type SAI_GCR = crate::Reg<sai_gcr::SAI_GCR_SPEC>;
#[doc = "Global configuration register"]
pub mod sai_gcr;
#[doc = "SAI_ACR1 (rw) register accessor: an alias for `Reg<SAI_ACR1_SPEC>`"]
pub type SAI_ACR1 = crate::Reg<sai_acr1::SAI_ACR1_SPEC>;
#[doc = "Configuration register 1"]
pub mod sai_acr1;
#[doc = "SAI_ACR2 (rw) register accessor: an alias for `Reg<SAI_ACR2_SPEC>`"]
pub type SAI_ACR2 = crate::Reg<sai_acr2::SAI_ACR2_SPEC>;
#[doc = "Configuration register 2"]
pub mod sai_acr2;
#[doc = "SAI_AFRCR (rw) register accessor: an alias for `Reg<SAI_AFRCR_SPEC>`"]
pub type SAI_AFRCR = crate::Reg<sai_afrcr::SAI_AFRCR_SPEC>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_afrcr;
#[doc = "SAI_ASLOTR (rw) register accessor: an alias for `Reg<SAI_ASLOTR_SPEC>`"]
pub type SAI_ASLOTR = crate::Reg<sai_aslotr::SAI_ASLOTR_SPEC>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_aslotr;
#[doc = "SAI_AIM (rw) register accessor: an alias for `Reg<SAI_AIM_SPEC>`"]
pub type SAI_AIM = crate::Reg<sai_aim::SAI_AIM_SPEC>;
#[doc = "Interrupt mask register"]
pub mod sai_aim;
#[doc = "SAI_ASR (r) register accessor: an alias for `Reg<SAI_ASR_SPEC>`"]
pub type SAI_ASR = crate::Reg<sai_asr::SAI_ASR_SPEC>;
#[doc = "Status register"]
pub mod sai_asr;
#[doc = "SAI_ACLRFR (w) register accessor: an alias for `Reg<SAI_ACLRFR_SPEC>`"]
pub type SAI_ACLRFR = crate::Reg<sai_aclrfr::SAI_ACLRFR_SPEC>;
#[doc = "Clear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI_ADR (rw) register accessor: an alias for `Reg<SAI_ADR_SPEC>`"]
pub type SAI_ADR = crate::Reg<sai_adr::SAI_ADR_SPEC>;
#[doc = "Data register"]
pub mod sai_adr;
#[doc = "SAI_BCR1 (rw) register accessor: an alias for `Reg<SAI_BCR1_SPEC>`"]
pub type SAI_BCR1 = crate::Reg<sai_bcr1::SAI_BCR1_SPEC>;
#[doc = "Configuration register 1"]
pub mod sai_bcr1;
#[doc = "SAI_BCR2 (rw) register accessor: an alias for `Reg<SAI_BCR2_SPEC>`"]
pub type SAI_BCR2 = crate::Reg<sai_bcr2::SAI_BCR2_SPEC>;
#[doc = "Configuration register 2"]
pub mod sai_bcr2;
#[doc = "SAI_BFRCR (rw) register accessor: an alias for `Reg<SAI_BFRCR_SPEC>`"]
pub type SAI_BFRCR = crate::Reg<sai_bfrcr::SAI_BFRCR_SPEC>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bfrcr;
#[doc = "SAI_BSLOTR (rw) register accessor: an alias for `Reg<SAI_BSLOTR_SPEC>`"]
pub type SAI_BSLOTR = crate::Reg<sai_bslotr::SAI_BSLOTR_SPEC>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bslotr;
#[doc = "SAI_BIM (rw) register accessor: an alias for `Reg<SAI_BIM_SPEC>`"]
pub type SAI_BIM = crate::Reg<sai_bim::SAI_BIM_SPEC>;
#[doc = "Interrupt mask register"]
pub mod sai_bim;
#[doc = "SAI_BSR (r) register accessor: an alias for `Reg<SAI_BSR_SPEC>`"]
pub type SAI_BSR = crate::Reg<sai_bsr::SAI_BSR_SPEC>;
#[doc = "Status register"]
pub mod sai_bsr;
#[doc = "SAI_BCLRFR (w) register accessor: an alias for `Reg<SAI_BCLRFR_SPEC>`"]
pub type SAI_BCLRFR = crate::Reg<sai_bclrfr::SAI_BCLRFR_SPEC>;
#[doc = "Clear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI_BDR (rw) register accessor: an alias for `Reg<SAI_BDR_SPEC>`"]
pub type SAI_BDR = crate::Reg<sai_bdr::SAI_BDR_SPEC>;
#[doc = "Data register"]
pub mod sai_bdr;
#[doc = "SAI_PDMCR (rw) register accessor: an alias for `Reg<SAI_PDMCR_SPEC>`"]
pub type SAI_PDMCR = crate::Reg<sai_pdmcr::SAI_PDMCR_SPEC>;
#[doc = "PDM control register"]
pub mod sai_pdmcr;
#[doc = "SAI_PDMDLY (rw) register accessor: an alias for `Reg<SAI_PDMDLY_SPEC>`"]
pub type SAI_PDMDLY = crate::Reg<sai_pdmdly::SAI_PDMDLY_SPEC>;
#[doc = "PDM delay register"]
pub mod sai_pdmdly;
#[doc = "SAI_HWCFGR (r) register accessor: an alias for `Reg<SAI_HWCFGR_SPEC>`"]
pub type SAI_HWCFGR = crate::Reg<sai_hwcfgr::SAI_HWCFGR_SPEC>;
#[doc = "SAI hardware configuration register"]
pub mod sai_hwcfgr;
#[doc = "SAI_VERR (r) register accessor: an alias for `Reg<SAI_VERR_SPEC>`"]
pub type SAI_VERR = crate::Reg<sai_verr::SAI_VERR_SPEC>;
#[doc = "SAI version register"]
pub mod sai_verr;
#[doc = "SAI_IPIDR (r) register accessor: an alias for `Reg<SAI_IPIDR_SPEC>`"]
pub type SAI_IPIDR = crate::Reg<sai_ipidr::SAI_IPIDR_SPEC>;
#[doc = "SAI identification register"]
pub mod sai_ipidr;
#[doc = "SAI_SIDR (r) register accessor: an alias for `Reg<SAI_SIDR_SPEC>`"]
pub type SAI_SIDR = crate::Reg<sai_sidr::SAI_SIDR_SPEC>;
#[doc = "SAI size identification register"]
pub mod sai_sidr;
