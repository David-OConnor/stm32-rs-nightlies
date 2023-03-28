#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub mcr: MCR,
    #[doc = "0x04 - CAN_MSR"]
    pub msr: MSR,
    #[doc = "0x08 - CAN_TSR"]
    pub tsr: TSR,
    #[doc = "0x0c..0x14 - CAN_RF%sR"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - CAN_IER"]
    pub ier: IER,
    #[doc = "0x18 - CAN_ESR"]
    pub esr: ESR,
    #[doc = "0x1c - CAN_BTR"]
    pub btr: BTR,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    pub rx: [RX; 2],
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "CAN_MCR"]
pub mod mcr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "CAN_MSR"]
pub mod msr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "CAN_TSR"]
pub mod tsr;
#[doc = "RFR (rw) register accessor: an alias for `Reg<RFR_SPEC>`"]
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
#[doc = "CAN_RF%sR"]
pub mod rfr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CAN_IER"]
pub mod ier;
#[doc = "ESR (rw) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "CAN_ESR"]
pub mod esr;
#[doc = "BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "CAN_BTR"]
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
