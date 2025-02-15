#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - SPI control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - SPI status register"]
    pub sr: SR,
    #[doc = "0x0c - SPI data register"]
    pub dr: DR,
    #[doc = "0x10 - SPI CRC polynomial register"]
    pub crcpr: CRCPR,
    #[doc = "0x14 - SPI Rx CRC register"]
    pub rxcrcr: RXCRCR,
    #[doc = "0x18 - SPI Tx CRC register"]
    pub txcrcr: TXCRCR,
    #[doc = "0x1c - SPI_I2S configuration register"]
    pub i2scfgr: I2SCFGR,
    #[doc = "0x20 - SPI_I2S prescaler register"]
    pub i2spr: I2SPR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "SPI control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "SPI control register 2"]
pub mod cr2;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SPI status register"]
pub mod sr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "SPI data register"]
pub mod dr;
#[doc = "CRCPR (rw) register accessor: an alias for `Reg<CRCPR_SPEC>`"]
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
#[doc = "SPI CRC polynomial register"]
pub mod crcpr;
#[doc = "RXCRCR (r) register accessor: an alias for `Reg<RXCRCR_SPEC>`"]
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCR_SPEC>;
#[doc = "SPI Rx CRC register"]
pub mod rxcrcr;
#[doc = "TXCRCR (r) register accessor: an alias for `Reg<TXCRCR_SPEC>`"]
pub type TXCRCR = crate::Reg<txcrcr::TXCRCR_SPEC>;
#[doc = "SPI Tx CRC register"]
pub mod txcrcr;
#[doc = "I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "SPI_I2S configuration register"]
pub mod i2scfgr;
#[doc = "I2SPR (rw) register accessor: an alias for `Reg<I2SPR_SPEC>`"]
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
#[doc = "SPI_I2S prescaler register"]
pub mod i2spr;
