#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - CAN_TI0R"]
    pub tir: TIR,
    #[doc = "0x04 - CAN_TDT0R"]
    pub tdtr: TDTR,
    #[doc = "0x08 - CAN_TDL0R"]
    pub tdlr: TDLR,
    #[doc = "0x0c - CAN_TDH0R"]
    pub tdhr: TDHR,
}
#[doc = "TIR (rw) register accessor: an alias for `Reg<TIR_SPEC>`"]
pub type TIR = crate::Reg<tir::TIR_SPEC>;
#[doc = "CAN_TI0R"]
pub mod tir;
#[doc = "TDTR (rw) register accessor: an alias for `Reg<TDTR_SPEC>`"]
pub type TDTR = crate::Reg<tdtr::TDTR_SPEC>;
#[doc = "CAN_TDT0R"]
pub mod tdtr;
#[doc = "TDLR (rw) register accessor: an alias for `Reg<TDLR_SPEC>`"]
pub type TDLR = crate::Reg<tdlr::TDLR_SPEC>;
#[doc = "CAN_TDL0R"]
pub mod tdlr;
#[doc = "TDHR (rw) register accessor: an alias for `Reg<TDHR_SPEC>`"]
pub type TDHR = crate::Reg<tdhr::TDHR_SPEC>;
#[doc = "CAN_TDH0R"]
pub mod tdhr;
