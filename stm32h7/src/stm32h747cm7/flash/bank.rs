#[doc = r"Register block"]
#[repr(C)]
pub struct BANK {
    #[doc = "0x00 - FLASH key register for bank 1"]
    pub keyr: KEYR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - FLASH control register for bank 1"]
    pub cr: CR,
    #[doc = "0x0c - FLASH status register for bank 1"]
    pub sr: SR,
    #[doc = "0x10 - FLASH clear control register for bank 1"]
    pub ccr: CCR,
    _reserved4: [u8; 0x10],
    #[doc = "0x24 - FLASH protection address for bank 1"]
    pub prar_cur: PRAR_CUR,
    #[doc = "0x28 - FLASH protection address for bank 1"]
    pub prar_prg: PRAR_PRG,
    #[doc = "0x2c - FLASH secure address for bank 1"]
    pub scar_cur: SCAR_CUR,
    #[doc = "0x30 - FLASH secure address for bank 1"]
    pub scar_prg: SCAR_PRG,
    #[doc = "0x34 - FLASH write sector protection for bank 1"]
    pub wpsn_curr: WPSN_CURR,
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    pub wpsn_prgr: WPSN_PRGR,
    _reserved10: [u8; 0x10],
    #[doc = "0x4c - FLASH CRC control register for bank 1"]
    pub crccr: CRCCR,
    #[doc = "0x50 - FLASH CRC start address register for bank 1"]
    pub crcsaddr: CRCSADDR,
    #[doc = "0x54 - FLASH CRC end address register for bank 1"]
    pub crceaddr: CRCEADDR,
    _reserved13: [u8; 0x04],
    #[doc = "0x5c - FLASH ECC fail address for bank 1"]
    pub far: FAR,
    _reserved_end: [u8; 0xa0],
}
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "FLASH key register for bank 1"]
pub mod keyr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "FLASH control register for bank 1"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FLASH status register for bank 1"]
pub mod sr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "FLASH clear control register for bank 1"]
pub mod ccr;
#[doc = "PRAR_CUR (rw) register accessor: an alias for `Reg<PRAR_CUR_SPEC>`"]
pub type PRAR_CUR = crate::Reg<prar_cur::PRAR_CUR_SPEC>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur;
#[doc = "PRAR_PRG (rw) register accessor: an alias for `Reg<PRAR_PRG_SPEC>`"]
pub type PRAR_PRG = crate::Reg<prar_prg::PRAR_PRG_SPEC>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_prg;
#[doc = "SCAR_CUR (rw) register accessor: an alias for `Reg<SCAR_CUR_SPEC>`"]
pub type SCAR_CUR = crate::Reg<scar_cur::SCAR_CUR_SPEC>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_cur;
#[doc = "SCAR_PRG (rw) register accessor: an alias for `Reg<SCAR_PRG_SPEC>`"]
pub type SCAR_PRG = crate::Reg<scar_prg::SCAR_PRG_SPEC>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_prg;
#[doc = "WPSN_CURR (rw) register accessor: an alias for `Reg<WPSN_CURR_SPEC>`"]
pub type WPSN_CURR = crate::Reg<wpsn_curr::WPSN_CURR_SPEC>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_curr;
#[doc = "WPSN_PRGR (rw) register accessor: an alias for `Reg<WPSN_PRGR_SPEC>`"]
pub type WPSN_PRGR = crate::Reg<wpsn_prgr::WPSN_PRGR_SPEC>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_prgr;
#[doc = "CRCCR (rw) register accessor: an alias for `Reg<CRCCR_SPEC>`"]
pub type CRCCR = crate::Reg<crccr::CRCCR_SPEC>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr;
#[doc = "CRCSADDR (rw) register accessor: an alias for `Reg<CRCSADDR_SPEC>`"]
pub type CRCSADDR = crate::Reg<crcsaddr::CRCSADDR_SPEC>;
#[doc = "FLASH CRC start address register for bank 1"]
pub mod crcsaddr;
#[doc = "CRCEADDR (rw) register accessor: an alias for `Reg<CRCEADDR_SPEC>`"]
pub type CRCEADDR = crate::Reg<crceaddr::CRCEADDR_SPEC>;
#[doc = "FLASH CRC end address register for bank 1"]
pub mod crceaddr;
#[doc = "FAR (rw) register accessor: an alias for `Reg<FAR_SPEC>`"]
pub type FAR = crate::Reg<far::FAR_SPEC>;
#[doc = "FLASH ECC fail address for bank 1"]
pub mod far;
