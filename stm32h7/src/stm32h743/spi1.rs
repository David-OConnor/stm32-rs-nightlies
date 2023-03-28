#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - configuration register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Interrupt/Status Flags Clear Register"]
    pub ifcr: IFCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Transmit Data Register"]
    pub txdr: TXDR,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Receive Data Register"]
    pub rxdr: RXDR,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - Polynomial Register"]
    pub crcpoly: CRCPOLY,
    #[doc = "0x44 - Transmitter CRC Register"]
    pub txcrc: TXCRC,
    #[doc = "0x48 - Receiver CRC Register"]
    pub rxcrc: RXCRC,
    #[doc = "0x4c - Underrun Data Register"]
    pub udrdr: UDRDR,
    #[doc = "0x50 - configuration register"]
    pub i2scfgr: I2SCFGR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "configuration register 2"]
pub mod cfg2;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt/Status Flags Clear Register"]
pub mod ifcr;
#[doc = "TXDR (w) register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod txdr;
#[doc = "RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rxdr;
#[doc = "CRCPOLY (rw) register accessor: an alias for `Reg<CRCPOLY_SPEC>`"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "Polynomial Register"]
pub mod crcpoly;
#[doc = "TXCRC (rw) register accessor: an alias for `Reg<TXCRC_SPEC>`"]
pub type TXCRC = crate::Reg<txcrc::TXCRC_SPEC>;
#[doc = "Transmitter CRC Register"]
pub mod txcrc;
#[doc = "RXCRC (rw) register accessor: an alias for `Reg<RXCRC_SPEC>`"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "Receiver CRC Register"]
pub mod rxcrc;
#[doc = "UDRDR (rw) register accessor: an alias for `Reg<UDRDR_SPEC>`"]
pub type UDRDR = crate::Reg<udrdr::UDRDR_SPEC>;
#[doc = "Underrun Data Register"]
pub mod udrdr;
#[doc = "I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "configuration register"]
pub mod i2scfgr;
