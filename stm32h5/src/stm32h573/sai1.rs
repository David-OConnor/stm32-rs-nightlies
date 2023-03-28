#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAI global configuration register"]
    pub sai_gcr: SAI_GCR,
    #[doc = "0x04 - SAI configuration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - SAI configuration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - SAI frame configuration register"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - SAI slot register"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - SAI interrupt mask register"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - SAI status register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - SAI clear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - SAI data register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - SAI configuration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - SAI configuration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - SAI frame configuration register"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - SAI slot register"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - SAI interrupt mask register"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - SAI status register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - SAI clear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - SAI data register"]
    pub sai_bdr: SAI_BDR,
    #[doc = "0x44 - SAI PDM control register"]
    pub sai_pdmcr: SAI_PDMCR,
    #[doc = "0x48 - SAI PDM delay register"]
    pub sai_pdmdly: SAI_PDMDLY,
}
#[doc = "SAI_GCR (rw) register accessor: an alias for `Reg<SAI_GCR_SPEC>`"]
pub type SAI_GCR = crate::Reg<sai_gcr::SAI_GCR_SPEC>;
#[doc = "SAI global configuration register"]
pub mod sai_gcr;
#[doc = "SAI_ACR1 (rw) register accessor: an alias for `Reg<SAI_ACR1_SPEC>`"]
pub type SAI_ACR1 = crate::Reg<sai_acr1::SAI_ACR1_SPEC>;
#[doc = "SAI configuration register 1"]
pub mod sai_acr1;
#[doc = "SAI_ACR2 (rw) register accessor: an alias for `Reg<SAI_ACR2_SPEC>`"]
pub type SAI_ACR2 = crate::Reg<sai_acr2::SAI_ACR2_SPEC>;
#[doc = "SAI configuration register 2"]
pub mod sai_acr2;
#[doc = "SAI_AFRCR (rw) register accessor: an alias for `Reg<SAI_AFRCR_SPEC>`"]
pub type SAI_AFRCR = crate::Reg<sai_afrcr::SAI_AFRCR_SPEC>;
#[doc = "SAI frame configuration register"]
pub mod sai_afrcr;
#[doc = "SAI_ASLOTR (rw) register accessor: an alias for `Reg<SAI_ASLOTR_SPEC>`"]
pub type SAI_ASLOTR = crate::Reg<sai_aslotr::SAI_ASLOTR_SPEC>;
#[doc = "SAI slot register"]
pub mod sai_aslotr;
#[doc = "SAI_AIM (rw) register accessor: an alias for `Reg<SAI_AIM_SPEC>`"]
pub type SAI_AIM = crate::Reg<sai_aim::SAI_AIM_SPEC>;
#[doc = "SAI interrupt mask register"]
pub mod sai_aim;
#[doc = "SAI_ASR (r) register accessor: an alias for `Reg<SAI_ASR_SPEC>`"]
pub type SAI_ASR = crate::Reg<sai_asr::SAI_ASR_SPEC>;
#[doc = "SAI status register"]
pub mod sai_asr;
#[doc = "SAI_ACLRFR (w) register accessor: an alias for `Reg<SAI_ACLRFR_SPEC>`"]
pub type SAI_ACLRFR = crate::Reg<sai_aclrfr::SAI_ACLRFR_SPEC>;
#[doc = "SAI clear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI_ADR (rw) register accessor: an alias for `Reg<SAI_ADR_SPEC>`"]
pub type SAI_ADR = crate::Reg<sai_adr::SAI_ADR_SPEC>;
#[doc = "SAI data register"]
pub mod sai_adr;
#[doc = "SAI_BCR1 (rw) register accessor: an alias for `Reg<SAI_BCR1_SPEC>`"]
pub type SAI_BCR1 = crate::Reg<sai_bcr1::SAI_BCR1_SPEC>;
#[doc = "SAI configuration register 1"]
pub mod sai_bcr1;
#[doc = "SAI_BCR2 (rw) register accessor: an alias for `Reg<SAI_BCR2_SPEC>`"]
pub type SAI_BCR2 = crate::Reg<sai_bcr2::SAI_BCR2_SPEC>;
#[doc = "SAI configuration register 2"]
pub mod sai_bcr2;
#[doc = "SAI_BFRCR (rw) register accessor: an alias for `Reg<SAI_BFRCR_SPEC>`"]
pub type SAI_BFRCR = crate::Reg<sai_bfrcr::SAI_BFRCR_SPEC>;
#[doc = "SAI frame configuration register"]
pub mod sai_bfrcr;
#[doc = "SAI_BSLOTR (rw) register accessor: an alias for `Reg<SAI_BSLOTR_SPEC>`"]
pub type SAI_BSLOTR = crate::Reg<sai_bslotr::SAI_BSLOTR_SPEC>;
#[doc = "SAI slot register"]
pub mod sai_bslotr;
#[doc = "SAI_BIM (rw) register accessor: an alias for `Reg<SAI_BIM_SPEC>`"]
pub type SAI_BIM = crate::Reg<sai_bim::SAI_BIM_SPEC>;
#[doc = "SAI interrupt mask register"]
pub mod sai_bim;
#[doc = "SAI_BSR (r) register accessor: an alias for `Reg<SAI_BSR_SPEC>`"]
pub type SAI_BSR = crate::Reg<sai_bsr::SAI_BSR_SPEC>;
#[doc = "SAI status register"]
pub mod sai_bsr;
#[doc = "SAI_BCLRFR (w) register accessor: an alias for `Reg<SAI_BCLRFR_SPEC>`"]
pub type SAI_BCLRFR = crate::Reg<sai_bclrfr::SAI_BCLRFR_SPEC>;
#[doc = "SAI clear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI_BDR (rw) register accessor: an alias for `Reg<SAI_BDR_SPEC>`"]
pub type SAI_BDR = crate::Reg<sai_bdr::SAI_BDR_SPEC>;
#[doc = "SAI data register"]
pub mod sai_bdr;
#[doc = "SAI_PDMCR (rw) register accessor: an alias for `Reg<SAI_PDMCR_SPEC>`"]
pub type SAI_PDMCR = crate::Reg<sai_pdmcr::SAI_PDMCR_SPEC>;
#[doc = "SAI PDM control register"]
pub mod sai_pdmcr;
#[doc = "SAI_PDMDLY (rw) register accessor: an alias for `Reg<SAI_PDMDLY_SPEC>`"]
pub type SAI_PDMDLY = crate::Reg<sai_pdmdly::SAI_PDMDLY_SPEC>;
#[doc = "SAI PDM delay register"]
pub mod sai_pdmdly;
