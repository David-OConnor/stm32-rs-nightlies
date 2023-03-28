#[doc = r"Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - CAN_RI0R"]
    pub rir: RIR,
    #[doc = "0x04 - CAN_RDT0R"]
    pub rdtr: RDTR,
    #[doc = "0x08 - CAN_RDL0R"]
    pub rdlr: RDLR,
    #[doc = "0x0c - CAN_RDH0R"]
    pub rdhr: RDHR,
}
#[doc = "RIR (r) register accessor: an alias for `Reg<RIR_SPEC>`"]
pub type RIR = crate::Reg<rir::RIR_SPEC>;
#[doc = "CAN_RI0R"]
pub mod rir;
#[doc = "RDTR (r) register accessor: an alias for `Reg<RDTR_SPEC>`"]
pub type RDTR = crate::Reg<rdtr::RDTR_SPEC>;
#[doc = "CAN_RDT0R"]
pub mod rdtr;
#[doc = "RDLR (r) register accessor: an alias for `Reg<RDLR_SPEC>`"]
pub type RDLR = crate::Reg<rdlr::RDLR_SPEC>;
#[doc = "CAN_RDL0R"]
pub mod rdlr;
#[doc = "RDHR (r) register accessor: an alias for `Reg<RDHR_SPEC>`"]
pub type RDHR = crate::Reg<rdhr::RDHR_SPEC>;
#[doc = "CAN_RDH0R"]
pub mod rdhr;
