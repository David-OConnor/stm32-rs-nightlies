#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    #[doc = "0x18 - Flash ECC register"]
    pub eccr: ECCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash Bank 1 PCROP Start address register"]
    pub pcrop1sr: PCROP1SR,
    #[doc = "0x28 - Flash Bank 1 PCROP End address register"]
    pub pcrop1er: PCROP1ER,
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x30 - Flash Bank 2 WRP area A address register"]
    pub wrp2ar: WRP2AR,
    _reserved12: [u8; 0x10],
    #[doc = "0x44 - Flash Bank 2 PCROP Start address register"]
    pub pcrop2sr: PCROP2SR,
    #[doc = "0x48 - Flash Bank 2 PCROP End address register"]
    pub pcrop2er: PCROP2ER,
    #[doc = "0x4c - Flash Bank 1 WRP area B address register"]
    pub wrp1br: WRP1BR,
    #[doc = "0x50 - Flash Bank 2 WRP area B address register"]
    pub wrp2br: WRP2BR,
    _reserved16: [u8; 0xdc],
    #[doc = "0x130 - flash configuration register"]
    pub cfgr: CFGR,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PDKEYR (w) register accessor: an alias for `Reg<PDKEYR_SPEC>`"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "ECCR (rw) register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR (rw) register accessor: an alias for `Reg<OPTR_SPEC>`"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "PCROP1SR (rw) register accessor: an alias for `Reg<PCROP1SR_SPEC>`"]
pub type PCROP1SR = crate::Reg<pcrop1sr::PCROP1SR_SPEC>;
#[doc = "Flash Bank 1 PCROP Start address register"]
pub mod pcrop1sr;
#[doc = "PCROP1ER (rw) register accessor: an alias for `Reg<PCROP1ER_SPEC>`"]
pub type PCROP1ER = crate::Reg<pcrop1er::PCROP1ER_SPEC>;
#[doc = "Flash Bank 1 PCROP End address register"]
pub mod pcrop1er;
#[doc = "WRP1AR (rw) register accessor: an alias for `Reg<WRP1AR_SPEC>`"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: an alias for `Reg<WRP1BR_SPEC>`"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP2SR (rw) register accessor: an alias for `Reg<PCROP2SR_SPEC>`"]
pub type PCROP2SR = crate::Reg<pcrop2sr::PCROP2SR_SPEC>;
#[doc = "Flash Bank 2 PCROP Start address register"]
pub mod pcrop2sr;
#[doc = "PCROP2ER (rw) register accessor: an alias for `Reg<PCROP2ER_SPEC>`"]
pub type PCROP2ER = crate::Reg<pcrop2er::PCROP2ER_SPEC>;
#[doc = "Flash Bank 2 PCROP End address register"]
pub mod pcrop2er;
#[doc = "WRP2AR (rw) register accessor: an alias for `Reg<WRP2AR_SPEC>`"]
pub type WRP2AR = crate::Reg<wrp2ar::WRP2AR_SPEC>;
#[doc = "Flash Bank 2 WRP area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: an alias for `Reg<WRP2BR_SPEC>`"]
pub type WRP2BR = crate::Reg<wrp2br::WRP2BR_SPEC>;
#[doc = "Flash Bank 2 WRP area B address register"]
pub mod wrp2br;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "flash configuration register"]
pub mod cfgr;
