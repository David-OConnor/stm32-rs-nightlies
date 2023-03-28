#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    pub usb_chep0r: USB_CHEP0R,
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    pub usb_chep1r: USB_CHEP1R,
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    pub usb_chep2r: USB_CHEP2R,
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    pub usb_chep3r: USB_CHEP3R,
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    pub usb_chep4r: USB_CHEP4R,
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    pub usb_chep5r: USB_CHEP5R,
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    pub usb_chep6r: USB_CHEP6R,
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    pub usb_chep7r: USB_CHEP7R,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - "]
    pub usb_cntr: USB_CNTR,
    #[doc = "0x44 - USB interrupt status register"]
    pub usb_istr: USB_ISTR,
    #[doc = "0x48 - USB frame number register"]
    pub usb_fnr: USB_FNR,
    #[doc = "0x4c - USB_DADDR"]
    pub usb_daddr: USB_DADDR,
    _reserved12: [u8; 0x04],
    #[doc = "0x54 - USB_LPMCSR"]
    pub usb_lpmcsr: USB_LPMCSR,
    #[doc = "0x58 - USB_BCDR"]
    pub usb_bcdr: USB_BCDR,
}
#[doc = "USB_CHEP0R (rw) register accessor: an alias for `Reg<USB_CHEP0R_SPEC>`"]
pub type USB_CHEP0R = crate::Reg<usb_chep0r::USB_CHEP0R_SPEC>;
#[doc = "USB endpoint/channel 0 register"]
pub mod usb_chep0r;
#[doc = "USB_CHEP1R (rw) register accessor: an alias for `Reg<USB_CHEP1R_SPEC>`"]
pub type USB_CHEP1R = crate::Reg<usb_chep1r::USB_CHEP1R_SPEC>;
#[doc = "USB endpoint/channel 1 register"]
pub mod usb_chep1r;
#[doc = "USB_CHEP2R (rw) register accessor: an alias for `Reg<USB_CHEP2R_SPEC>`"]
pub type USB_CHEP2R = crate::Reg<usb_chep2r::USB_CHEP2R_SPEC>;
#[doc = "USB endpoint/channel 2 register"]
pub mod usb_chep2r;
#[doc = "USB_CHEP3R (rw) register accessor: an alias for `Reg<USB_CHEP3R_SPEC>`"]
pub type USB_CHEP3R = crate::Reg<usb_chep3r::USB_CHEP3R_SPEC>;
#[doc = "USB endpoint/channel 3 register"]
pub mod usb_chep3r;
#[doc = "USB_CHEP4R (rw) register accessor: an alias for `Reg<USB_CHEP4R_SPEC>`"]
pub type USB_CHEP4R = crate::Reg<usb_chep4r::USB_CHEP4R_SPEC>;
#[doc = "USB endpoint/channel 4 register"]
pub mod usb_chep4r;
#[doc = "USB_CHEP5R (rw) register accessor: an alias for `Reg<USB_CHEP5R_SPEC>`"]
pub type USB_CHEP5R = crate::Reg<usb_chep5r::USB_CHEP5R_SPEC>;
#[doc = "USB endpoint/channel 5 register"]
pub mod usb_chep5r;
#[doc = "USB_CHEP6R (rw) register accessor: an alias for `Reg<USB_CHEP6R_SPEC>`"]
pub type USB_CHEP6R = crate::Reg<usb_chep6r::USB_CHEP6R_SPEC>;
#[doc = "USB endpoint/channel 6 register"]
pub mod usb_chep6r;
#[doc = "USB_CHEP7R (rw) register accessor: an alias for `Reg<USB_CHEP7R_SPEC>`"]
pub type USB_CHEP7R = crate::Reg<usb_chep7r::USB_CHEP7R_SPEC>;
#[doc = "USB endpoint/channel 7 register"]
pub mod usb_chep7r;
#[doc = "USB_CNTR (rw) register accessor: an alias for `Reg<USB_CNTR_SPEC>`"]
pub type USB_CNTR = crate::Reg<usb_cntr::USB_CNTR_SPEC>;
#[doc = ""]
pub mod usb_cntr;
#[doc = "USB_ISTR (rw) register accessor: an alias for `Reg<USB_ISTR_SPEC>`"]
pub type USB_ISTR = crate::Reg<usb_istr::USB_ISTR_SPEC>;
#[doc = "USB interrupt status register"]
pub mod usb_istr;
#[doc = "USB_FNR (r) register accessor: an alias for `Reg<USB_FNR_SPEC>`"]
pub type USB_FNR = crate::Reg<usb_fnr::USB_FNR_SPEC>;
#[doc = "USB frame number register"]
pub mod usb_fnr;
#[doc = "USB_DADDR (rw) register accessor: an alias for `Reg<USB_DADDR_SPEC>`"]
pub type USB_DADDR = crate::Reg<usb_daddr::USB_DADDR_SPEC>;
#[doc = "USB_DADDR"]
pub mod usb_daddr;
#[doc = "USB_LPMCSR (rw) register accessor: an alias for `Reg<USB_LPMCSR_SPEC>`"]
pub type USB_LPMCSR = crate::Reg<usb_lpmcsr::USB_LPMCSR_SPEC>;
#[doc = "USB_LPMCSR"]
pub mod usb_lpmcsr;
#[doc = "USB_BCDR (rw) register accessor: an alias for `Reg<USB_BCDR_SPEC>`"]
pub type USB_BCDR = crate::Reg<usb_bcdr::USB_BCDR_SPEC>;
#[doc = "USB_BCDR"]
pub mod usb_bcdr;
