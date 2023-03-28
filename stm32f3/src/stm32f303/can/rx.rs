#[doc = r"Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - receive FIFO mailbox identifier register"]
    pub rir: RIR,
    #[doc = "0x04 - receive FIFO mailbox data length control and time stamp register"]
    pub rdtr: RDTR,
    #[doc = "0x08 - receive FIFO mailbox data low register"]
    pub rdlr: RDLR,
    #[doc = "0x0c - receive FIFO mailbox data high register"]
    pub rdhr: RDHR,
}
#[doc = "RIR (r) register accessor: an alias for `Reg<RIR_SPEC>`"]
pub type RIR = crate::Reg<rir::RIR_SPEC>;
#[doc = "receive FIFO mailbox identifier register"]
pub mod rir;
#[doc = "RDTR (r) register accessor: an alias for `Reg<RDTR_SPEC>`"]
pub type RDTR = crate::Reg<rdtr::RDTR_SPEC>;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdtr;
#[doc = "RDLR (r) register accessor: an alias for `Reg<RDLR_SPEC>`"]
pub type RDLR = crate::Reg<rdlr::RDLR_SPEC>;
#[doc = "receive FIFO mailbox data low register"]
pub mod rdlr;
#[doc = "RDHR (r) register accessor: an alias for `Reg<RDHR_SPEC>`"]
pub type RDHR = crate::Reg<rdhr::RDHR_SPEC>;
#[doc = "receive FIFO mailbox data high register"]
pub mod rdhr;
