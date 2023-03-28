#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr2: BTR,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr3: BCR,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr3: BTR,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr4: BCR,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr4: BTR,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: PCR,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: SR,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: PMEM2,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: PATT2,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: ECCR2,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - PC Card/NAND Flash control register 2"]
    pub pcr3: PCR,
    #[doc = "0x84 - FIFO status and interrupt register 2"]
    pub sr3: SR,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: PMEM3,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: PATT3,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: ECCR3,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - PC Card/NAND Flash control register 2"]
    pub pcr4: PCR,
    #[doc = "0xa4 - FIFO status and interrupt register 2"]
    pub sr4: SR,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: PMEM4,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: PATT4,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: BWTR,
    _reserved27: [u8; 0x20],
    #[doc = "0x140..0x148 - SDRAM Control Register 1"]
    pub sdcr: [SDCR; 2],
    #[doc = "0x148..0x150 - SDRAM Timing register 1"]
    pub sdtr: [SDTR; 2],
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub sdcmr: SDCMR,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub sdrtr: SDRTR,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: SDSR,
}
impl RegisterBlock {
    #[doc = "0x140 - SDRAM Control Register 1"]
    #[inline(always)]
    pub fn sdcr1(&self) -> &SDCR {
        &self.sdcr[0]
    }
    #[doc = "0x144 - SDRAM Control Register 1"]
    #[inline(always)]
    pub fn sdcr2(&self) -> &SDCR {
        &self.sdcr[1]
    }
    #[doc = "0x148 - SDRAM Timing register 1"]
    #[inline(always)]
    pub fn sdtr1(&self) -> &SDTR {
        &self.sdtr[0]
    }
    #[doc = "0x14c - SDRAM Timing register 1"]
    #[inline(always)]
    pub fn sdtr2(&self) -> &SDTR {
        &self.sdtr[1]
    }
}
#[doc = "BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr;
#[doc = "BCR (rw) register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FIFO status and interrupt register 2"]
pub mod sr;
#[doc = "PMEM2 (rw) register accessor: an alias for `Reg<PMEM2_SPEC>`"]
pub type PMEM2 = crate::Reg<pmem2::PMEM2_SPEC>;
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "PATT2 (rw) register accessor: an alias for `Reg<PATT2_SPEC>`"]
pub type PATT2 = crate::Reg<patt2::PATT2_SPEC>;
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECCR2 (r) register accessor: an alias for `Reg<ECCR2_SPEC>`"]
pub type ECCR2 = crate::Reg<eccr2::ECCR2_SPEC>;
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PMEM3 (rw) register accessor: an alias for `Reg<PMEM3_SPEC>`"]
pub type PMEM3 = crate::Reg<pmem3::PMEM3_SPEC>;
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "PATT3 (rw) register accessor: an alias for `Reg<PATT3_SPEC>`"]
pub type PATT3 = crate::Reg<patt3::PATT3_SPEC>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECCR3 (r) register accessor: an alias for `Reg<ECCR3_SPEC>`"]
pub type ECCR3 = crate::Reg<eccr3::ECCR3_SPEC>;
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PMEM4 (rw) register accessor: an alias for `Reg<PMEM4_SPEC>`"]
pub type PMEM4 = crate::Reg<pmem4::PMEM4_SPEC>;
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "PATT4 (rw) register accessor: an alias for `Reg<PATT4_SPEC>`"]
pub type PATT4 = crate::Reg<patt4::PATT4_SPEC>;
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "PIO4 (rw) register accessor: an alias for `Reg<PIO4_SPEC>`"]
pub type PIO4 = crate::Reg<pio4::PIO4_SPEC>;
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "BWTR (rw) register accessor: an alias for `Reg<BWTR_SPEC>`"]
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;
#[doc = "SDCR (rw) register accessor: an alias for `Reg<SDCR_SPEC>`"]
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
#[doc = "SDRAM Control Register 1"]
pub mod sdcr;
#[doc = "SDTR (rw) register accessor: an alias for `Reg<SDTR_SPEC>`"]
pub type SDTR = crate::Reg<sdtr::SDTR_SPEC>;
#[doc = "SDRAM Timing register 1"]
pub mod sdtr;
#[doc = "SDCMR (rw) register accessor: an alias for `Reg<SDCMR_SPEC>`"]
pub type SDCMR = crate::Reg<sdcmr::SDCMR_SPEC>;
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRTR (rw) register accessor: an alias for `Reg<SDRTR_SPEC>`"]
pub type SDRTR = crate::Reg<sdrtr::SDRTR_SPEC>;
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDSR (r) register accessor: an alias for `Reg<SDSR_SPEC>`"]
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
#[doc = "SDRAM Status register"]
pub mod sdsr;
