#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: EP0R,
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: EP1R,
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: EP2R,
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: EP3R,
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: EP4R,
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: EP5R,
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: EP6R,
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: EP7R,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
    _reserved13: [u8; 0x04],
    #[doc = "0x58 - Battery charging detector"]
    pub bcdr: BCDR,
}
#[doc = "EP0R (rw) register accessor: an alias for `Reg<EP0R_SPEC>`"]
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "EP1R (rw) register accessor: an alias for `Reg<EP1R_SPEC>`"]
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "EP2R (rw) register accessor: an alias for `Reg<EP2R_SPEC>`"]
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "EP3R (rw) register accessor: an alias for `Reg<EP3R_SPEC>`"]
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "EP4R (rw) register accessor: an alias for `Reg<EP4R_SPEC>`"]
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "EP5R (rw) register accessor: an alias for `Reg<EP5R_SPEC>`"]
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "EP6R (rw) register accessor: an alias for `Reg<EP6R_SPEC>`"]
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "EP7R (rw) register accessor: an alias for `Reg<EP7R_SPEC>`"]
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "control register"]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: an alias for `Reg<BTABLE_SPEC>`"]
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "BCDR (rw) register accessor: an alias for `Reg<BCDR_SPEC>`"]
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
#[doc = "Battery charging detector"]
pub mod bcdr;
