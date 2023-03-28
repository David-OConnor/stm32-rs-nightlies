#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Flash access control register 2"]
    pub acr2: ACR2,
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
    #[doc = "0x3c - Flash IPCC data buffer address register"]
    pub ipccbr: IPCCBR,
    _reserved15: [u8; 0x1c],
    #[doc = "0x5c - Flash CPU2 access control register"]
    pub c2acr: C2ACR,
    #[doc = "0x60 - Flash CPU2 status register"]
    pub c2sr: C2SR,
    #[doc = "0x64 - Flash CPU2 control register"]
    pub c2cr: C2CR,
    _reserved18: [u8; 0x18],
    #[doc = "0x80 - Flash secure Flash start address register"]
    pub sfr: SFR,
    #[doc = "0x84 - Flash secure SRAM start address and CPU2 reset vector register"]
    pub srrvr: SRRVR,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "ACR2 (rw) register accessor: an alias for `Reg<ACR2_SPEC>`"]
pub type ACR2 = crate::Reg<acr2::ACR2_SPEC>;
#[doc = "Flash access control register 2"]
pub mod acr2;
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
#[doc = "IPCCBR (rw) register accessor: an alias for `Reg<IPCCBR_SPEC>`"]
pub type IPCCBR = crate::Reg<ipccbr::IPCCBR_SPEC>;
#[doc = "Flash IPCC data buffer address register"]
pub mod ipccbr;
#[doc = "C2ACR (rw) register accessor: an alias for `Reg<C2ACR_SPEC>`"]
pub type C2ACR = crate::Reg<c2acr::C2ACR_SPEC>;
#[doc = "Flash CPU2 access control register"]
pub mod c2acr;
#[doc = "C2SR (rw) register accessor: an alias for `Reg<C2SR_SPEC>`"]
pub type C2SR = crate::Reg<c2sr::C2SR_SPEC>;
#[doc = "Flash CPU2 status register"]
pub mod c2sr;
#[doc = "C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "Flash CPU2 control register"]
pub mod c2cr;
#[doc = "SFR (rw) register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Flash secure Flash start address register"]
pub mod sfr;
#[doc = "SRRVR (rw) register accessor: an alias for `Reg<SRRVR_SPEC>`"]
pub type SRRVR = crate::Reg<srrvr::SRRVR_SPEC>;
#[doc = "Flash secure SRAM start address and CPU2 reset vector register"]
pub mod srrvr;
