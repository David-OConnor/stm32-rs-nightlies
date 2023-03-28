#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    pub chep0r: CHEP0R,
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    pub chep1r: CHEP1R,
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    pub chep2r: CHEP2R,
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    pub chep3r: CHEP3R,
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    pub chep4r: CHEP4R,
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    pub chep5r: CHEP5R,
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    pub chep6r: CHEP6R,
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    pub chep7r: CHEP7R,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - USB control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - USB interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - USB frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - USB device address"]
    pub daddr: DADDR,
    _reserved12: [u8; 0x04],
    #[doc = "0x54 - LPM control and status register"]
    pub lpmcsr: LPMCSR,
    #[doc = "0x58 - Battery charging detector"]
    pub bcdr: BCDR,
}
#[doc = "CHEP0R (rw) register accessor: an alias for `Reg<CHEP0R_SPEC>`"]
pub type CHEP0R = crate::Reg<chep0r::CHEP0R_SPEC>;
#[doc = "USB endpoint/channel 0 register"]
pub mod chep0r;
#[doc = "CHEP1R (rw) register accessor: an alias for `Reg<CHEP1R_SPEC>`"]
pub type CHEP1R = crate::Reg<chep1r::CHEP1R_SPEC>;
#[doc = "USB endpoint/channel 1 register"]
pub mod chep1r;
#[doc = "CHEP2R (rw) register accessor: an alias for `Reg<CHEP2R_SPEC>`"]
pub type CHEP2R = crate::Reg<chep2r::CHEP2R_SPEC>;
#[doc = "USB endpoint/channel 2 register"]
pub mod chep2r;
#[doc = "CHEP3R (rw) register accessor: an alias for `Reg<CHEP3R_SPEC>`"]
pub type CHEP3R = crate::Reg<chep3r::CHEP3R_SPEC>;
#[doc = "USB endpoint/channel 3 register"]
pub mod chep3r;
#[doc = "CHEP4R (rw) register accessor: an alias for `Reg<CHEP4R_SPEC>`"]
pub type CHEP4R = crate::Reg<chep4r::CHEP4R_SPEC>;
#[doc = "USB endpoint/channel 4 register"]
pub mod chep4r;
#[doc = "CHEP5R (rw) register accessor: an alias for `Reg<CHEP5R_SPEC>`"]
pub type CHEP5R = crate::Reg<chep5r::CHEP5R_SPEC>;
#[doc = "USB endpoint/channel 5 register"]
pub mod chep5r;
#[doc = "CHEP6R (rw) register accessor: an alias for `Reg<CHEP6R_SPEC>`"]
pub type CHEP6R = crate::Reg<chep6r::CHEP6R_SPEC>;
#[doc = "USB endpoint/channel 6 register"]
pub mod chep6r;
#[doc = "CHEP7R (rw) register accessor: an alias for `Reg<CHEP7R_SPEC>`"]
pub type CHEP7R = crate::Reg<chep7r::CHEP7R_SPEC>;
#[doc = "USB endpoint/channel 7 register"]
pub mod chep7r;
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "USB control register"]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "USB device address"]
pub mod daddr;
#[doc = "LPMCSR (rw) register accessor: an alias for `Reg<LPMCSR_SPEC>`"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
#[doc = "LPM control and status register"]
pub mod lpmcsr;
#[doc = "BCDR (rw) register accessor: an alias for `Reg<BCDR_SPEC>`"]
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
#[doc = "Battery charging detector"]
pub mod bcdr;
