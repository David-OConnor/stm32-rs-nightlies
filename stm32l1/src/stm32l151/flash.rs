#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Program/erase control register"]
    pub pecr: PECR,
    #[doc = "0x08 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x0c - Program/erase key register"]
    pub pekeyr: PEKEYR,
    #[doc = "0x10 - Program memory key register"]
    pub prgkeyr: PRGKEYR,
    #[doc = "0x14 - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x18 - Status register"]
    pub sr: SR,
    #[doc = "0x1c - Option byte register"]
    pub obr: OBR,
    #[doc = "0x20 - Write protection register"]
    pub wrpr1: WRPR1,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - Write protection register"]
    pub wrpr2: WRPR2,
    #[doc = "0x84 - Write protection register"]
    pub wrpr3: WRPR3,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PECR (rw) register accessor: an alias for `Reg<PECR_SPEC>`"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "Program/erase control register"]
pub mod pecr;
#[doc = "PDKEYR (w) register accessor: an alias for `Reg<PDKEYR_SPEC>`"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "PEKEYR (w) register accessor: an alias for `Reg<PEKEYR_SPEC>`"]
pub type PEKEYR = crate::Reg<pekeyr::PEKEYR_SPEC>;
#[doc = "Program/erase key register"]
pub mod pekeyr;
#[doc = "PRGKEYR (w) register accessor: an alias for `Reg<PRGKEYR_SPEC>`"]
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYR_SPEC>;
#[doc = "Program memory key register"]
pub mod prgkeyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "OBR (r) register accessor: an alias for `Reg<OBR_SPEC>`"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR1 (rw) register accessor: an alias for `Reg<WRPR1_SPEC>`"]
pub type WRPR1 = crate::Reg<wrpr1::WRPR1_SPEC>;
#[doc = "Write protection register"]
pub mod wrpr1;
#[doc = "WRPR2 (rw) register accessor: an alias for `Reg<WRPR2_SPEC>`"]
pub type WRPR2 = crate::Reg<wrpr2::WRPR2_SPEC>;
#[doc = "Write protection register"]
pub mod wrpr2;
#[doc = "WRPR3 (rw) register accessor: an alias for `Reg<WRPR3_SPEC>`"]
pub type WRPR3 = crate::Reg<wrpr3::WRPR3_SPEC>;
#[doc = "Write protection register"]
pub mod wrpr3;
