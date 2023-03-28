#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC_POWER"]
    pub sdmmc_power: SDMMC_POWER,
    #[doc = "0x04 - SDMMC clock control register"]
    pub sdmmc_clkcr: SDMMC_CLKCR,
    #[doc = "0x08 - SDMMC argument register"]
    pub sdmmc_argr: SDMMC_ARGR,
    #[doc = "0x0c - SDMMC command register"]
    pub sdmmc_cmdr: SDMMC_CMDR,
    #[doc = "0x10 - SDMMC command response register"]
    pub sdmmc_respcmdr: SDMMC_RESPCMDR,
    #[doc = "0x14 - SDMMC response 1 register"]
    pub sdmmc_resp1r: SDMMC_RESP1R,
    #[doc = "0x18 - SDMMC response 2 register"]
    pub sdmmc_resp2r: SDMMC_RESP2R,
    #[doc = "0x1c - SDMMC response 3 register"]
    pub sdmmc_resp3r: SDMMC_RESP3R,
    #[doc = "0x20 - SDMMC response 4 register"]
    pub sdmmc_resp4r: SDMMC_RESP4R,
    #[doc = "0x24 - SDMMC data timer register"]
    pub sdmmc_dtimer: SDMMC_DTIMER,
    #[doc = "0x28 - SDMMC data length register"]
    pub sdmmc_dlenr: SDMMC_DLENR,
    #[doc = "0x2c - SDMMC data control register"]
    pub sdmmc_dctrl: SDMMC_DCTRL,
    #[doc = "0x30 - SDMMC data counter register"]
    pub sdmmc_dcntr: SDMMC_DCNTR,
    #[doc = "0x34 - SDMMC status register"]
    pub sdmmc_star: SDMMC_STAR,
    #[doc = "0x38 - SDMMC interrupt clear register"]
    pub sdmmc_icr: SDMMC_ICR,
    #[doc = "0x3c - SDMMC mask register"]
    pub sdmmc_maskr: SDMMC_MASKR,
    #[doc = "0x40 - SDMMC acknowledgment timer register"]
    pub sdmmc_acktimer: SDMMC_ACKTIMER,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - SDMMC DMA control register"]
    pub sdmmc_idmactrlr: SDMMC_IDMACTRLR,
    #[doc = "0x54 - SDMMC IDMA buffer size register"]
    pub sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    #[doc = "0x58 - SDMMC IDMA buffer base address register"]
    pub sdmmc_idmabaser: SDMMC_IDMABASER,
    _reserved20: [u8; 0x08],
    #[doc = "0x64 - SDMMC_IDMALAR"]
    pub sdmmc_idmalar: SDMMC_IDMALAR,
    #[doc = "0x68 - SDMMC_IDMABAR"]
    pub sdmmc_idmabar: SDMMC_IDMABAR,
    _reserved22: [u8; 0x14],
    #[doc = "0x80 - SDMMC data FIFO registers 0"]
    pub sdmmc_fifor0: SDMMC_FIFOR0,
    #[doc = "0x84 - SDMMC data FIFO registers 1"]
    pub sdmmc_fifor1: SDMMC_FIFOR1,
    #[doc = "0x88 - SDMMC data FIFO registers 2"]
    pub sdmmc_fifor2: SDMMC_FIFOR2,
    #[doc = "0x8c - SDMMC data FIFO registers 3"]
    pub sdmmc_fifor3: SDMMC_FIFOR3,
    #[doc = "0x90 - SDMMC data FIFO registers 4"]
    pub sdmmc_fifor4: SDMMC_FIFOR4,
    #[doc = "0x94 - SDMMC data FIFO registers 5"]
    pub sdmmc_fifor5: SDMMC_FIFOR5,
    #[doc = "0x98 - SDMMC data FIFO registers 6"]
    pub sdmmc_fifor6: SDMMC_FIFOR6,
    #[doc = "0x9c - SDMMC data FIFO registers 7"]
    pub sdmmc_fifor7: SDMMC_FIFOR7,
    #[doc = "0xa0 - SDMMC data FIFO registers 8"]
    pub sdmmc_fifor8: SDMMC_FIFOR8,
    #[doc = "0xa4 - SDMMC data FIFO registers 9"]
    pub sdmmc_fifor9: SDMMC_FIFOR9,
    #[doc = "0xa8 - SDMMC data FIFO registers 10"]
    pub sdmmc_fifor10: SDMMC_FIFOR10,
    #[doc = "0xac - SDMMC data FIFO registers 11"]
    pub sdmmc_fifor11: SDMMC_FIFOR11,
    #[doc = "0xb0 - SDMMC data FIFO registers 12"]
    pub sdmmc_fifor12: SDMMC_FIFOR12,
    #[doc = "0xb4 - SDMMC data FIFO registers 13"]
    pub sdmmc_fifor13: SDMMC_FIFOR13,
    #[doc = "0xb8 - SDMMC data FIFO registers 14"]
    pub sdmmc_fifor14: SDMMC_FIFOR14,
    #[doc = "0xbc - SDMMC data FIFO registers 15"]
    pub sdmmc_fifor15: SDMMC_FIFOR15,
}
#[doc = "SDMMC_POWER (rw) register accessor: an alias for `Reg<SDMMC_POWER_SPEC>`"]
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWER_SPEC>;
#[doc = "SDMMC_POWER"]
pub mod sdmmc_power;
#[doc = "SDMMC_CLKCR (rw) register accessor: an alias for `Reg<SDMMC_CLKCR_SPEC>`"]
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCR_SPEC>;
#[doc = "SDMMC clock control register"]
pub mod sdmmc_clkcr;
#[doc = "SDMMC_ARGR (rw) register accessor: an alias for `Reg<SDMMC_ARGR_SPEC>`"]
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGR_SPEC>;
#[doc = "SDMMC argument register"]
pub mod sdmmc_argr;
#[doc = "SDMMC_CMDR (rw) register accessor: an alias for `Reg<SDMMC_CMDR_SPEC>`"]
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDR_SPEC>;
#[doc = "SDMMC command register"]
pub mod sdmmc_cmdr;
#[doc = "SDMMC_RESPCMDR (r) register accessor: an alias for `Reg<SDMMC_RESPCMDR_SPEC>`"]
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDR_SPEC>;
#[doc = "SDMMC command response register"]
pub mod sdmmc_respcmdr;
#[doc = "SDMMC_RESP1R (r) register accessor: an alias for `Reg<SDMMC_RESP1R_SPEC>`"]
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1R_SPEC>;
#[doc = "SDMMC response 1 register"]
pub mod sdmmc_resp1r;
#[doc = "SDMMC_RESP2R (r) register accessor: an alias for `Reg<SDMMC_RESP2R_SPEC>`"]
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2R_SPEC>;
#[doc = "SDMMC response 2 register"]
pub mod sdmmc_resp2r;
#[doc = "SDMMC_RESP3R (r) register accessor: an alias for `Reg<SDMMC_RESP3R_SPEC>`"]
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3R_SPEC>;
#[doc = "SDMMC response 3 register"]
pub mod sdmmc_resp3r;
#[doc = "SDMMC_RESP4R (r) register accessor: an alias for `Reg<SDMMC_RESP4R_SPEC>`"]
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4R_SPEC>;
#[doc = "SDMMC response 4 register"]
pub mod sdmmc_resp4r;
#[doc = "SDMMC_DTIMER (rw) register accessor: an alias for `Reg<SDMMC_DTIMER_SPEC>`"]
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMER_SPEC>;
#[doc = "SDMMC data timer register"]
pub mod sdmmc_dtimer;
#[doc = "SDMMC_DLENR (rw) register accessor: an alias for `Reg<SDMMC_DLENR_SPEC>`"]
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENR_SPEC>;
#[doc = "SDMMC data length register"]
pub mod sdmmc_dlenr;
#[doc = "SDMMC_DCTRL (rw) register accessor: an alias for `Reg<SDMMC_DCTRL_SPEC>`"]
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRL_SPEC>;
#[doc = "SDMMC data control register"]
pub mod sdmmc_dctrl;
#[doc = "SDMMC_DCNTR (r) register accessor: an alias for `Reg<SDMMC_DCNTR_SPEC>`"]
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTR_SPEC>;
#[doc = "SDMMC data counter register"]
pub mod sdmmc_dcntr;
#[doc = "SDMMC_STAR (r) register accessor: an alias for `Reg<SDMMC_STAR_SPEC>`"]
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STAR_SPEC>;
#[doc = "SDMMC status register"]
pub mod sdmmc_star;
#[doc = "SDMMC_ICR (rw) register accessor: an alias for `Reg<SDMMC_ICR_SPEC>`"]
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICR_SPEC>;
#[doc = "SDMMC interrupt clear register"]
pub mod sdmmc_icr;
#[doc = "SDMMC_MASKR (rw) register accessor: an alias for `Reg<SDMMC_MASKR_SPEC>`"]
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKR_SPEC>;
#[doc = "SDMMC mask register"]
pub mod sdmmc_maskr;
#[doc = "SDMMC_ACKTIMER (rw) register accessor: an alias for `Reg<SDMMC_ACKTIMER_SPEC>`"]
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMER_SPEC>;
#[doc = "SDMMC acknowledgment timer register"]
pub mod sdmmc_acktimer;
#[doc = "SDMMC_IDMACTRLR (rw) register accessor: an alias for `Reg<SDMMC_IDMACTRLR_SPEC>`"]
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLR_SPEC>;
#[doc = "SDMMC DMA control register"]
pub mod sdmmc_idmactrlr;
#[doc = "SDMMC_IDMABSIZER (rw) register accessor: an alias for `Reg<SDMMC_IDMABSIZER_SPEC>`"]
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZER_SPEC>;
#[doc = "SDMMC IDMA buffer size register"]
pub mod sdmmc_idmabsizer;
#[doc = "SDMMC_IDMABASER (rw) register accessor: an alias for `Reg<SDMMC_IDMABASER_SPEC>`"]
pub type SDMMC_IDMABASER = crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASER_SPEC>;
#[doc = "SDMMC IDMA buffer base address register"]
pub mod sdmmc_idmabaser;
#[doc = "SDMMC_IDMALAR (rw) register accessor: an alias for `Reg<SDMMC_IDMALAR_SPEC>`"]
pub type SDMMC_IDMALAR = crate::Reg<sdmmc_idmalar::SDMMC_IDMALAR_SPEC>;
#[doc = "SDMMC_IDMALAR"]
pub mod sdmmc_idmalar;
#[doc = "SDMMC_IDMABAR (rw) register accessor: an alias for `Reg<SDMMC_IDMABAR_SPEC>`"]
pub type SDMMC_IDMABAR = crate::Reg<sdmmc_idmabar::SDMMC_IDMABAR_SPEC>;
#[doc = "SDMMC_IDMABAR"]
pub mod sdmmc_idmabar;
#[doc = "SDMMC_FIFOR0 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR0_SPEC>`"]
pub type SDMMC_FIFOR0 = crate::Reg<sdmmc_fifor0::SDMMC_FIFOR0_SPEC>;
#[doc = "SDMMC data FIFO registers 0"]
pub mod sdmmc_fifor0;
#[doc = "SDMMC_FIFOR1 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR1_SPEC>`"]
pub type SDMMC_FIFOR1 = crate::Reg<sdmmc_fifor1::SDMMC_FIFOR1_SPEC>;
#[doc = "SDMMC data FIFO registers 1"]
pub mod sdmmc_fifor1;
#[doc = "SDMMC_FIFOR2 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR2_SPEC>`"]
pub type SDMMC_FIFOR2 = crate::Reg<sdmmc_fifor2::SDMMC_FIFOR2_SPEC>;
#[doc = "SDMMC data FIFO registers 2"]
pub mod sdmmc_fifor2;
#[doc = "SDMMC_FIFOR3 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR3_SPEC>`"]
pub type SDMMC_FIFOR3 = crate::Reg<sdmmc_fifor3::SDMMC_FIFOR3_SPEC>;
#[doc = "SDMMC data FIFO registers 3"]
pub mod sdmmc_fifor3;
#[doc = "SDMMC_FIFOR4 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR4_SPEC>`"]
pub type SDMMC_FIFOR4 = crate::Reg<sdmmc_fifor4::SDMMC_FIFOR4_SPEC>;
#[doc = "SDMMC data FIFO registers 4"]
pub mod sdmmc_fifor4;
#[doc = "SDMMC_FIFOR5 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR5_SPEC>`"]
pub type SDMMC_FIFOR5 = crate::Reg<sdmmc_fifor5::SDMMC_FIFOR5_SPEC>;
#[doc = "SDMMC data FIFO registers 5"]
pub mod sdmmc_fifor5;
#[doc = "SDMMC_FIFOR6 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR6_SPEC>`"]
pub type SDMMC_FIFOR6 = crate::Reg<sdmmc_fifor6::SDMMC_FIFOR6_SPEC>;
#[doc = "SDMMC data FIFO registers 6"]
pub mod sdmmc_fifor6;
#[doc = "SDMMC_FIFOR7 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR7_SPEC>`"]
pub type SDMMC_FIFOR7 = crate::Reg<sdmmc_fifor7::SDMMC_FIFOR7_SPEC>;
#[doc = "SDMMC data FIFO registers 7"]
pub mod sdmmc_fifor7;
#[doc = "SDMMC_FIFOR8 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR8_SPEC>`"]
pub type SDMMC_FIFOR8 = crate::Reg<sdmmc_fifor8::SDMMC_FIFOR8_SPEC>;
#[doc = "SDMMC data FIFO registers 8"]
pub mod sdmmc_fifor8;
#[doc = "SDMMC_FIFOR9 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR9_SPEC>`"]
pub type SDMMC_FIFOR9 = crate::Reg<sdmmc_fifor9::SDMMC_FIFOR9_SPEC>;
#[doc = "SDMMC data FIFO registers 9"]
pub mod sdmmc_fifor9;
#[doc = "SDMMC_FIFOR10 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR10_SPEC>`"]
pub type SDMMC_FIFOR10 = crate::Reg<sdmmc_fifor10::SDMMC_FIFOR10_SPEC>;
#[doc = "SDMMC data FIFO registers 10"]
pub mod sdmmc_fifor10;
#[doc = "SDMMC_FIFOR11 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR11_SPEC>`"]
pub type SDMMC_FIFOR11 = crate::Reg<sdmmc_fifor11::SDMMC_FIFOR11_SPEC>;
#[doc = "SDMMC data FIFO registers 11"]
pub mod sdmmc_fifor11;
#[doc = "SDMMC_FIFOR12 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR12_SPEC>`"]
pub type SDMMC_FIFOR12 = crate::Reg<sdmmc_fifor12::SDMMC_FIFOR12_SPEC>;
#[doc = "SDMMC data FIFO registers 12"]
pub mod sdmmc_fifor12;
#[doc = "SDMMC_FIFOR13 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR13_SPEC>`"]
pub type SDMMC_FIFOR13 = crate::Reg<sdmmc_fifor13::SDMMC_FIFOR13_SPEC>;
#[doc = "SDMMC data FIFO registers 13"]
pub mod sdmmc_fifor13;
#[doc = "SDMMC_FIFOR14 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR14_SPEC>`"]
pub type SDMMC_FIFOR14 = crate::Reg<sdmmc_fifor14::SDMMC_FIFOR14_SPEC>;
#[doc = "SDMMC data FIFO registers 14"]
pub mod sdmmc_fifor14;
#[doc = "SDMMC_FIFOR15 (rw) register accessor: an alias for `Reg<SDMMC_FIFOR15_SPEC>`"]
pub type SDMMC_FIFOR15 = crate::Reg<sdmmc_fifor15::SDMMC_FIFOR15_SPEC>;
#[doc = "SDMMC data FIFO registers 15"]
pub mod sdmmc_fifor15;
