#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x08 - Flash option key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x0c - Flash status register"]
    pub sr: SR,
    #[doc = "0x10 - Flash control register"]
    pub cr: CR,
    #[doc = "0x14 - Flash address register"]
    pub ar: AR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte register"]
    pub obr: OBR,
    #[doc = "0x20 - Write protection register"]
    pub wrpr: WRPR,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Flash status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "AR (w) register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Flash address register"]
pub mod ar;
#[doc = "OBR (r) register accessor: an alias for `Reg<OBR_SPEC>`"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR (r) register accessor: an alias for `Reg<WRPR_SPEC>`"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = "Write protection register"]
pub mod wrpr;
