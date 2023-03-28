#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    pub gtzc1_tzsc_privcfgr1: GTZC1_TZSC_PRIVCFGR1,
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    pub gtzc1_tzsc_privcfgr2: GTZC1_TZSC_PRIVCFGR2,
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    pub gtzc1_tzsc_privcfgr3: GTZC1_TZSC_PRIVCFGR3,
    _reserved3: [u8; 0x44],
    #[doc = "0x70 - GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
    pub gtzc1_tzsc_mpcwm4acfgr: GTZC1_TZSC_MPCWM4ACFGR,
    #[doc = "0x74 - GTZC1 TZSC BKPSRAM sub-region A watermark register"]
    pub gtzc1_tzsc_mpcwm4ar: GTZC1_TZSC_MPCWM4AR,
    #[doc = "0x78 - GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
    pub gtzc1_tzsc_mpcwm4bcfgr: GTZC1_TZSC_MPCWM4BCFGR,
    #[doc = "0x7c - GTZC1 TZSC BKPSRAM sub-region B watermark register"]
    pub gtzc1_tzsc_mpcwm4br: GTZC1_TZSC_MPCWM4BR,
    _reserved7: [u8; 0x0180],
    #[doc = "0x200 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
    pub gtzc1_mpcbb1_privcfgr0: GTZC1_MPCBB1_PRIVCFGR0,
    #[doc = "0x204 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
    pub gtzc1_mpcbb1_privcfgr1: GTZC1_MPCBB1_PRIVCFGR1,
    #[doc = "0x208 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
    pub gtzc1_mpcbb1_privcfgr2: GTZC1_MPCBB1_PRIVCFGR2,
    #[doc = "0x20c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
    pub gtzc1_mpcbb1_privcfgr3: GTZC1_MPCBB1_PRIVCFGR3,
    #[doc = "0x210 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
    pub gtzc1_mpcbb1_privcfgr4: GTZC1_MPCBB1_PRIVCFGR4,
    #[doc = "0x214 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
    pub gtzc1_mpcbb1_privcfgr5: GTZC1_MPCBB1_PRIVCFGR5,
    #[doc = "0x218 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
    pub gtzc1_mpcbb1_privcfgr6: GTZC1_MPCBB1_PRIVCFGR6,
    #[doc = "0x21c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
    pub gtzc1_mpcbb1_privcfgr7: GTZC1_MPCBB1_PRIVCFGR7,
    #[doc = "0x220 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
    pub gtzc1_mpcbb1_privcfgr8: GTZC1_MPCBB1_PRIVCFGR8,
    #[doc = "0x224 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
    pub gtzc1_mpcbb1_privcfgr9: GTZC1_MPCBB1_PRIVCFGR9,
    #[doc = "0x228 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
    pub gtzc1_mpcbb1_privcfgr10: GTZC1_MPCBB1_PRIVCFGR10,
    #[doc = "0x22c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
    pub gtzc1_mpcbb1_privcfgr11: GTZC1_MPCBB1_PRIVCFGR11,
    #[doc = "0x230 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
    pub gtzc1_mpcbb1_privcfgr12: GTZC1_MPCBB1_PRIVCFGR12,
    #[doc = "0x234 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
    pub gtzc1_mpcbb1_privcfgr13: GTZC1_MPCBB1_PRIVCFGR13,
    #[doc = "0x238 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
    pub gtzc1_mpcbb1_privcfgr14: GTZC1_MPCBB1_PRIVCFGR14,
    #[doc = "0x23c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
    pub gtzc1_mpcbb1_privcfgr15: GTZC1_MPCBB1_PRIVCFGR15,
    #[doc = "0x240 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
    pub gtzc1_mpcbb1_privcfgr16: GTZC1_MPCBB1_PRIVCFGR16,
    #[doc = "0x244 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
    pub gtzc1_mpcbb1_privcfgr17: GTZC1_MPCBB1_PRIVCFGR17,
    #[doc = "0x248 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
    pub gtzc1_mpcbb1_privcfgr18: GTZC1_MPCBB1_PRIVCFGR18,
    #[doc = "0x24c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
    pub gtzc1_mpcbb1_privcfgr19: GTZC1_MPCBB1_PRIVCFGR19,
    #[doc = "0x250 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
    pub gtzc1_mpcbb1_privcfgr20: GTZC1_MPCBB1_PRIVCFGR20,
    #[doc = "0x254 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
    pub gtzc1_mpcbb1_privcfgr21: GTZC1_MPCBB1_PRIVCFGR21,
    #[doc = "0x258 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
    pub gtzc1_mpcbb1_privcfgr22: GTZC1_MPCBB1_PRIVCFGR22,
    #[doc = "0x25c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
    pub gtzc1_mpcbb1_privcfgr23: GTZC1_MPCBB1_PRIVCFGR23,
    #[doc = "0x260 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
    pub gtzc1_mpcbb1_privcfgr24: GTZC1_MPCBB1_PRIVCFGR24,
    #[doc = "0x264 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
    pub gtzc1_mpcbb1_privcfgr25: GTZC1_MPCBB1_PRIVCFGR25,
    #[doc = "0x268 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
    pub gtzc1_mpcbb1_privcfgr26: GTZC1_MPCBB1_PRIVCFGR26,
    #[doc = "0x26c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
    pub gtzc1_mpcbb1_privcfgr27: GTZC1_MPCBB1_PRIVCFGR27,
    #[doc = "0x270 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
    pub gtzc1_mpcbb1_privcfgr28: GTZC1_MPCBB1_PRIVCFGR28,
    #[doc = "0x274 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
    pub gtzc1_mpcbb1_privcfgr29: GTZC1_MPCBB1_PRIVCFGR29,
    #[doc = "0x278 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
    pub gtzc1_mpcbb1_privcfgr30: GTZC1_MPCBB1_PRIVCFGR30,
    #[doc = "0x27c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
    pub gtzc1_mpcbb1_privcfgr31: GTZC1_MPCBB1_PRIVCFGR31,
    _reserved39: [u8; 0x0380],
    #[doc = "0x600 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
    pub gtzc1_mpcbb2_privcfgr0: GTZC1_MPCBB2_PRIVCFGR0,
    #[doc = "0x604 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
    pub gtzc1_mpcbb2_privcfgr1: GTZC1_MPCBB2_PRIVCFGR1,
    #[doc = "0x608 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
    pub gtzc1_mpcbb2_privcfgr2: GTZC1_MPCBB2_PRIVCFGR2,
    #[doc = "0x60c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
    pub gtzc1_mpcbb2_privcfgr3: GTZC1_MPCBB2_PRIVCFGR3,
    #[doc = "0x610 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
    pub gtzc1_mpcbb2_privcfgr4: GTZC1_MPCBB2_PRIVCFGR4,
    #[doc = "0x614 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
    pub gtzc1_mpcbb2_privcfgr5: GTZC1_MPCBB2_PRIVCFGR5,
    #[doc = "0x618 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
    pub gtzc1_mpcbb2_privcfgr6: GTZC1_MPCBB2_PRIVCFGR6,
    #[doc = "0x61c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
    pub gtzc1_mpcbb2_privcfgr7: GTZC1_MPCBB2_PRIVCFGR7,
    #[doc = "0x620 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
    pub gtzc1_mpcbb2_privcfgr8: GTZC1_MPCBB2_PRIVCFGR8,
    #[doc = "0x624 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
    pub gtzc1_mpcbb2_privcfgr9: GTZC1_MPCBB2_PRIVCFGR9,
    #[doc = "0x628 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
    pub gtzc1_mpcbb2_privcfgr10: GTZC1_MPCBB2_PRIVCFGR10,
    #[doc = "0x62c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
    pub gtzc1_mpcbb2_privcfgr11: GTZC1_MPCBB2_PRIVCFGR11,
    #[doc = "0x630 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
    pub gtzc1_mpcbb2_privcfgr12: GTZC1_MPCBB2_PRIVCFGR12,
    #[doc = "0x634 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
    pub gtzc1_mpcbb2_privcfgr13: GTZC1_MPCBB2_PRIVCFGR13,
    #[doc = "0x638 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
    pub gtzc1_mpcbb2_privcfgr14: GTZC1_MPCBB2_PRIVCFGR14,
    #[doc = "0x63c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
    pub gtzc1_mpcbb2_privcfgr15: GTZC1_MPCBB2_PRIVCFGR15,
    #[doc = "0x640 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
    pub gtzc1_mpcbb2_privcfgr16: GTZC1_MPCBB2_PRIVCFGR16,
    #[doc = "0x644 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
    pub gtzc1_mpcbb2_privcfgr17: GTZC1_MPCBB2_PRIVCFGR17,
    #[doc = "0x648 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
    pub gtzc1_mpcbb2_privcfgr18: GTZC1_MPCBB2_PRIVCFGR18,
    #[doc = "0x64c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
    pub gtzc1_mpcbb2_privcfgr19: GTZC1_MPCBB2_PRIVCFGR19,
    #[doc = "0x650 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
    pub gtzc1_mpcbb2_privcfgr20: GTZC1_MPCBB2_PRIVCFGR20,
    #[doc = "0x654 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
    pub gtzc1_mpcbb2_privcfgr21: GTZC1_MPCBB2_PRIVCFGR21,
    #[doc = "0x658 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
    pub gtzc1_mpcbb2_privcfgr22: GTZC1_MPCBB2_PRIVCFGR22,
    #[doc = "0x65c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
    pub gtzc1_mpcbb2_privcfgr23: GTZC1_MPCBB2_PRIVCFGR23,
    #[doc = "0x660 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
    pub gtzc1_mpcbb2_privcfgr24: GTZC1_MPCBB2_PRIVCFGR24,
    #[doc = "0x664 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
    pub gtzc1_mpcbb2_privcfgr25: GTZC1_MPCBB2_PRIVCFGR25,
    #[doc = "0x668 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
    pub gtzc1_mpcbb2_privcfgr26: GTZC1_MPCBB2_PRIVCFGR26,
    #[doc = "0x66c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
    pub gtzc1_mpcbb2_privcfgr27: GTZC1_MPCBB2_PRIVCFGR27,
    #[doc = "0x670 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
    pub gtzc1_mpcbb2_privcfgr28: GTZC1_MPCBB2_PRIVCFGR28,
    #[doc = "0x674 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
    pub gtzc1_mpcbb2_privcfgr29: GTZC1_MPCBB2_PRIVCFGR29,
    #[doc = "0x678 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
    pub gtzc1_mpcbb2_privcfgr30: GTZC1_MPCBB2_PRIVCFGR30,
    #[doc = "0x67c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
    pub gtzc1_mpcbb2_privcfgr31: GTZC1_MPCBB2_PRIVCFGR31,
}
#[doc = "GTZC1_TZSC_PRIVCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR1_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR1 = crate::Reg<gtzc1_tzsc_privcfgr1::GTZC1_TZSC_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod gtzc1_tzsc_privcfgr1;
#[doc = "GTZC1_TZSC_PRIVCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR2_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR2 = crate::Reg<gtzc1_tzsc_privcfgr2::GTZC1_TZSC_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod gtzc1_tzsc_privcfgr2;
#[doc = "GTZC1_TZSC_PRIVCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_TZSC_PRIVCFGR3_SPEC>`"]
pub type GTZC1_TZSC_PRIVCFGR3 = crate::Reg<gtzc1_tzsc_privcfgr3::GTZC1_TZSC_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod gtzc1_tzsc_privcfgr3;
#[doc = "GTZC1_TZSC_MPCWM4ACFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4ACFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4ACFGR = crate::Reg<gtzc1_tzsc_mpcwm4acfgr::GTZC1_TZSC_MPCWM4ACFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4acfgr;
#[doc = "GTZC1_TZSC_MPCWM4AR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4AR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4AR = crate::Reg<gtzc1_tzsc_mpcwm4ar::GTZC1_TZSC_MPCWM4AR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm4ar;
#[doc = "GTZC1_TZSC_MPCWM4BCFGR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4BCFGR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4BCFGR = crate::Reg<gtzc1_tzsc_mpcwm4bcfgr::GTZC1_TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4bcfgr;
#[doc = "GTZC1_TZSC_MPCWM4BR (rw) register accessor: an alias for `Reg<GTZC1_TZSC_MPCWM4BR_SPEC>`"]
pub type GTZC1_TZSC_MPCWM4BR = crate::Reg<gtzc1_tzsc_mpcwm4br::GTZC1_TZSC_MPCWM4BR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm4br;
#[doc = "GTZC1_MPCBB1_PRIVCFGR0 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR0_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb1_privcfgr0::GTZC1_MPCBB1_PRIVCFGR0_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
pub mod gtzc1_mpcbb1_privcfgr0;
#[doc = "GTZC1_MPCBB1_PRIVCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR1_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb1_privcfgr1::GTZC1_MPCBB1_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
pub mod gtzc1_mpcbb1_privcfgr1;
#[doc = "GTZC1_MPCBB1_PRIVCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR2_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb1_privcfgr2::GTZC1_MPCBB1_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
pub mod gtzc1_mpcbb1_privcfgr2;
#[doc = "GTZC1_MPCBB1_PRIVCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR3_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb1_privcfgr3::GTZC1_MPCBB1_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
pub mod gtzc1_mpcbb1_privcfgr3;
#[doc = "GTZC1_MPCBB1_PRIVCFGR4 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR4_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb1_privcfgr4::GTZC1_MPCBB1_PRIVCFGR4_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
pub mod gtzc1_mpcbb1_privcfgr4;
#[doc = "GTZC1_MPCBB1_PRIVCFGR5 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR5_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb1_privcfgr5::GTZC1_MPCBB1_PRIVCFGR5_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
pub mod gtzc1_mpcbb1_privcfgr5;
#[doc = "GTZC1_MPCBB1_PRIVCFGR6 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR6_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb1_privcfgr6::GTZC1_MPCBB1_PRIVCFGR6_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
pub mod gtzc1_mpcbb1_privcfgr6;
#[doc = "GTZC1_MPCBB1_PRIVCFGR7 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR7_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb1_privcfgr7::GTZC1_MPCBB1_PRIVCFGR7_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
pub mod gtzc1_mpcbb1_privcfgr7;
#[doc = "GTZC1_MPCBB1_PRIVCFGR8 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR8_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb1_privcfgr8::GTZC1_MPCBB1_PRIVCFGR8_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
pub mod gtzc1_mpcbb1_privcfgr8;
#[doc = "GTZC1_MPCBB1_PRIVCFGR9 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR9_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb1_privcfgr9::GTZC1_MPCBB1_PRIVCFGR9_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
pub mod gtzc1_mpcbb1_privcfgr9;
#[doc = "GTZC1_MPCBB1_PRIVCFGR10 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR10_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR10 =
    crate::Reg<gtzc1_mpcbb1_privcfgr10::GTZC1_MPCBB1_PRIVCFGR10_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
pub mod gtzc1_mpcbb1_privcfgr10;
#[doc = "GTZC1_MPCBB1_PRIVCFGR11 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR11_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR11 =
    crate::Reg<gtzc1_mpcbb1_privcfgr11::GTZC1_MPCBB1_PRIVCFGR11_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
pub mod gtzc1_mpcbb1_privcfgr11;
#[doc = "GTZC1_MPCBB1_PRIVCFGR12 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR12_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR12 =
    crate::Reg<gtzc1_mpcbb1_privcfgr12::GTZC1_MPCBB1_PRIVCFGR12_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
pub mod gtzc1_mpcbb1_privcfgr12;
#[doc = "GTZC1_MPCBB1_PRIVCFGR13 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR13_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR13 =
    crate::Reg<gtzc1_mpcbb1_privcfgr13::GTZC1_MPCBB1_PRIVCFGR13_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
pub mod gtzc1_mpcbb1_privcfgr13;
#[doc = "GTZC1_MPCBB1_PRIVCFGR14 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR14_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR14 =
    crate::Reg<gtzc1_mpcbb1_privcfgr14::GTZC1_MPCBB1_PRIVCFGR14_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
pub mod gtzc1_mpcbb1_privcfgr14;
#[doc = "GTZC1_MPCBB1_PRIVCFGR15 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR15_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR15 =
    crate::Reg<gtzc1_mpcbb1_privcfgr15::GTZC1_MPCBB1_PRIVCFGR15_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
pub mod gtzc1_mpcbb1_privcfgr15;
#[doc = "GTZC1_MPCBB1_PRIVCFGR16 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR16_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR16 =
    crate::Reg<gtzc1_mpcbb1_privcfgr16::GTZC1_MPCBB1_PRIVCFGR16_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
pub mod gtzc1_mpcbb1_privcfgr16;
#[doc = "GTZC1_MPCBB1_PRIVCFGR17 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR17_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR17 =
    crate::Reg<gtzc1_mpcbb1_privcfgr17::GTZC1_MPCBB1_PRIVCFGR17_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
pub mod gtzc1_mpcbb1_privcfgr17;
#[doc = "GTZC1_MPCBB1_PRIVCFGR18 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR18_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR18 =
    crate::Reg<gtzc1_mpcbb1_privcfgr18::GTZC1_MPCBB1_PRIVCFGR18_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
pub mod gtzc1_mpcbb1_privcfgr18;
#[doc = "GTZC1_MPCBB1_PRIVCFGR19 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR19_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR19 =
    crate::Reg<gtzc1_mpcbb1_privcfgr19::GTZC1_MPCBB1_PRIVCFGR19_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
pub mod gtzc1_mpcbb1_privcfgr19;
#[doc = "GTZC1_MPCBB1_PRIVCFGR20 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR20_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR20 =
    crate::Reg<gtzc1_mpcbb1_privcfgr20::GTZC1_MPCBB1_PRIVCFGR20_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
pub mod gtzc1_mpcbb1_privcfgr20;
#[doc = "GTZC1_MPCBB1_PRIVCFGR21 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR21_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR21 =
    crate::Reg<gtzc1_mpcbb1_privcfgr21::GTZC1_MPCBB1_PRIVCFGR21_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
pub mod gtzc1_mpcbb1_privcfgr21;
#[doc = "GTZC1_MPCBB1_PRIVCFGR22 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR22_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR22 =
    crate::Reg<gtzc1_mpcbb1_privcfgr22::GTZC1_MPCBB1_PRIVCFGR22_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
pub mod gtzc1_mpcbb1_privcfgr22;
#[doc = "GTZC1_MPCBB1_PRIVCFGR23 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR23_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR23 =
    crate::Reg<gtzc1_mpcbb1_privcfgr23::GTZC1_MPCBB1_PRIVCFGR23_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
pub mod gtzc1_mpcbb1_privcfgr23;
#[doc = "GTZC1_MPCBB1_PRIVCFGR24 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR24_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR24 =
    crate::Reg<gtzc1_mpcbb1_privcfgr24::GTZC1_MPCBB1_PRIVCFGR24_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
pub mod gtzc1_mpcbb1_privcfgr24;
#[doc = "GTZC1_MPCBB1_PRIVCFGR25 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR25_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR25 =
    crate::Reg<gtzc1_mpcbb1_privcfgr25::GTZC1_MPCBB1_PRIVCFGR25_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
pub mod gtzc1_mpcbb1_privcfgr25;
#[doc = "GTZC1_MPCBB1_PRIVCFGR26 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR26_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR26 =
    crate::Reg<gtzc1_mpcbb1_privcfgr26::GTZC1_MPCBB1_PRIVCFGR26_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
pub mod gtzc1_mpcbb1_privcfgr26;
#[doc = "GTZC1_MPCBB1_PRIVCFGR27 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR27_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR27 =
    crate::Reg<gtzc1_mpcbb1_privcfgr27::GTZC1_MPCBB1_PRIVCFGR27_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
pub mod gtzc1_mpcbb1_privcfgr27;
#[doc = "GTZC1_MPCBB1_PRIVCFGR28 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR28_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR28 =
    crate::Reg<gtzc1_mpcbb1_privcfgr28::GTZC1_MPCBB1_PRIVCFGR28_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
pub mod gtzc1_mpcbb1_privcfgr28;
#[doc = "GTZC1_MPCBB1_PRIVCFGR29 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR29_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR29 =
    crate::Reg<gtzc1_mpcbb1_privcfgr29::GTZC1_MPCBB1_PRIVCFGR29_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
pub mod gtzc1_mpcbb1_privcfgr29;
#[doc = "GTZC1_MPCBB1_PRIVCFGR30 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR30_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR30 =
    crate::Reg<gtzc1_mpcbb1_privcfgr30::GTZC1_MPCBB1_PRIVCFGR30_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
pub mod gtzc1_mpcbb1_privcfgr30;
#[doc = "GTZC1_MPCBB1_PRIVCFGR31 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_PRIVCFGR31_SPEC>`"]
pub type GTZC1_MPCBB1_PRIVCFGR31 =
    crate::Reg<gtzc1_mpcbb1_privcfgr31::GTZC1_MPCBB1_PRIVCFGR31_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
pub mod gtzc1_mpcbb1_privcfgr31;
#[doc = "GTZC1_MPCBB2_PRIVCFGR0 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR0_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb2_privcfgr0::GTZC1_MPCBB2_PRIVCFGR0_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
pub mod gtzc1_mpcbb2_privcfgr0;
#[doc = "GTZC1_MPCBB2_PRIVCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR1_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb2_privcfgr1::GTZC1_MPCBB2_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
pub mod gtzc1_mpcbb2_privcfgr1;
#[doc = "GTZC1_MPCBB2_PRIVCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR2_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb2_privcfgr2::GTZC1_MPCBB2_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
pub mod gtzc1_mpcbb2_privcfgr2;
#[doc = "GTZC1_MPCBB2_PRIVCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR3_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb2_privcfgr3::GTZC1_MPCBB2_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
pub mod gtzc1_mpcbb2_privcfgr3;
#[doc = "GTZC1_MPCBB2_PRIVCFGR4 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR4_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb2_privcfgr4::GTZC1_MPCBB2_PRIVCFGR4_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
pub mod gtzc1_mpcbb2_privcfgr4;
#[doc = "GTZC1_MPCBB2_PRIVCFGR5 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR5_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb2_privcfgr5::GTZC1_MPCBB2_PRIVCFGR5_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
pub mod gtzc1_mpcbb2_privcfgr5;
#[doc = "GTZC1_MPCBB2_PRIVCFGR6 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR6_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb2_privcfgr6::GTZC1_MPCBB2_PRIVCFGR6_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
pub mod gtzc1_mpcbb2_privcfgr6;
#[doc = "GTZC1_MPCBB2_PRIVCFGR7 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR7_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb2_privcfgr7::GTZC1_MPCBB2_PRIVCFGR7_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
pub mod gtzc1_mpcbb2_privcfgr7;
#[doc = "GTZC1_MPCBB2_PRIVCFGR8 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR8_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb2_privcfgr8::GTZC1_MPCBB2_PRIVCFGR8_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
pub mod gtzc1_mpcbb2_privcfgr8;
#[doc = "GTZC1_MPCBB2_PRIVCFGR9 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR9_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb2_privcfgr9::GTZC1_MPCBB2_PRIVCFGR9_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
pub mod gtzc1_mpcbb2_privcfgr9;
#[doc = "GTZC1_MPCBB2_PRIVCFGR10 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR10_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR10 =
    crate::Reg<gtzc1_mpcbb2_privcfgr10::GTZC1_MPCBB2_PRIVCFGR10_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
pub mod gtzc1_mpcbb2_privcfgr10;
#[doc = "GTZC1_MPCBB2_PRIVCFGR11 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR11_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR11 =
    crate::Reg<gtzc1_mpcbb2_privcfgr11::GTZC1_MPCBB2_PRIVCFGR11_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
pub mod gtzc1_mpcbb2_privcfgr11;
#[doc = "GTZC1_MPCBB2_PRIVCFGR12 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR12_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR12 =
    crate::Reg<gtzc1_mpcbb2_privcfgr12::GTZC1_MPCBB2_PRIVCFGR12_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
pub mod gtzc1_mpcbb2_privcfgr12;
#[doc = "GTZC1_MPCBB2_PRIVCFGR13 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR13_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR13 =
    crate::Reg<gtzc1_mpcbb2_privcfgr13::GTZC1_MPCBB2_PRIVCFGR13_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
pub mod gtzc1_mpcbb2_privcfgr13;
#[doc = "GTZC1_MPCBB2_PRIVCFGR14 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR14_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR14 =
    crate::Reg<gtzc1_mpcbb2_privcfgr14::GTZC1_MPCBB2_PRIVCFGR14_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
pub mod gtzc1_mpcbb2_privcfgr14;
#[doc = "GTZC1_MPCBB2_PRIVCFGR15 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR15_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR15 =
    crate::Reg<gtzc1_mpcbb2_privcfgr15::GTZC1_MPCBB2_PRIVCFGR15_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
pub mod gtzc1_mpcbb2_privcfgr15;
#[doc = "GTZC1_MPCBB2_PRIVCFGR16 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR16_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR16 =
    crate::Reg<gtzc1_mpcbb2_privcfgr16::GTZC1_MPCBB2_PRIVCFGR16_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
pub mod gtzc1_mpcbb2_privcfgr16;
#[doc = "GTZC1_MPCBB2_PRIVCFGR17 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR17_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR17 =
    crate::Reg<gtzc1_mpcbb2_privcfgr17::GTZC1_MPCBB2_PRIVCFGR17_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
pub mod gtzc1_mpcbb2_privcfgr17;
#[doc = "GTZC1_MPCBB2_PRIVCFGR18 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR18_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR18 =
    crate::Reg<gtzc1_mpcbb2_privcfgr18::GTZC1_MPCBB2_PRIVCFGR18_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
pub mod gtzc1_mpcbb2_privcfgr18;
#[doc = "GTZC1_MPCBB2_PRIVCFGR19 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR19_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR19 =
    crate::Reg<gtzc1_mpcbb2_privcfgr19::GTZC1_MPCBB2_PRIVCFGR19_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
pub mod gtzc1_mpcbb2_privcfgr19;
#[doc = "GTZC1_MPCBB2_PRIVCFGR20 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR20_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR20 =
    crate::Reg<gtzc1_mpcbb2_privcfgr20::GTZC1_MPCBB2_PRIVCFGR20_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
pub mod gtzc1_mpcbb2_privcfgr20;
#[doc = "GTZC1_MPCBB2_PRIVCFGR21 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR21_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR21 =
    crate::Reg<gtzc1_mpcbb2_privcfgr21::GTZC1_MPCBB2_PRIVCFGR21_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
pub mod gtzc1_mpcbb2_privcfgr21;
#[doc = "GTZC1_MPCBB2_PRIVCFGR22 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR22_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR22 =
    crate::Reg<gtzc1_mpcbb2_privcfgr22::GTZC1_MPCBB2_PRIVCFGR22_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
pub mod gtzc1_mpcbb2_privcfgr22;
#[doc = "GTZC1_MPCBB2_PRIVCFGR23 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR23_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR23 =
    crate::Reg<gtzc1_mpcbb2_privcfgr23::GTZC1_MPCBB2_PRIVCFGR23_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
pub mod gtzc1_mpcbb2_privcfgr23;
#[doc = "GTZC1_MPCBB2_PRIVCFGR24 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR24_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR24 =
    crate::Reg<gtzc1_mpcbb2_privcfgr24::GTZC1_MPCBB2_PRIVCFGR24_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
pub mod gtzc1_mpcbb2_privcfgr24;
#[doc = "GTZC1_MPCBB2_PRIVCFGR25 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR25_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR25 =
    crate::Reg<gtzc1_mpcbb2_privcfgr25::GTZC1_MPCBB2_PRIVCFGR25_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
pub mod gtzc1_mpcbb2_privcfgr25;
#[doc = "GTZC1_MPCBB2_PRIVCFGR26 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR26_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR26 =
    crate::Reg<gtzc1_mpcbb2_privcfgr26::GTZC1_MPCBB2_PRIVCFGR26_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
pub mod gtzc1_mpcbb2_privcfgr26;
#[doc = "GTZC1_MPCBB2_PRIVCFGR27 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR27_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR27 =
    crate::Reg<gtzc1_mpcbb2_privcfgr27::GTZC1_MPCBB2_PRIVCFGR27_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
pub mod gtzc1_mpcbb2_privcfgr27;
#[doc = "GTZC1_MPCBB2_PRIVCFGR28 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR28_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR28 =
    crate::Reg<gtzc1_mpcbb2_privcfgr28::GTZC1_MPCBB2_PRIVCFGR28_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
pub mod gtzc1_mpcbb2_privcfgr28;
#[doc = "GTZC1_MPCBB2_PRIVCFGR29 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR29_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR29 =
    crate::Reg<gtzc1_mpcbb2_privcfgr29::GTZC1_MPCBB2_PRIVCFGR29_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
pub mod gtzc1_mpcbb2_privcfgr29;
#[doc = "GTZC1_MPCBB2_PRIVCFGR30 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR30_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR30 =
    crate::Reg<gtzc1_mpcbb2_privcfgr30::GTZC1_MPCBB2_PRIVCFGR30_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
pub mod gtzc1_mpcbb2_privcfgr30;
#[doc = "GTZC1_MPCBB2_PRIVCFGR31 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR31_SPEC>`"]
pub type GTZC1_MPCBB2_PRIVCFGR31 =
    crate::Reg<gtzc1_mpcbb2_privcfgr31::GTZC1_MPCBB2_PRIVCFGR31_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
pub mod gtzc1_mpcbb2_privcfgr31;
