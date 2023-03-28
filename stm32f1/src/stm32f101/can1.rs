#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c..0x14 - receive FIFO %s register"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - error status register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved9: [u8; 0x30],
    #[doc = "0x200 - filter master register"]
    pub fmr: FMR,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: FM1R,
    _reserved11: [u8; 0x04],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: FS1R,
    _reserved12: [u8; 0x04],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: FFA1R,
    _reserved13: [u8; 0x04],
    #[doc = "0x21c - filter activation register"]
    pub fa1r: FA1R,
    _reserved14: [u8; 0x20],
    #[doc = "0x240..0x320 - CAN Filter Bank cluster"]
    pub fb: [FB; 28],
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "master control register"]
pub mod mcr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "master status register"]
pub mod msr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "RFR (rw) register accessor: an alias for `Reg<RFR_SPEC>`"]
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
#[doc = "receive FIFO %s register"]
pub mod rfr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ESR (rw) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "error status register"]
pub mod esr;
#[doc = "BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "bit timing register"]
pub mod btr;
#[doc = "CAN Transmit cluster"]
pub use self::tx::TX;
#[doc = r"Cluster"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = "CAN Receive cluster"]
pub use self::rx::RX;
#[doc = r"Cluster"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = "FMR (rw) register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "filter master register"]
pub mod fmr;
#[doc = "FM1R (rw) register accessor: an alias for `Reg<FM1R_SPEC>`"]
pub type FM1R = crate::Reg<fm1r::FM1R_SPEC>;
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "FS1R (rw) register accessor: an alias for `Reg<FS1R_SPEC>`"]
pub type FS1R = crate::Reg<fs1r::FS1R_SPEC>;
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "FFA1R (rw) register accessor: an alias for `Reg<FFA1R_SPEC>`"]
pub type FFA1R = crate::Reg<ffa1r::FFA1R_SPEC>;
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "FA1R (rw) register accessor: an alias for `Reg<FA1R_SPEC>`"]
pub type FA1R = crate::Reg<fa1r::FA1R_SPEC>;
#[doc = "filter activation register"]
pub mod fa1r;
#[doc = "CAN Filter Bank cluster"]
pub use self::fb::FB;
#[doc = r"Cluster"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
