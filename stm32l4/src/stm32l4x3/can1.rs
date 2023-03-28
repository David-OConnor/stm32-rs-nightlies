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
    #[doc = "0x18 - interrupt enable register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved9: [u8; 0x70],
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
#[doc = "interrupt enable register"]
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
#[doc = "CAN Filter Bank cluster"]
pub use self::fb::FB;
#[doc = r"Cluster"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
