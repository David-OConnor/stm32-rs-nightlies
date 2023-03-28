#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - UCPD configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - UCPD configuration register 3"]
    pub cfgr3: CFGR3,
    #[doc = "0x0c - UCPD control register"]
    pub cr: CR,
    #[doc = "0x10 - UCPD interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x14 - UCPD status register"]
    pub sr: SR,
    #[doc = "0x18 - UCPD interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x1c - UCPD Tx ordered set type register"]
    pub tx_ordsetr: TX_ORDSETR,
    #[doc = "0x20 - UCPD Tx payload size register"]
    pub tx_payszr: TX_PAYSZR,
    #[doc = "0x24 - UCPD Tx data register"]
    pub txdr: TXDR,
    #[doc = "0x28 - UCPD Rx ordered set register"]
    pub rx_ordsetr: RX_ORDSETR,
    #[doc = "0x2c - UCPD Rx payload size register"]
    pub rx_payszr: RX_PAYSZR,
    #[doc = "0x30 - UCPD receive data register"]
    pub rxdr: RXDR,
    #[doc = "0x34 - UCPD Rx ordered set extension register 1"]
    pub rx_ordextr1: RX_ORDEXTR1,
    #[doc = "0x38 - UCPD Rx ordered set extension register 2"]
    pub rx_ordextr2: RX_ORDEXTR2,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "UCPD configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "UCPD configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "UCPD configuration register 3"]
pub mod cfgr3;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "UCPD control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "UCPD interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "UCPD status register"]
pub mod sr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "UCPD interrupt clear register"]
pub mod icr;
#[doc = "TX_ORDSETR (rw) register accessor: an alias for `Reg<TX_ORDSETR_SPEC>`"]
pub type TX_ORDSETR = crate::Reg<tx_ordsetr::TX_ORDSETR_SPEC>;
#[doc = "UCPD Tx ordered set type register"]
pub mod tx_ordsetr;
#[doc = "TX_PAYSZR (rw) register accessor: an alias for `Reg<TX_PAYSZR_SPEC>`"]
pub type TX_PAYSZR = crate::Reg<tx_payszr::TX_PAYSZR_SPEC>;
#[doc = "UCPD Tx payload size register"]
pub mod tx_payszr;
#[doc = "TXDR (rw) register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "UCPD Tx data register"]
pub mod txdr;
#[doc = "RX_ORDSETR (r) register accessor: an alias for `Reg<RX_ORDSETR_SPEC>`"]
pub type RX_ORDSETR = crate::Reg<rx_ordsetr::RX_ORDSETR_SPEC>;
#[doc = "UCPD Rx ordered set register"]
pub mod rx_ordsetr;
#[doc = "RX_PAYSZR (r) register accessor: an alias for `Reg<RX_PAYSZR_SPEC>`"]
pub type RX_PAYSZR = crate::Reg<rx_payszr::RX_PAYSZR_SPEC>;
#[doc = "UCPD Rx payload size register"]
pub mod rx_payszr;
#[doc = "RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "UCPD receive data register"]
pub mod rxdr;
#[doc = "RX_ORDEXTR1 (rw) register accessor: an alias for `Reg<RX_ORDEXTR1_SPEC>`"]
pub type RX_ORDEXTR1 = crate::Reg<rx_ordextr1::RX_ORDEXTR1_SPEC>;
#[doc = "UCPD Rx ordered set extension register 1"]
pub mod rx_ordextr1;
#[doc = "RX_ORDEXTR2 (rw) register accessor: an alias for `Reg<RX_ORDEXTR2_SPEC>`"]
pub type RX_ORDEXTR2 = crate::Reg<rx_ordextr2::RX_ORDEXTR2_SPEC>;
#[doc = "UCPD Rx ordered set extension register 2"]
pub mod rx_ordextr2;
