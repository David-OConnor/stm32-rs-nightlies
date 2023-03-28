#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - TX mailbox identifier register"]
    pub tir: TIR,
    #[doc = "0x04 - mailbox data length control and time stamp register"]
    pub tdtr: TDTR,
    #[doc = "0x08 - mailbox data low register"]
    pub tdlr: TDLR,
    #[doc = "0x0c - mailbox data high register"]
    pub tdhr: TDHR,
}
#[doc = "TIR (rw) register accessor: an alias for `Reg<TIR_SPEC>`"]
pub type TIR = crate::Reg<tir::TIR_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod tir;
#[doc = "TDTR (rw) register accessor: an alias for `Reg<TDTR_SPEC>`"]
pub type TDTR = crate::Reg<tdtr::TDTR_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdtr;
#[doc = "TDLR (rw) register accessor: an alias for `Reg<TDLR_SPEC>`"]
pub type TDLR = crate::Reg<tdlr::TDLR_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdlr;
#[doc = "TDHR (rw) register accessor: an alias for `Reg<TDHR_SPEC>`"]
pub type TDHR = crate::Reg<tdhr::TDHR_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdhr;
