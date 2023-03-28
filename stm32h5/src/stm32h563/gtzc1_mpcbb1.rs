#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GTZC1 SRAM1 MPCBB control register"]
    pub gtzc1_mpcbb1_cr: GTZC1_MPCBB1_CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - GTZC1 SRAM1 MPCBB configuration lock register 1"]
    pub gtzc1_mpcbb1_cfglock1: GTZC1_MPCBB1_CFGLOCK1,
    _reserved2: [u8; 0xec],
    #[doc = "0x100 - GTZC1 SRAM1 MPCBB security configuration for super-block 0 register"]
    pub gtzc1_mpcbb1_seccfgr0: GTZC1_MPCBB1_SECCFGR0,
    #[doc = "0x104 - GTZC1 SRAM1 MPCBB security configuration for super-block 1 register"]
    pub gtzc1_mpcbb1_seccfgr1: GTZC1_MPCBB1_SECCFGR1,
    #[doc = "0x108 - GTZC1 SRAM1 MPCBB security configuration for super-block 2 register"]
    pub gtzc1_mpcbb1_seccfgr2: GTZC1_MPCBB1_SECCFGR2,
    #[doc = "0x10c - GTZC1 SRAM1 MPCBB security configuration for super-block 3 register"]
    pub gtzc1_mpcbb1_seccfgr3: GTZC1_MPCBB1_SECCFGR3,
    #[doc = "0x110 - GTZC1 SRAM1 MPCBB security configuration for super-block 4 register"]
    pub gtzc1_mpcbb1_seccfgr4: GTZC1_MPCBB1_SECCFGR4,
    #[doc = "0x114 - GTZC1 SRAM1 MPCBB security configuration for super-block 5 register"]
    pub gtzc1_mpcbb1_seccfgr5: GTZC1_MPCBB1_SECCFGR5,
    #[doc = "0x118 - GTZC1 SRAM1 MPCBB security configuration for super-block 6 register"]
    pub gtzc1_mpcbb1_seccfgr6: GTZC1_MPCBB1_SECCFGR6,
    #[doc = "0x11c - GTZC1 SRAM1 MPCBB security configuration for super-block 7 register"]
    pub gtzc1_mpcbb1_seccfgr7: GTZC1_MPCBB1_SECCFGR7,
    #[doc = "0x120 - GTZC1 SRAM1 MPCBB security configuration for super-block 8 register"]
    pub gtzc1_mpcbb1_seccfgr8: GTZC1_MPCBB1_SECCFGR8,
    #[doc = "0x124 - GTZC1 SRAM1 MPCBB security configuration for super-block 9 register"]
    pub gtzc1_mpcbb1_seccfgr9: GTZC1_MPCBB1_SECCFGR9,
    #[doc = "0x128 - GTZC1 SRAM1 MPCBB security configuration for super-block 10 register"]
    pub gtzc1_mpcbb1_seccfgr10: GTZC1_MPCBB1_SECCFGR10,
    #[doc = "0x12c - GTZC1 SRAM1 MPCBB security configuration for super-block 11 register"]
    pub gtzc1_mpcbb1_seccfgr11: GTZC1_MPCBB1_SECCFGR11,
    #[doc = "0x130 - GTZC1 SRAM1 MPCBB security configuration for super-block 12 register"]
    pub gtzc1_mpcbb1_seccfgr12: GTZC1_MPCBB1_SECCFGR12,
    #[doc = "0x134 - GTZC1 SRAM1 MPCBB security configuration for super-block 13 register"]
    pub gtzc1_mpcbb1_seccfgr13: GTZC1_MPCBB1_SECCFGR13,
    #[doc = "0x138 - GTZC1 SRAM1 MPCBB security configuration for super-block 14 register"]
    pub gtzc1_mpcbb1_seccfgr14: GTZC1_MPCBB1_SECCFGR14,
    #[doc = "0x13c - GTZC1 SRAM1 MPCBB security configuration for super-block 15 register"]
    pub gtzc1_mpcbb1_seccfgr15: GTZC1_MPCBB1_SECCFGR15,
    #[doc = "0x140 - GTZC1 SRAM1 MPCBB security configuration for super-block 16 register"]
    pub gtzc1_mpcbb1_seccfgr16: GTZC1_MPCBB1_SECCFGR16,
    #[doc = "0x144 - GTZC1 SRAM1 MPCBB security configuration for super-block 17 register"]
    pub gtzc1_mpcbb1_seccfgr17: GTZC1_MPCBB1_SECCFGR17,
    #[doc = "0x148 - GTZC1 SRAM1 MPCBB security configuration for super-block 18 register"]
    pub gtzc1_mpcbb1_seccfgr18: GTZC1_MPCBB1_SECCFGR18,
    #[doc = "0x14c - GTZC1 SRAM1 MPCBB security configuration for super-block 19 register"]
    pub gtzc1_mpcbb1_seccfgr19: GTZC1_MPCBB1_SECCFGR19,
    #[doc = "0x150 - GTZC1 SRAM1 MPCBB security configuration for super-block 20 register"]
    pub gtzc1_mpcbb1_seccfgr20: GTZC1_MPCBB1_SECCFGR20,
    #[doc = "0x154 - GTZC1 SRAM1 MPCBB security configuration for super-block 21 register"]
    pub gtzc1_mpcbb1_seccfgr21: GTZC1_MPCBB1_SECCFGR21,
    #[doc = "0x158 - GTZC1 SRAM1 MPCBB security configuration for super-block 22 register"]
    pub gtzc1_mpcbb1_seccfgr22: GTZC1_MPCBB1_SECCFGR22,
    #[doc = "0x15c - GTZC1 SRAM1 MPCBB security configuration for super-block 23 register"]
    pub gtzc1_mpcbb1_seccfgr23: GTZC1_MPCBB1_SECCFGR23,
    #[doc = "0x160 - GTZC1 SRAM1 MPCBB security configuration for super-block 24 register"]
    pub gtzc1_mpcbb1_seccfgr24: GTZC1_MPCBB1_SECCFGR24,
    #[doc = "0x164 - GTZC1 SRAM1 MPCBB security configuration for super-block 25 register"]
    pub gtzc1_mpcbb1_seccfgr25: GTZC1_MPCBB1_SECCFGR25,
    #[doc = "0x168 - GTZC1 SRAM1 MPCBB security configuration for super-block 26 register"]
    pub gtzc1_mpcbb1_seccfgr26: GTZC1_MPCBB1_SECCFGR26,
    #[doc = "0x16c - GTZC1 SRAM1 MPCBB security configuration for super-block 27 register"]
    pub gtzc1_mpcbb1_seccfgr27: GTZC1_MPCBB1_SECCFGR27,
    #[doc = "0x170 - GTZC1 SRAM1 MPCBB security configuration for super-block 28 register"]
    pub gtzc1_mpcbb1_seccfgr28: GTZC1_MPCBB1_SECCFGR28,
    #[doc = "0x174 - GTZC1 SRAM1 MPCBB security configuration for super-block 29 register"]
    pub gtzc1_mpcbb1_seccfgr29: GTZC1_MPCBB1_SECCFGR29,
    #[doc = "0x178 - GTZC1 SRAM1 MPCBB security configuration for super-block 30 register"]
    pub gtzc1_mpcbb1_seccfgr30: GTZC1_MPCBB1_SECCFGR30,
    #[doc = "0x17c - GTZC1 SRAM1 MPCBB security configuration for super-block 31 register"]
    pub gtzc1_mpcbb1_seccfgr31: GTZC1_MPCBB1_SECCFGR31,
    _reserved34: [u8; 0x80],
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
}
#[doc = "GTZC1_MPCBB1_CR (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_CR_SPEC>`"]
pub type GTZC1_MPCBB1_CR = crate::Reg<gtzc1_mpcbb1_cr::GTZC1_MPCBB1_CR_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB control register"]
pub mod gtzc1_mpcbb1_cr;
#[doc = "GTZC1_MPCBB1_CFGLOCK1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_CFGLOCK1_SPEC>`"]
pub type GTZC1_MPCBB1_CFGLOCK1 = crate::Reg<gtzc1_mpcbb1_cfglock1::GTZC1_MPCBB1_CFGLOCK1_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB configuration lock register 1"]
pub mod gtzc1_mpcbb1_cfglock1;
#[doc = "GTZC1_MPCBB1_SECCFGR0 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR0_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR0 = crate::Reg<gtzc1_mpcbb1_seccfgr0::GTZC1_MPCBB1_SECCFGR0_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 0 register"]
pub mod gtzc1_mpcbb1_seccfgr0;
#[doc = "GTZC1_MPCBB1_SECCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR1_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR1 = crate::Reg<gtzc1_mpcbb1_seccfgr1::GTZC1_MPCBB1_SECCFGR1_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 1 register"]
pub mod gtzc1_mpcbb1_seccfgr1;
#[doc = "GTZC1_MPCBB1_SECCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR2_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR2 = crate::Reg<gtzc1_mpcbb1_seccfgr2::GTZC1_MPCBB1_SECCFGR2_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 2 register"]
pub mod gtzc1_mpcbb1_seccfgr2;
#[doc = "GTZC1_MPCBB1_SECCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR3_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR3 = crate::Reg<gtzc1_mpcbb1_seccfgr3::GTZC1_MPCBB1_SECCFGR3_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 3 register"]
pub mod gtzc1_mpcbb1_seccfgr3;
#[doc = "GTZC1_MPCBB1_SECCFGR4 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR4_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR4 = crate::Reg<gtzc1_mpcbb1_seccfgr4::GTZC1_MPCBB1_SECCFGR4_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 4 register"]
pub mod gtzc1_mpcbb1_seccfgr4;
#[doc = "GTZC1_MPCBB1_SECCFGR5 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR5_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR5 = crate::Reg<gtzc1_mpcbb1_seccfgr5::GTZC1_MPCBB1_SECCFGR5_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 5 register"]
pub mod gtzc1_mpcbb1_seccfgr5;
#[doc = "GTZC1_MPCBB1_SECCFGR6 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR6_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR6 = crate::Reg<gtzc1_mpcbb1_seccfgr6::GTZC1_MPCBB1_SECCFGR6_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 6 register"]
pub mod gtzc1_mpcbb1_seccfgr6;
#[doc = "GTZC1_MPCBB1_SECCFGR7 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR7_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR7 = crate::Reg<gtzc1_mpcbb1_seccfgr7::GTZC1_MPCBB1_SECCFGR7_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 7 register"]
pub mod gtzc1_mpcbb1_seccfgr7;
#[doc = "GTZC1_MPCBB1_SECCFGR8 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR8_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR8 = crate::Reg<gtzc1_mpcbb1_seccfgr8::GTZC1_MPCBB1_SECCFGR8_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 8 register"]
pub mod gtzc1_mpcbb1_seccfgr8;
#[doc = "GTZC1_MPCBB1_SECCFGR9 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR9_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR9 = crate::Reg<gtzc1_mpcbb1_seccfgr9::GTZC1_MPCBB1_SECCFGR9_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 9 register"]
pub mod gtzc1_mpcbb1_seccfgr9;
#[doc = "GTZC1_MPCBB1_SECCFGR10 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR10_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR10 = crate::Reg<gtzc1_mpcbb1_seccfgr10::GTZC1_MPCBB1_SECCFGR10_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 10 register"]
pub mod gtzc1_mpcbb1_seccfgr10;
#[doc = "GTZC1_MPCBB1_SECCFGR11 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR11_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR11 = crate::Reg<gtzc1_mpcbb1_seccfgr11::GTZC1_MPCBB1_SECCFGR11_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 11 register"]
pub mod gtzc1_mpcbb1_seccfgr11;
#[doc = "GTZC1_MPCBB1_SECCFGR12 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR12_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR12 = crate::Reg<gtzc1_mpcbb1_seccfgr12::GTZC1_MPCBB1_SECCFGR12_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 12 register"]
pub mod gtzc1_mpcbb1_seccfgr12;
#[doc = "GTZC1_MPCBB1_SECCFGR13 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR13_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR13 = crate::Reg<gtzc1_mpcbb1_seccfgr13::GTZC1_MPCBB1_SECCFGR13_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 13 register"]
pub mod gtzc1_mpcbb1_seccfgr13;
#[doc = "GTZC1_MPCBB1_SECCFGR14 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR14_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR14 = crate::Reg<gtzc1_mpcbb1_seccfgr14::GTZC1_MPCBB1_SECCFGR14_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 14 register"]
pub mod gtzc1_mpcbb1_seccfgr14;
#[doc = "GTZC1_MPCBB1_SECCFGR15 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR15_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR15 = crate::Reg<gtzc1_mpcbb1_seccfgr15::GTZC1_MPCBB1_SECCFGR15_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 15 register"]
pub mod gtzc1_mpcbb1_seccfgr15;
#[doc = "GTZC1_MPCBB1_SECCFGR16 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR16_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR16 = crate::Reg<gtzc1_mpcbb1_seccfgr16::GTZC1_MPCBB1_SECCFGR16_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 16 register"]
pub mod gtzc1_mpcbb1_seccfgr16;
#[doc = "GTZC1_MPCBB1_SECCFGR17 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR17_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR17 = crate::Reg<gtzc1_mpcbb1_seccfgr17::GTZC1_MPCBB1_SECCFGR17_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 17 register"]
pub mod gtzc1_mpcbb1_seccfgr17;
#[doc = "GTZC1_MPCBB1_SECCFGR18 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR18_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR18 = crate::Reg<gtzc1_mpcbb1_seccfgr18::GTZC1_MPCBB1_SECCFGR18_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 18 register"]
pub mod gtzc1_mpcbb1_seccfgr18;
#[doc = "GTZC1_MPCBB1_SECCFGR19 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR19_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR19 = crate::Reg<gtzc1_mpcbb1_seccfgr19::GTZC1_MPCBB1_SECCFGR19_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 19 register"]
pub mod gtzc1_mpcbb1_seccfgr19;
#[doc = "GTZC1_MPCBB1_SECCFGR20 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR20_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR20 = crate::Reg<gtzc1_mpcbb1_seccfgr20::GTZC1_MPCBB1_SECCFGR20_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 20 register"]
pub mod gtzc1_mpcbb1_seccfgr20;
#[doc = "GTZC1_MPCBB1_SECCFGR21 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR21_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR21 = crate::Reg<gtzc1_mpcbb1_seccfgr21::GTZC1_MPCBB1_SECCFGR21_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 21 register"]
pub mod gtzc1_mpcbb1_seccfgr21;
#[doc = "GTZC1_MPCBB1_SECCFGR22 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR22_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR22 = crate::Reg<gtzc1_mpcbb1_seccfgr22::GTZC1_MPCBB1_SECCFGR22_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 22 register"]
pub mod gtzc1_mpcbb1_seccfgr22;
#[doc = "GTZC1_MPCBB1_SECCFGR23 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR23_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR23 = crate::Reg<gtzc1_mpcbb1_seccfgr23::GTZC1_MPCBB1_SECCFGR23_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 23 register"]
pub mod gtzc1_mpcbb1_seccfgr23;
#[doc = "GTZC1_MPCBB1_SECCFGR24 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR24_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR24 = crate::Reg<gtzc1_mpcbb1_seccfgr24::GTZC1_MPCBB1_SECCFGR24_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 24 register"]
pub mod gtzc1_mpcbb1_seccfgr24;
#[doc = "GTZC1_MPCBB1_SECCFGR25 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR25_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR25 = crate::Reg<gtzc1_mpcbb1_seccfgr25::GTZC1_MPCBB1_SECCFGR25_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 25 register"]
pub mod gtzc1_mpcbb1_seccfgr25;
#[doc = "GTZC1_MPCBB1_SECCFGR26 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR26_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR26 = crate::Reg<gtzc1_mpcbb1_seccfgr26::GTZC1_MPCBB1_SECCFGR26_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 26 register"]
pub mod gtzc1_mpcbb1_seccfgr26;
#[doc = "GTZC1_MPCBB1_SECCFGR27 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR27_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR27 = crate::Reg<gtzc1_mpcbb1_seccfgr27::GTZC1_MPCBB1_SECCFGR27_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 27 register"]
pub mod gtzc1_mpcbb1_seccfgr27;
#[doc = "GTZC1_MPCBB1_SECCFGR28 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR28_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR28 = crate::Reg<gtzc1_mpcbb1_seccfgr28::GTZC1_MPCBB1_SECCFGR28_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 28 register"]
pub mod gtzc1_mpcbb1_seccfgr28;
#[doc = "GTZC1_MPCBB1_SECCFGR29 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR29_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR29 = crate::Reg<gtzc1_mpcbb1_seccfgr29::GTZC1_MPCBB1_SECCFGR29_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 29 register"]
pub mod gtzc1_mpcbb1_seccfgr29;
#[doc = "GTZC1_MPCBB1_SECCFGR30 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR30_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR30 = crate::Reg<gtzc1_mpcbb1_seccfgr30::GTZC1_MPCBB1_SECCFGR30_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 30 register"]
pub mod gtzc1_mpcbb1_seccfgr30;
#[doc = "GTZC1_MPCBB1_SECCFGR31 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB1_SECCFGR31_SPEC>`"]
pub type GTZC1_MPCBB1_SECCFGR31 = crate::Reg<gtzc1_mpcbb1_seccfgr31::GTZC1_MPCBB1_SECCFGR31_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB security configuration for super-block 31 register"]
pub mod gtzc1_mpcbb1_seccfgr31;
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
