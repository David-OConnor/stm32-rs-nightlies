#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x08 - Flash option key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x0c - Status register"]
    pub sr: SR,
    #[doc = "0x10 - Control register"]
    pub cr: CR,
    #[doc = "0x14 - Flash option control register"]
    pub optcr: OPTCR,
    #[doc = "0x18 - Flash option control register 1"]
    pub optcr1: OPTCR1,
    #[doc = "0x1c - Flash option control register"]
    pub optcr2: OPTCR2,
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
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "Flash option control register"]
pub mod optcr;
#[doc = "OPTCR1 (rw) register accessor: an alias for `Reg<OPTCR1_SPEC>`"]
pub type OPTCR1 = crate::Reg<optcr1::OPTCR1_SPEC>;
#[doc = "Flash option control register 1"]
pub mod optcr1;
#[doc = "OPTCR2 (rw) register accessor: an alias for `Reg<OPTCR2_SPEC>`"]
pub type OPTCR2 = crate::Reg<optcr2::OPTCR2_SPEC>;
#[doc = "Flash option control register"]
pub mod optcr2;
