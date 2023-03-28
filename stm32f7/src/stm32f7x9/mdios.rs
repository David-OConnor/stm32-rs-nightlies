#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub cr: CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub wrfr: WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub cwrfr: CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub rdfr: RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub crdfr: CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub sr: SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub clrfr: CLRFR,
    #[doc = "0x1c..0x9c - MDIOS input data register %s"]
    pub dinr: [DINR; 32],
    #[doc = "0x9c..0x11c - MDIOS output data register %s"]
    pub doutr: [DOUTR; 32],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "MDIOS configuration register"]
pub mod cr;
#[doc = "WRFR (r) register accessor: an alias for `Reg<WRFR_SPEC>`"]
pub type WRFR = crate::Reg<wrfr::WRFR_SPEC>;
#[doc = "MDIOS write flag register"]
pub mod wrfr;
#[doc = "CWRFR (rw) register accessor: an alias for `Reg<CWRFR_SPEC>`"]
pub type CWRFR = crate::Reg<cwrfr::CWRFR_SPEC>;
#[doc = "MDIOS clear write flag register"]
pub mod cwrfr;
#[doc = "RDFR (r) register accessor: an alias for `Reg<RDFR_SPEC>`"]
pub type RDFR = crate::Reg<rdfr::RDFR_SPEC>;
#[doc = "MDIOS read flag register"]
pub mod rdfr;
#[doc = "CRDFR (rw) register accessor: an alias for `Reg<CRDFR_SPEC>`"]
pub type CRDFR = crate::Reg<crdfr::CRDFR_SPEC>;
#[doc = "MDIOS clear read flag register"]
pub mod crdfr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "MDIOS status register"]
pub mod sr;
#[doc = "CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`"]
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
#[doc = "MDIOS clear flag register"]
pub mod clrfr;
#[doc = "DINR (r) register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "MDIOS input data register %s"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "MDIOS output data register %s"]
pub mod doutr;
