#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    pub cr1: CR1,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - PWR control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - PWR control register 4"]
    pub cr4: CR4,
    #[doc = "0x10 - PWR status register 1"]
    pub sr1: SR1,
    #[doc = "0x14 - PWR status register 2"]
    pub sr2: SR2,
    #[doc = "0x18 - PWR status clear register"]
    pub scr: SCR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - PWR Port A pull-up control register"]
    pub pucra: PUCRA,
    #[doc = "0x24 - PWR Port A pull-down control register"]
    pub pdcra: PDCRA,
    #[doc = "0x28 - PWR Port B pull-up control register"]
    pub pucrb: PUCRB,
    #[doc = "0x2c - PWR Port B pull-down control register"]
    pub pdcrb: PDCRB,
    #[doc = "0x30 - PWR Port C pull-up control register"]
    pub pucrc: PUCRC,
    #[doc = "0x34 - PWR Port C pull-down control register"]
    pub pdcrc: PDCRC,
    #[doc = "0x38 - PWR Port D pull-up control register"]
    pub pucrd: PUCRD,
    #[doc = "0x3c - PWR Port D pull-down control register"]
    pub pdcrd: PDCRD,
    _reserved14: [u8; 0x08],
    #[doc = "0x48 - PWR Port F pull-up control register"]
    pub pucrf: PUCRF,
    #[doc = "0x4c - PWR Port F pull-down control register"]
    pub pdcrf: PDCRF,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "PWR control register 1"]
pub mod cr1;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "PWR control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: an alias for `Reg<CR4_SPEC>`"]
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
#[doc = "PWR control register 4"]
pub mod cr4;
#[doc = "SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "PWR status register 1"]
pub mod sr1;
#[doc = "SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "PWR status register 2"]
pub mod sr2;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "PWR status clear register"]
pub mod scr;
#[doc = "PUCRA (rw) register accessor: an alias for `Reg<PUCRA_SPEC>`"]
pub type PUCRA = crate::Reg<pucra::PUCRA_SPEC>;
#[doc = "PWR Port A pull-up control register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: an alias for `Reg<PDCRA_SPEC>`"]
pub type PDCRA = crate::Reg<pdcra::PDCRA_SPEC>;
#[doc = "PWR Port A pull-down control register"]
pub mod pdcra;
#[doc = "PUCRB (rw) register accessor: an alias for `Reg<PUCRB_SPEC>`"]
pub type PUCRB = crate::Reg<pucrb::PUCRB_SPEC>;
#[doc = "PWR Port B pull-up control register"]
pub mod pucrb;
#[doc = "PDCRB (rw) register accessor: an alias for `Reg<PDCRB_SPEC>`"]
pub type PDCRB = crate::Reg<pdcrb::PDCRB_SPEC>;
#[doc = "PWR Port B pull-down control register"]
pub mod pdcrb;
#[doc = "PUCRC (rw) register accessor: an alias for `Reg<PUCRC_SPEC>`"]
pub type PUCRC = crate::Reg<pucrc::PUCRC_SPEC>;
#[doc = "PWR Port C pull-up control register"]
pub mod pucrc;
#[doc = "PDCRC (rw) register accessor: an alias for `Reg<PDCRC_SPEC>`"]
pub type PDCRC = crate::Reg<pdcrc::PDCRC_SPEC>;
#[doc = "PWR Port C pull-down control register"]
pub mod pdcrc;
#[doc = "PUCRD (rw) register accessor: an alias for `Reg<PUCRD_SPEC>`"]
pub type PUCRD = crate::Reg<pucrd::PUCRD_SPEC>;
#[doc = "PWR Port D pull-up control register"]
pub mod pucrd;
#[doc = "PDCRD (rw) register accessor: an alias for `Reg<PDCRD_SPEC>`"]
pub type PDCRD = crate::Reg<pdcrd::PDCRD_SPEC>;
#[doc = "PWR Port D pull-down control register"]
pub mod pdcrd;
#[doc = "PUCRF (rw) register accessor: an alias for `Reg<PUCRF_SPEC>`"]
pub type PUCRF = crate::Reg<pucrf::PUCRF_SPEC>;
#[doc = "PWR Port F pull-up control register"]
pub mod pucrf;
#[doc = "PDCRF (rw) register accessor: an alias for `Reg<PDCRF_SPEC>`"]
pub type PDCRF = crate::Reg<pdcrf::PDCRF_SPEC>;
#[doc = "PWR Port F pull-down control register"]
pub mod pdcrf;
