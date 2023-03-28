#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register for bank 1"]
    pub fmc_bcr1: FMC_BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register for bank 1"]
    pub fmc_btr1: FMC_BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register for bank 2"]
    pub fmc_bcr2: FMC_BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register for bank 2"]
    pub fmc_btr2: FMC_BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register for bank 3"]
    pub fmc_bcr3: FMC_BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register for bank 3"]
    pub fmc_btr3: FMC_BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register for bank 4"]
    pub fmc_bcr4: FMC_BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register for bank 4"]
    pub fmc_btr4: FMC_BTR4,
    #[doc = "0x20 - PSRAM chip select counter register"]
    pub fmc_pcscntr: FMC_PCSCNTR,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - NAND Flash control registers"]
    pub fmc_pcr: FMC_PCR,
    #[doc = "0x84 - FIFO status and interrupt register"]
    pub fmc_sr: FMC_SR,
    #[doc = "0x88 - Common memory space timing register"]
    pub fmc_pmem: FMC_PMEM,
    #[doc = "0x8c - Attribute memory space timing register"]
    pub fmc_patt: FMC_PATT,
    _reserved13: [u8; 0x04],
    #[doc = "0x94 - ECC result registers"]
    pub fmc_eccr: FMC_ECCR,
    _reserved14: [u8; 0x6c],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub fmc_bwtr1: FMC_BWTR1,
    _reserved15: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub fmc_bwtr2: FMC_BWTR2,
    _reserved16: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub fmc_bwtr3: FMC_BWTR3,
    _reserved17: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub fmc_bwtr4: FMC_BWTR4,
    _reserved18: [u8; 0x20],
    #[doc = "0x140 - SDRAM control registers 1"]
    pub fmc_sdcr1: FMC_SDCR1,
    #[doc = "0x144 - SDRAM control registers 2"]
    pub fmc_sdcr2: FMC_SDCR2,
    #[doc = "0x148 - SDRAM timing registers 1"]
    pub fmc_sdtr1: FMC_SDTR1,
    #[doc = "0x14c - SDRAM timing registers 2"]
    pub fmc_sdtr2: FMC_SDTR2,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub fmc_sdcmr: FMC_SDCMR,
    #[doc = "0x154 - SDRAM refresh timer register"]
    pub fmc_sdrtr: FMC_SDRTR,
    #[doc = "0x158 - SDRAM status register"]
    pub fmc_sdsr: FMC_SDSR,
}
#[doc = "FMC_BCR1 (rw) register accessor: an alias for `Reg<FMC_BCR1_SPEC>`"]
pub type FMC_BCR1 = crate::Reg<fmc_bcr1::FMC_BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register for bank 1"]
pub mod fmc_bcr1;
#[doc = "FMC_BTR1 (rw) register accessor: an alias for `Reg<FMC_BTR1_SPEC>`"]
pub type FMC_BTR1 = crate::Reg<fmc_btr1::FMC_BTR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 1"]
pub mod fmc_btr1;
#[doc = "FMC_BCR2 (rw) register accessor: an alias for `Reg<FMC_BCR2_SPEC>`"]
pub type FMC_BCR2 = crate::Reg<fmc_bcr2::FMC_BCR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register for bank 2"]
pub mod fmc_bcr2;
#[doc = "FMC_BTR2 (rw) register accessor: an alias for `Reg<FMC_BTR2_SPEC>`"]
pub type FMC_BTR2 = crate::Reg<fmc_btr2::FMC_BTR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 2"]
pub mod fmc_btr2;
#[doc = "FMC_BCR3 (rw) register accessor: an alias for `Reg<FMC_BCR3_SPEC>`"]
pub type FMC_BCR3 = crate::Reg<fmc_bcr3::FMC_BCR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register for bank 3"]
pub mod fmc_bcr3;
#[doc = "FMC_BTR3 (rw) register accessor: an alias for `Reg<FMC_BTR3_SPEC>`"]
pub type FMC_BTR3 = crate::Reg<fmc_btr3::FMC_BTR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 3"]
pub mod fmc_btr3;
#[doc = "FMC_BCR4 (rw) register accessor: an alias for `Reg<FMC_BCR4_SPEC>`"]
pub type FMC_BCR4 = crate::Reg<fmc_bcr4::FMC_BCR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register for bank 4"]
pub mod fmc_bcr4;
#[doc = "FMC_BTR4 (rw) register accessor: an alias for `Reg<FMC_BTR4_SPEC>`"]
pub type FMC_BTR4 = crate::Reg<fmc_btr4::FMC_BTR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 4"]
pub mod fmc_btr4;
#[doc = "FMC_PCSCNTR (rw) register accessor: an alias for `Reg<FMC_PCSCNTR_SPEC>`"]
pub type FMC_PCSCNTR = crate::Reg<fmc_pcscntr::FMC_PCSCNTR_SPEC>;
#[doc = "PSRAM chip select counter register"]
pub mod fmc_pcscntr;
#[doc = "FMC_PCR (rw) register accessor: an alias for `Reg<FMC_PCR_SPEC>`"]
pub type FMC_PCR = crate::Reg<fmc_pcr::FMC_PCR_SPEC>;
#[doc = "NAND Flash control registers"]
pub mod fmc_pcr;
#[doc = "FMC_SR (rw) register accessor: an alias for `Reg<FMC_SR_SPEC>`"]
pub type FMC_SR = crate::Reg<fmc_sr::FMC_SR_SPEC>;
#[doc = "FIFO status and interrupt register"]
pub mod fmc_sr;
#[doc = "FMC_PMEM (rw) register accessor: an alias for `Reg<FMC_PMEM_SPEC>`"]
pub type FMC_PMEM = crate::Reg<fmc_pmem::FMC_PMEM_SPEC>;
#[doc = "Common memory space timing register"]
pub mod fmc_pmem;
#[doc = "FMC_PATT (rw) register accessor: an alias for `Reg<FMC_PATT_SPEC>`"]
pub type FMC_PATT = crate::Reg<fmc_patt::FMC_PATT_SPEC>;
#[doc = "Attribute memory space timing register"]
pub mod fmc_patt;
#[doc = "FMC_ECCR (r) register accessor: an alias for `Reg<FMC_ECCR_SPEC>`"]
pub type FMC_ECCR = crate::Reg<fmc_eccr::FMC_ECCR_SPEC>;
#[doc = "ECC result registers"]
pub mod fmc_eccr;
#[doc = "FMC_BWTR1 (rw) register accessor: an alias for `Reg<FMC_BWTR1_SPEC>`"]
pub type FMC_BWTR1 = crate::Reg<fmc_bwtr1::FMC_BWTR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod fmc_bwtr1;
#[doc = "FMC_BWTR2 (rw) register accessor: an alias for `Reg<FMC_BWTR2_SPEC>`"]
pub type FMC_BWTR2 = crate::Reg<fmc_bwtr2::FMC_BWTR2_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod fmc_bwtr2;
#[doc = "FMC_BWTR3 (rw) register accessor: an alias for `Reg<FMC_BWTR3_SPEC>`"]
pub type FMC_BWTR3 = crate::Reg<fmc_bwtr3::FMC_BWTR3_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod fmc_bwtr3;
#[doc = "FMC_BWTR4 (rw) register accessor: an alias for `Reg<FMC_BWTR4_SPEC>`"]
pub type FMC_BWTR4 = crate::Reg<fmc_bwtr4::FMC_BWTR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod fmc_bwtr4;
#[doc = "FMC_SDCR1 (rw) register accessor: an alias for `Reg<FMC_SDCR1_SPEC>`"]
pub type FMC_SDCR1 = crate::Reg<fmc_sdcr1::FMC_SDCR1_SPEC>;
#[doc = "SDRAM control registers 1"]
pub mod fmc_sdcr1;
#[doc = "FMC_SDCR2 (rw) register accessor: an alias for `Reg<FMC_SDCR2_SPEC>`"]
pub type FMC_SDCR2 = crate::Reg<fmc_sdcr2::FMC_SDCR2_SPEC>;
#[doc = "SDRAM control registers 2"]
pub mod fmc_sdcr2;
#[doc = "FMC_SDTR1 (rw) register accessor: an alias for `Reg<FMC_SDTR1_SPEC>`"]
pub type FMC_SDTR1 = crate::Reg<fmc_sdtr1::FMC_SDTR1_SPEC>;
#[doc = "SDRAM timing registers 1"]
pub mod fmc_sdtr1;
#[doc = "FMC_SDTR2 (rw) register accessor: an alias for `Reg<FMC_SDTR2_SPEC>`"]
pub type FMC_SDTR2 = crate::Reg<fmc_sdtr2::FMC_SDTR2_SPEC>;
#[doc = "SDRAM timing registers 2"]
pub mod fmc_sdtr2;
#[doc = "FMC_SDCMR (rw) register accessor: an alias for `Reg<FMC_SDCMR_SPEC>`"]
pub type FMC_SDCMR = crate::Reg<fmc_sdcmr::FMC_SDCMR_SPEC>;
#[doc = "SDRAM Command Mode register"]
pub mod fmc_sdcmr;
#[doc = "FMC_SDRTR (rw) register accessor: an alias for `Reg<FMC_SDRTR_SPEC>`"]
pub type FMC_SDRTR = crate::Reg<fmc_sdrtr::FMC_SDRTR_SPEC>;
#[doc = "SDRAM refresh timer register"]
pub mod fmc_sdrtr;
#[doc = "FMC_SDSR (r) register accessor: an alias for `Reg<FMC_SDSR_SPEC>`"]
pub type FMC_SDSR = crate::Reg<fmc_sdsr::FMC_SDSR_SPEC>;
#[doc = "SDRAM status register"]
pub mod fmc_sdsr;
