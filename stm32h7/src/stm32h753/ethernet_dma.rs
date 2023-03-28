#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA mode register"]
    pub dmamr: DMAMR,
    #[doc = "0x04 - System bus mode register"]
    pub dmasbmr: DMASBMR,
    #[doc = "0x08 - Interrupt status register"]
    pub dmaisr: DMAISR,
    #[doc = "0x0c - Debug status register"]
    pub dmadsr: DMADSR,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - Channel control register"]
    pub dmaccr: DMACCR,
    #[doc = "0x104 - Channel transmit control register"]
    pub dmactx_cr: DMACTX_CR,
    #[doc = "0x108 - Channel receive control register"]
    pub dmacrx_cr: DMACRX_CR,
    _reserved7: [u8; 0x08],
    #[doc = "0x114 - Channel Tx descriptor list address register"]
    pub dmactx_dlar: DMACTX_DLAR,
    _reserved8: [u8; 0x04],
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    pub dmacrx_dlar: DMACRX_DLAR,
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    pub dmactx_dtpr: DMACTX_DTPR,
    _reserved10: [u8; 0x04],
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    pub dmacrx_dtpr: DMACRX_DTPR,
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    pub dmactx_rlr: DMACTX_RLR,
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    pub dmacrx_rlr: DMACRX_RLR,
    #[doc = "0x134 - Channel interrupt enable register"]
    pub dmacier: DMACIER,
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    pub dmacrx_iwtr: DMACRX_IWTR,
    _reserved15: [u8; 0x08],
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    pub dmaccatx_dr: DMACCATX_DR,
    _reserved16: [u8; 0x04],
    #[doc = "0x14c - Channel current application receive descriptor register"]
    pub dmaccarx_dr: DMACCARX_DR,
    _reserved17: [u8; 0x04],
    #[doc = "0x154 - Channel current application transmit buffer register"]
    pub dmaccatx_br: DMACCATX_BR,
    _reserved18: [u8; 0x04],
    #[doc = "0x15c - Channel current application receive buffer register"]
    pub dmaccarx_br: DMACCARX_BR,
    #[doc = "0x160 - Channel status register"]
    pub dmacsr: DMACSR,
    _reserved20: [u8; 0x08],
    #[doc = "0x16c - Channel missed frame count register"]
    pub dmacmfcr: DMACMFCR,
}
#[doc = "DMAMR (rw) register accessor: an alias for `Reg<DMAMR_SPEC>`"]
pub type DMAMR = crate::Reg<dmamr::DMAMR_SPEC>;
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "DMASBMR (rw) register accessor: an alias for `Reg<DMASBMR_SPEC>`"]
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMR_SPEC>;
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "DMAISR (r) register accessor: an alias for `Reg<DMAISR_SPEC>`"]
pub type DMAISR = crate::Reg<dmaisr::DMAISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "DMADSR (r) register accessor: an alias for `Reg<DMADSR_SPEC>`"]
pub type DMADSR = crate::Reg<dmadsr::DMADSR_SPEC>;
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "DMACCR (rw) register accessor: an alias for `Reg<DMACCR_SPEC>`"]
pub type DMACCR = crate::Reg<dmaccr::DMACCR_SPEC>;
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "DMACTxCR (rw) register accessor: an alias for `Reg<DMACTX_CR_SPEC>`"]
pub type DMACTX_CR = crate::Reg<dmactx_cr::DMACTX_CR_SPEC>;
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "DMACRxCR (rw) register accessor: an alias for `Reg<DMACRX_CR_SPEC>`"]
pub type DMACRX_CR = crate::Reg<dmacrx_cr::DMACRX_CR_SPEC>;
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "DMACTxDLAR (rw) register accessor: an alias for `Reg<DMACTX_DLAR_SPEC>`"]
pub type DMACTX_DLAR = crate::Reg<dmactx_dlar::DMACTX_DLAR_SPEC>;
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "DMACRxDLAR (rw) register accessor: an alias for `Reg<DMACRX_DLAR_SPEC>`"]
pub type DMACRX_DLAR = crate::Reg<dmacrx_dlar::DMACRX_DLAR_SPEC>;
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "DMACTxDTPR (rw) register accessor: an alias for `Reg<DMACTX_DTPR_SPEC>`"]
pub type DMACTX_DTPR = crate::Reg<dmactx_dtpr::DMACTX_DTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "DMACRxDTPR (rw) register accessor: an alias for `Reg<DMACRX_DTPR_SPEC>`"]
pub type DMACRX_DTPR = crate::Reg<dmacrx_dtpr::DMACRX_DTPR_SPEC>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "DMACTxRLR (rw) register accessor: an alias for `Reg<DMACTX_RLR_SPEC>`"]
pub type DMACTX_RLR = crate::Reg<dmactx_rlr::DMACTX_RLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "DMACRxRLR (rw) register accessor: an alias for `Reg<DMACRX_RLR_SPEC>`"]
pub type DMACRX_RLR = crate::Reg<dmacrx_rlr::DMACRX_RLR_SPEC>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "DMACIER (rw) register accessor: an alias for `Reg<DMACIER_SPEC>`"]
pub type DMACIER = crate::Reg<dmacier::DMACIER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "DMACRxIWTR (rw) register accessor: an alias for `Reg<DMACRX_IWTR_SPEC>`"]
pub type DMACRX_IWTR = crate::Reg<dmacrx_iwtr::DMACRX_IWTR_SPEC>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "DMACCATxDR (r) register accessor: an alias for `Reg<DMACCATX_DR_SPEC>`"]
pub type DMACCATX_DR = crate::Reg<dmaccatx_dr::DMACCATX_DR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "DMACCARxDR (r) register accessor: an alias for `Reg<DMACCARX_DR_SPEC>`"]
pub type DMACCARX_DR = crate::Reg<dmaccarx_dr::DMACCARX_DR_SPEC>;
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "DMACCATxBR (r) register accessor: an alias for `Reg<DMACCATX_BR_SPEC>`"]
pub type DMACCATX_BR = crate::Reg<dmaccatx_br::DMACCATX_BR_SPEC>;
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "DMACCARxBR (r) register accessor: an alias for `Reg<DMACCARX_BR_SPEC>`"]
pub type DMACCARX_BR = crate::Reg<dmaccarx_br::DMACCARX_BR_SPEC>;
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "DMACSR (rw) register accessor: an alias for `Reg<DMACSR_SPEC>`"]
pub type DMACSR = crate::Reg<dmacsr::DMACSR_SPEC>;
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "DMACMFCR (r) register accessor: an alias for `Reg<DMACMFCR_SPEC>`"]
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
