#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    pub ucpd_cfgr1: UCPD_CFGR1,
    #[doc = "0x04 - UCPD configuration register 2"]
    pub ucpd_cfgr2: UCPD_CFGR2,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - UCPD control register"]
    pub ucpd_cr: UCPD_CR,
    #[doc = "0x10 - UCPD interrupt mask register"]
    pub ucpd_imr: UCPD_IMR,
    #[doc = "0x14 - UCPD status register"]
    pub ucpd_sr: UCPD_SR,
    #[doc = "0x18 - UCPD interrupt clear register"]
    pub ucpd_icr: UCPD_ICR,
    #[doc = "0x1c - UCPD Tx ordered set type register"]
    pub ucpd_tx_ordsetr: UCPD_TX_ORDSETR,
    #[doc = "0x20 - UCPD Tx payload size register"]
    pub ucpd_tx_payszr: UCPD_TX_PAYSZR,
    #[doc = "0x24 - UCPD Tx data register"]
    pub ucpd_txdr: UCPD_TXDR,
    #[doc = "0x28 - UCPD Rx ordered set register"]
    pub ucpd_rx_ordsetr: UCPD_RX_ORDSETR,
    #[doc = "0x2c - UCPD Rx payload size register"]
    pub ucpd_rx_payszr: UCPD_RX_PAYSZR,
    #[doc = "0x30 - UCPD receive data register"]
    pub ucpd_rxdr: UCPD_RXDR,
    #[doc = "0x34 - UCPD Rx ordered set extension register 1"]
    pub ucpd_rx_ordextr1: UCPD_RX_ORDEXTR1,
    #[doc = "0x38 - UCPD Rx ordered set extension register 2"]
    pub ucpd_rx_ordextr2: UCPD_RX_ORDEXTR2,
}
#[doc = "UCPD_CFGR1 (rw) register accessor: an alias for `Reg<UCPD_CFGR1_SPEC>`"]
pub type UCPD_CFGR1 = crate::Reg<ucpd_cfgr1::UCPD_CFGR1_SPEC>;
#[doc = "UCPD configuration register 1"]
pub mod ucpd_cfgr1;
#[doc = "UCPD_CFGR2 (rw) register accessor: an alias for `Reg<UCPD_CFGR2_SPEC>`"]
pub type UCPD_CFGR2 = crate::Reg<ucpd_cfgr2::UCPD_CFGR2_SPEC>;
#[doc = "UCPD configuration register 2"]
pub mod ucpd_cfgr2;
#[doc = "UCPD_CR (rw) register accessor: an alias for `Reg<UCPD_CR_SPEC>`"]
pub type UCPD_CR = crate::Reg<ucpd_cr::UCPD_CR_SPEC>;
#[doc = "UCPD control register"]
pub mod ucpd_cr;
#[doc = "UCPD_IMR (rw) register accessor: an alias for `Reg<UCPD_IMR_SPEC>`"]
pub type UCPD_IMR = crate::Reg<ucpd_imr::UCPD_IMR_SPEC>;
#[doc = "UCPD interrupt mask register"]
pub mod ucpd_imr;
#[doc = "UCPD_SR (r) register accessor: an alias for `Reg<UCPD_SR_SPEC>`"]
pub type UCPD_SR = crate::Reg<ucpd_sr::UCPD_SR_SPEC>;
#[doc = "UCPD status register"]
pub mod ucpd_sr;
#[doc = "UCPD_ICR (w) register accessor: an alias for `Reg<UCPD_ICR_SPEC>`"]
pub type UCPD_ICR = crate::Reg<ucpd_icr::UCPD_ICR_SPEC>;
#[doc = "UCPD interrupt clear register"]
pub mod ucpd_icr;
#[doc = "UCPD_TX_ORDSETR (rw) register accessor: an alias for `Reg<UCPD_TX_ORDSETR_SPEC>`"]
pub type UCPD_TX_ORDSETR = crate::Reg<ucpd_tx_ordsetr::UCPD_TX_ORDSETR_SPEC>;
#[doc = "UCPD Tx ordered set type register"]
pub mod ucpd_tx_ordsetr;
#[doc = "UCPD_TX_PAYSZR (rw) register accessor: an alias for `Reg<UCPD_TX_PAYSZR_SPEC>`"]
pub type UCPD_TX_PAYSZR = crate::Reg<ucpd_tx_payszr::UCPD_TX_PAYSZR_SPEC>;
#[doc = "UCPD Tx payload size register"]
pub mod ucpd_tx_payszr;
#[doc = "UCPD_TXDR (rw) register accessor: an alias for `Reg<UCPD_TXDR_SPEC>`"]
pub type UCPD_TXDR = crate::Reg<ucpd_txdr::UCPD_TXDR_SPEC>;
#[doc = "UCPD Tx data register"]
pub mod ucpd_txdr;
#[doc = "UCPD_RX_ORDSETR (r) register accessor: an alias for `Reg<UCPD_RX_ORDSETR_SPEC>`"]
pub type UCPD_RX_ORDSETR = crate::Reg<ucpd_rx_ordsetr::UCPD_RX_ORDSETR_SPEC>;
#[doc = "UCPD Rx ordered set register"]
pub mod ucpd_rx_ordsetr;
#[doc = "UCPD_RX_PAYSZR (r) register accessor: an alias for `Reg<UCPD_RX_PAYSZR_SPEC>`"]
pub type UCPD_RX_PAYSZR = crate::Reg<ucpd_rx_payszr::UCPD_RX_PAYSZR_SPEC>;
#[doc = "UCPD Rx payload size register"]
pub mod ucpd_rx_payszr;
#[doc = "UCPD_RXDR (r) register accessor: an alias for `Reg<UCPD_RXDR_SPEC>`"]
pub type UCPD_RXDR = crate::Reg<ucpd_rxdr::UCPD_RXDR_SPEC>;
#[doc = "UCPD receive data register"]
pub mod ucpd_rxdr;
#[doc = "UCPD_RX_ORDEXTR1 (rw) register accessor: an alias for `Reg<UCPD_RX_ORDEXTR1_SPEC>`"]
pub type UCPD_RX_ORDEXTR1 = crate::Reg<ucpd_rx_ordextr1::UCPD_RX_ORDEXTR1_SPEC>;
#[doc = "UCPD Rx ordered set extension register 1"]
pub mod ucpd_rx_ordextr1;
#[doc = "UCPD_RX_ORDEXTR2 (rw) register accessor: an alias for `Reg<UCPD_RX_ORDEXTR2_SPEC>`"]
pub type UCPD_RX_ORDEXTR2 = crate::Reg<ucpd_rx_ordextr2::UCPD_RX_ORDEXTR2_SPEC>;
#[doc = "UCPD Rx ordered set extension register 2"]
pub mod ucpd_rx_ordextr2;
