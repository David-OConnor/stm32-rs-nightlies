#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved1: [u8; 0x04],
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
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash PCROP zone A Start address register"]
    pub pcrop1asr: PCROP1ASR,
    #[doc = "0x28 - Flash PCROP zone A End address register"]
    pub pcrop1aer: PCROP1AER,
    #[doc = "0x2c - Flash WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x30 - Flash WRP area B address register"]
    pub wrp1br: WRP1BR,
    #[doc = "0x34 - Flash PCROP zone B Start address register"]
    pub pcrop1bsr: PCROP1BSR,
    #[doc = "0x38 - Flash PCROP zone B End address register"]
    pub pcrop1ber: PCROP1BER,
    _reserved13: [u8; 0x44],
    #[doc = "0x80 - Flash Security register"]
    pub secr: SECR,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
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
#[doc = "PCROP1ASR (rw) register accessor: an alias for `Reg<PCROP1ASR_SPEC>`"]
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASR_SPEC>;
#[doc = "Flash PCROP zone A Start address register"]
pub mod pcrop1asr;
#[doc = "PCROP1AER (rw) register accessor: an alias for `Reg<PCROP1AER_SPEC>`"]
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AER_SPEC>;
#[doc = "Flash PCROP zone A End address register"]
pub mod pcrop1aer;
#[doc = "WRP1AR (rw) register accessor: an alias for `Reg<WRP1AR_SPEC>`"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
#[doc = "Flash WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: an alias for `Reg<WRP1BR_SPEC>`"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
#[doc = "Flash WRP area B address register"]
pub mod wrp1br;
#[doc = "PCROP1BSR (rw) register accessor: an alias for `Reg<PCROP1BSR_SPEC>`"]
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>;
#[doc = "Flash PCROP zone B Start address register"]
pub mod pcrop1bsr;
#[doc = "PCROP1BER (rw) register accessor: an alias for `Reg<PCROP1BER_SPEC>`"]
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BER_SPEC>;
#[doc = "Flash PCROP zone B End address register"]
pub mod pcrop1ber;
#[doc = "SECR (rw) register accessor: an alias for `Reg<SECR_SPEC>`"]
pub type SECR = crate::Reg<secr::SECR_SPEC>;
#[doc = "Flash Security register"]
pub mod secr;
