#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TAMP control register 1"]
    pub tamp_cr1: TAMP_CR1,
    #[doc = "0x04 - TAMP control register 2"]
    pub tamp_cr2: TAMP_CR2,
    #[doc = "0x08 - TAMP control register 3"]
    pub tamp_cr3: TAMP_CR3,
    #[doc = "0x0c - TAMP filter control register"]
    pub tamp_fltcr: TAMP_FLTCR,
    #[doc = "0x10 - TAMP active tamper control register 1"]
    pub tamp_atcr1: TAMP_ATCR1,
    #[doc = "0x14 - TAMP active tamper seed register"]
    pub tamp_atseedr: TAMP_ATSEEDR,
    #[doc = "0x18 - TAMP active tamper output register"]
    pub tamp_ator: TAMP_ATOR,
    #[doc = "0x1c - TAMP active tamper control register 2"]
    pub tamp_atcr2: TAMP_ATCR2,
    #[doc = "0x20 - TAMP secure mode register"]
    pub tamp_seccfgr: TAMP_SECCFGR,
    #[doc = "0x24 - TAMP privilege mode control register"]
    pub tamp_privcfgr: TAMP_PRIVCFGR,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub tamp_ier: TAMP_IER,
    #[doc = "0x30 - TAMP status register"]
    pub tamp_sr: TAMP_SR,
    #[doc = "0x34 - TAMP non-secure masked interrupt status register"]
    pub tamp_misr: TAMP_MISR,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub tamp_smisr: TAMP_SMISR,
    #[doc = "0x3c - TAMP status clear register"]
    pub tamp_scr: TAMP_SCR,
    #[doc = "0x40 - TAMP monotonic counter 1 register"]
    pub tamp_count1r: TAMP_COUNT1R,
    _reserved16: [u8; 0x0c],
    #[doc = "0x50 - TAMP option register"]
    pub tamp_or: TAMP_OR,
    #[doc = "0x54 - TAMP erase configuration register"]
    pub tamp_ercfgr: TAMP_ERCFGR,
    _reserved18: [u8; 0xa8],
    #[doc = "0x100 - TAMP backup 0 register"]
    pub tamp_bkp0r: TAMP_BKP0R,
    #[doc = "0x104 - TAMP backup 1 register"]
    pub tamp_bkp1r: TAMP_BKP1R,
    #[doc = "0x108 - TAMP backup 2 register"]
    pub tamp_bkp2r: TAMP_BKP2R,
    #[doc = "0x10c - TAMP backup 3 register"]
    pub tamp_bkp3r: TAMP_BKP3R,
    #[doc = "0x110 - TAMP backup 4 register"]
    pub tamp_bkp4r: TAMP_BKP4R,
    #[doc = "0x114 - TAMP backup 5 register"]
    pub tamp_bkp5r: TAMP_BKP5R,
    #[doc = "0x118 - TAMP backup 6 register"]
    pub tamp_bkp6r: TAMP_BKP6R,
    #[doc = "0x11c - TAMP backup 7 register"]
    pub tamp_bkp7r: TAMP_BKP7R,
    #[doc = "0x120 - TAMP backup 8 register"]
    pub tamp_bkp8r: TAMP_BKP8R,
    #[doc = "0x124 - TAMP backup 9 register"]
    pub tamp_bkp9r: TAMP_BKP9R,
    #[doc = "0x128 - TAMP backup 10 register"]
    pub tamp_bkp10r: TAMP_BKP10R,
    #[doc = "0x12c - TAMP backup 11 register"]
    pub tamp_bkp11r: TAMP_BKP11R,
    #[doc = "0x130 - TAMP backup 12 register"]
    pub tamp_bkp12r: TAMP_BKP12R,
    #[doc = "0x134 - TAMP backup 13 register"]
    pub tamp_bkp13r: TAMP_BKP13R,
    #[doc = "0x138 - TAMP backup 14 register"]
    pub tamp_bkp14r: TAMP_BKP14R,
    #[doc = "0x13c - TAMP backup 15 register"]
    pub tamp_bkp15r: TAMP_BKP15R,
    #[doc = "0x140 - TAMP backup 16 register"]
    pub tamp_bkp16r: TAMP_BKP16R,
    #[doc = "0x144 - TAMP backup 17 register"]
    pub tamp_bkp17r: TAMP_BKP17R,
    #[doc = "0x148 - TAMP backup 18 register"]
    pub tamp_bkp18r: TAMP_BKP18R,
    #[doc = "0x14c - TAMP backup 19 register"]
    pub tamp_bkp19r: TAMP_BKP19R,
    #[doc = "0x150 - TAMP backup 20 register"]
    pub tamp_bkp20r: TAMP_BKP20R,
    #[doc = "0x154 - TAMP backup 21 register"]
    pub tamp_bkp21r: TAMP_BKP21R,
    #[doc = "0x158 - TAMP backup 22 register"]
    pub tamp_bkp22r: TAMP_BKP22R,
    #[doc = "0x15c - TAMP backup 23 register"]
    pub tamp_bkp23r: TAMP_BKP23R,
    #[doc = "0x160 - TAMP backup 24 register"]
    pub tamp_bkp24r: TAMP_BKP24R,
    #[doc = "0x164 - TAMP backup 25 register"]
    pub tamp_bkp25r: TAMP_BKP25R,
    #[doc = "0x168 - TAMP backup 26 register"]
    pub tamp_bkp26r: TAMP_BKP26R,
    #[doc = "0x16c - TAMP backup 27 register"]
    pub tamp_bkp27r: TAMP_BKP27R,
    #[doc = "0x170 - TAMP backup 28 register"]
    pub tamp_bkp28r: TAMP_BKP28R,
    #[doc = "0x174 - TAMP backup 29 register"]
    pub tamp_bkp29r: TAMP_BKP29R,
    #[doc = "0x178 - TAMP backup 30 register"]
    pub tamp_bkp30r: TAMP_BKP30R,
    #[doc = "0x17c - TAMP backup 31 register"]
    pub tamp_bkp31r: TAMP_BKP31R,
}
#[doc = "TAMP_CR1 (rw) register accessor: an alias for `Reg<TAMP_CR1_SPEC>`"]
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1_SPEC>;
#[doc = "TAMP control register 1"]
pub mod tamp_cr1;
#[doc = "TAMP_CR2 (rw) register accessor: an alias for `Reg<TAMP_CR2_SPEC>`"]
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2_SPEC>;
#[doc = "TAMP control register 2"]
pub mod tamp_cr2;
#[doc = "TAMP_CR3 (rw) register accessor: an alias for `Reg<TAMP_CR3_SPEC>`"]
pub type TAMP_CR3 = crate::Reg<tamp_cr3::TAMP_CR3_SPEC>;
#[doc = "TAMP control register 3"]
pub mod tamp_cr3;
#[doc = "TAMP_FLTCR (rw) register accessor: an alias for `Reg<TAMP_FLTCR_SPEC>`"]
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod tamp_fltcr;
#[doc = "TAMP_ATCR1 (rw) register accessor: an alias for `Reg<TAMP_ATCR1_SPEC>`"]
pub type TAMP_ATCR1 = crate::Reg<tamp_atcr1::TAMP_ATCR1_SPEC>;
#[doc = "TAMP active tamper control register 1"]
pub mod tamp_atcr1;
#[doc = "TAMP_ATSEEDR (w) register accessor: an alias for `Reg<TAMP_ATSEEDR_SPEC>`"]
pub type TAMP_ATSEEDR = crate::Reg<tamp_atseedr::TAMP_ATSEEDR_SPEC>;
#[doc = "TAMP active tamper seed register"]
pub mod tamp_atseedr;
#[doc = "TAMP_ATOR (r) register accessor: an alias for `Reg<TAMP_ATOR_SPEC>`"]
pub type TAMP_ATOR = crate::Reg<tamp_ator::TAMP_ATOR_SPEC>;
#[doc = "TAMP active tamper output register"]
pub mod tamp_ator;
#[doc = "TAMP_ATCR2 (rw) register accessor: an alias for `Reg<TAMP_ATCR2_SPEC>`"]
pub type TAMP_ATCR2 = crate::Reg<tamp_atcr2::TAMP_ATCR2_SPEC>;
#[doc = "TAMP active tamper control register 2"]
pub mod tamp_atcr2;
#[doc = "TAMP_SECCFGR (rw) register accessor: an alias for `Reg<TAMP_SECCFGR_SPEC>`"]
pub type TAMP_SECCFGR = crate::Reg<tamp_seccfgr::TAMP_SECCFGR_SPEC>;
#[doc = "TAMP secure mode register"]
pub mod tamp_seccfgr;
#[doc = "TAMP_PRIVCFGR (rw) register accessor: an alias for `Reg<TAMP_PRIVCFGR_SPEC>`"]
pub type TAMP_PRIVCFGR = crate::Reg<tamp_privcfgr::TAMP_PRIVCFGR_SPEC>;
#[doc = "TAMP privilege mode control register"]
pub mod tamp_privcfgr;
#[doc = "TAMP_IER (rw) register accessor: an alias for `Reg<TAMP_IER_SPEC>`"]
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod tamp_ier;
#[doc = "TAMP_SR (rw) register accessor: an alias for `Reg<TAMP_SR_SPEC>`"]
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SR_SPEC>;
#[doc = "TAMP status register"]
pub mod tamp_sr;
#[doc = "TAMP_MISR (r) register accessor: an alias for `Reg<TAMP_MISR_SPEC>`"]
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISR_SPEC>;
#[doc = "TAMP non-secure masked interrupt status register"]
pub mod tamp_misr;
#[doc = "TAMP_SMISR (r) register accessor: an alias for `Reg<TAMP_SMISR_SPEC>`"]
pub type TAMP_SMISR = crate::Reg<tamp_smisr::TAMP_SMISR_SPEC>;
#[doc = "TAMP secure masked interrupt status register"]
pub mod tamp_smisr;
#[doc = "TAMP_SCR (w) register accessor: an alias for `Reg<TAMP_SCR_SPEC>`"]
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod tamp_scr;
#[doc = "TAMP_COUNT1R (r) register accessor: an alias for `Reg<TAMP_COUNT1R_SPEC>`"]
pub type TAMP_COUNT1R = crate::Reg<tamp_count1r::TAMP_COUNT1R_SPEC>;
#[doc = "TAMP monotonic counter 1 register"]
pub mod tamp_count1r;
#[doc = "TAMP_OR (rw) register accessor: an alias for `Reg<TAMP_OR_SPEC>`"]
pub type TAMP_OR = crate::Reg<tamp_or::TAMP_OR_SPEC>;
#[doc = "TAMP option register"]
pub mod tamp_or;
#[doc = "TAMP_ERCFGR (rw) register accessor: an alias for `Reg<TAMP_ERCFGR_SPEC>`"]
pub type TAMP_ERCFGR = crate::Reg<tamp_ercfgr::TAMP_ERCFGR_SPEC>;
#[doc = "TAMP erase configuration register"]
pub mod tamp_ercfgr;
#[doc = "TAMP_BKP0R (rw) register accessor: an alias for `Reg<TAMP_BKP0R_SPEC>`"]
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0R_SPEC>;
#[doc = "TAMP backup 0 register"]
pub mod tamp_bkp0r;
#[doc = "TAMP_BKP1R (rw) register accessor: an alias for `Reg<TAMP_BKP1R_SPEC>`"]
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1R_SPEC>;
#[doc = "TAMP backup 1 register"]
pub mod tamp_bkp1r;
#[doc = "TAMP_BKP2R (rw) register accessor: an alias for `Reg<TAMP_BKP2R_SPEC>`"]
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2R_SPEC>;
#[doc = "TAMP backup 2 register"]
pub mod tamp_bkp2r;
#[doc = "TAMP_BKP3R (rw) register accessor: an alias for `Reg<TAMP_BKP3R_SPEC>`"]
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3R_SPEC>;
#[doc = "TAMP backup 3 register"]
pub mod tamp_bkp3r;
#[doc = "TAMP_BKP4R (rw) register accessor: an alias for `Reg<TAMP_BKP4R_SPEC>`"]
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4R_SPEC>;
#[doc = "TAMP backup 4 register"]
pub mod tamp_bkp4r;
#[doc = "TAMP_BKP5R (rw) register accessor: an alias for `Reg<TAMP_BKP5R_SPEC>`"]
pub type TAMP_BKP5R = crate::Reg<tamp_bkp5r::TAMP_BKP5R_SPEC>;
#[doc = "TAMP backup 5 register"]
pub mod tamp_bkp5r;
#[doc = "TAMP_BKP6R (rw) register accessor: an alias for `Reg<TAMP_BKP6R_SPEC>`"]
pub type TAMP_BKP6R = crate::Reg<tamp_bkp6r::TAMP_BKP6R_SPEC>;
#[doc = "TAMP backup 6 register"]
pub mod tamp_bkp6r;
#[doc = "TAMP_BKP7R (rw) register accessor: an alias for `Reg<TAMP_BKP7R_SPEC>`"]
pub type TAMP_BKP7R = crate::Reg<tamp_bkp7r::TAMP_BKP7R_SPEC>;
#[doc = "TAMP backup 7 register"]
pub mod tamp_bkp7r;
#[doc = "TAMP_BKP8R (rw) register accessor: an alias for `Reg<TAMP_BKP8R_SPEC>`"]
pub type TAMP_BKP8R = crate::Reg<tamp_bkp8r::TAMP_BKP8R_SPEC>;
#[doc = "TAMP backup 8 register"]
pub mod tamp_bkp8r;
#[doc = "TAMP_BKP9R (rw) register accessor: an alias for `Reg<TAMP_BKP9R_SPEC>`"]
pub type TAMP_BKP9R = crate::Reg<tamp_bkp9r::TAMP_BKP9R_SPEC>;
#[doc = "TAMP backup 9 register"]
pub mod tamp_bkp9r;
#[doc = "TAMP_BKP10R (rw) register accessor: an alias for `Reg<TAMP_BKP10R_SPEC>`"]
pub type TAMP_BKP10R = crate::Reg<tamp_bkp10r::TAMP_BKP10R_SPEC>;
#[doc = "TAMP backup 10 register"]
pub mod tamp_bkp10r;
#[doc = "TAMP_BKP11R (rw) register accessor: an alias for `Reg<TAMP_BKP11R_SPEC>`"]
pub type TAMP_BKP11R = crate::Reg<tamp_bkp11r::TAMP_BKP11R_SPEC>;
#[doc = "TAMP backup 11 register"]
pub mod tamp_bkp11r;
#[doc = "TAMP_BKP12R (rw) register accessor: an alias for `Reg<TAMP_BKP12R_SPEC>`"]
pub type TAMP_BKP12R = crate::Reg<tamp_bkp12r::TAMP_BKP12R_SPEC>;
#[doc = "TAMP backup 12 register"]
pub mod tamp_bkp12r;
#[doc = "TAMP_BKP13R (rw) register accessor: an alias for `Reg<TAMP_BKP13R_SPEC>`"]
pub type TAMP_BKP13R = crate::Reg<tamp_bkp13r::TAMP_BKP13R_SPEC>;
#[doc = "TAMP backup 13 register"]
pub mod tamp_bkp13r;
#[doc = "TAMP_BKP14R (rw) register accessor: an alias for `Reg<TAMP_BKP14R_SPEC>`"]
pub type TAMP_BKP14R = crate::Reg<tamp_bkp14r::TAMP_BKP14R_SPEC>;
#[doc = "TAMP backup 14 register"]
pub mod tamp_bkp14r;
#[doc = "TAMP_BKP15R (rw) register accessor: an alias for `Reg<TAMP_BKP15R_SPEC>`"]
pub type TAMP_BKP15R = crate::Reg<tamp_bkp15r::TAMP_BKP15R_SPEC>;
#[doc = "TAMP backup 15 register"]
pub mod tamp_bkp15r;
#[doc = "TAMP_BKP16R (rw) register accessor: an alias for `Reg<TAMP_BKP16R_SPEC>`"]
pub type TAMP_BKP16R = crate::Reg<tamp_bkp16r::TAMP_BKP16R_SPEC>;
#[doc = "TAMP backup 16 register"]
pub mod tamp_bkp16r;
#[doc = "TAMP_BKP17R (rw) register accessor: an alias for `Reg<TAMP_BKP17R_SPEC>`"]
pub type TAMP_BKP17R = crate::Reg<tamp_bkp17r::TAMP_BKP17R_SPEC>;
#[doc = "TAMP backup 17 register"]
pub mod tamp_bkp17r;
#[doc = "TAMP_BKP18R (rw) register accessor: an alias for `Reg<TAMP_BKP18R_SPEC>`"]
pub type TAMP_BKP18R = crate::Reg<tamp_bkp18r::TAMP_BKP18R_SPEC>;
#[doc = "TAMP backup 18 register"]
pub mod tamp_bkp18r;
#[doc = "TAMP_BKP19R (rw) register accessor: an alias for `Reg<TAMP_BKP19R_SPEC>`"]
pub type TAMP_BKP19R = crate::Reg<tamp_bkp19r::TAMP_BKP19R_SPEC>;
#[doc = "TAMP backup 19 register"]
pub mod tamp_bkp19r;
#[doc = "TAMP_BKP20R (rw) register accessor: an alias for `Reg<TAMP_BKP20R_SPEC>`"]
pub type TAMP_BKP20R = crate::Reg<tamp_bkp20r::TAMP_BKP20R_SPEC>;
#[doc = "TAMP backup 20 register"]
pub mod tamp_bkp20r;
#[doc = "TAMP_BKP21R (rw) register accessor: an alias for `Reg<TAMP_BKP21R_SPEC>`"]
pub type TAMP_BKP21R = crate::Reg<tamp_bkp21r::TAMP_BKP21R_SPEC>;
#[doc = "TAMP backup 21 register"]
pub mod tamp_bkp21r;
#[doc = "TAMP_BKP22R (rw) register accessor: an alias for `Reg<TAMP_BKP22R_SPEC>`"]
pub type TAMP_BKP22R = crate::Reg<tamp_bkp22r::TAMP_BKP22R_SPEC>;
#[doc = "TAMP backup 22 register"]
pub mod tamp_bkp22r;
#[doc = "TAMP_BKP23R (rw) register accessor: an alias for `Reg<TAMP_BKP23R_SPEC>`"]
pub type TAMP_BKP23R = crate::Reg<tamp_bkp23r::TAMP_BKP23R_SPEC>;
#[doc = "TAMP backup 23 register"]
pub mod tamp_bkp23r;
#[doc = "TAMP_BKP24R (rw) register accessor: an alias for `Reg<TAMP_BKP24R_SPEC>`"]
pub type TAMP_BKP24R = crate::Reg<tamp_bkp24r::TAMP_BKP24R_SPEC>;
#[doc = "TAMP backup 24 register"]
pub mod tamp_bkp24r;
#[doc = "TAMP_BKP25R (rw) register accessor: an alias for `Reg<TAMP_BKP25R_SPEC>`"]
pub type TAMP_BKP25R = crate::Reg<tamp_bkp25r::TAMP_BKP25R_SPEC>;
#[doc = "TAMP backup 25 register"]
pub mod tamp_bkp25r;
#[doc = "TAMP_BKP26R (rw) register accessor: an alias for `Reg<TAMP_BKP26R_SPEC>`"]
pub type TAMP_BKP26R = crate::Reg<tamp_bkp26r::TAMP_BKP26R_SPEC>;
#[doc = "TAMP backup 26 register"]
pub mod tamp_bkp26r;
#[doc = "TAMP_BKP27R (rw) register accessor: an alias for `Reg<TAMP_BKP27R_SPEC>`"]
pub type TAMP_BKP27R = crate::Reg<tamp_bkp27r::TAMP_BKP27R_SPEC>;
#[doc = "TAMP backup 27 register"]
pub mod tamp_bkp27r;
#[doc = "TAMP_BKP28R (rw) register accessor: an alias for `Reg<TAMP_BKP28R_SPEC>`"]
pub type TAMP_BKP28R = crate::Reg<tamp_bkp28r::TAMP_BKP28R_SPEC>;
#[doc = "TAMP backup 28 register"]
pub mod tamp_bkp28r;
#[doc = "TAMP_BKP29R (rw) register accessor: an alias for `Reg<TAMP_BKP29R_SPEC>`"]
pub type TAMP_BKP29R = crate::Reg<tamp_bkp29r::TAMP_BKP29R_SPEC>;
#[doc = "TAMP backup 29 register"]
pub mod tamp_bkp29r;
#[doc = "TAMP_BKP30R (rw) register accessor: an alias for `Reg<TAMP_BKP30R_SPEC>`"]
pub type TAMP_BKP30R = crate::Reg<tamp_bkp30r::TAMP_BKP30R_SPEC>;
#[doc = "TAMP backup 30 register"]
pub mod tamp_bkp30r;
#[doc = "TAMP_BKP31R (rw) register accessor: an alias for `Reg<TAMP_BKP31R_SPEC>`"]
pub type TAMP_BKP31R = crate::Reg<tamp_bkp31r::TAMP_BKP31R_SPEC>;
#[doc = "TAMP backup 31 register"]
pub mod tamp_bkp31r;
