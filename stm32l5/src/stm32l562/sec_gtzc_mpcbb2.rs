#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub mpcbb2_cr: MPCBB2_CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - MPCBB control register"]
    pub mpcbb2_lckvtr1: MPCBB2_LCKVTR1,
    #[doc = "0x14 - MPCBB control register"]
    pub mpcbb2_lckvtr2: MPCBB2_LCKVTR2,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100 - MPCBBx vector register"]
    pub mpcbb2_vctr0: MPCBB2_VCTR0,
    #[doc = "0x104 - MPCBBx vector register"]
    pub mpcbb2_vctr1: MPCBB2_VCTR1,
    #[doc = "0x108 - MPCBBx vector register"]
    pub mpcbb2_vctr2: MPCBB2_VCTR2,
    #[doc = "0x10c - MPCBBx vector register"]
    pub mpcbb2_vctr3: MPCBB2_VCTR3,
    #[doc = "0x110 - MPCBBx vector register"]
    pub mpcbb2_vctr4: MPCBB2_VCTR4,
    #[doc = "0x114 - MPCBBx vector register"]
    pub mpcbb2_vctr5: MPCBB2_VCTR5,
    #[doc = "0x118 - MPCBBx vector register"]
    pub mpcbb2_vctr6: MPCBB2_VCTR6,
    #[doc = "0x11c - MPCBBx vector register"]
    pub mpcbb2_vctr7: MPCBB2_VCTR7,
    #[doc = "0x120 - MPCBBx vector register"]
    pub mpcbb2_vctr8: MPCBB2_VCTR8,
    #[doc = "0x124 - MPCBBx vector register"]
    pub mpcbb2_vctr9: MPCBB2_VCTR9,
    #[doc = "0x128 - MPCBBx vector register"]
    pub mpcbb2_vctr10: MPCBB2_VCTR10,
    #[doc = "0x12c - MPCBBx vector register"]
    pub mpcbb2_vctr11: MPCBB2_VCTR11,
    #[doc = "0x130 - MPCBBx vector register"]
    pub mpcbb2_vctr12: MPCBB2_VCTR12,
    #[doc = "0x134 - MPCBBx vector register"]
    pub mpcbb2_vctr13: MPCBB2_VCTR13,
    #[doc = "0x138 - MPCBBx vector register"]
    pub mpcbb2_vctr14: MPCBB2_VCTR14,
    #[doc = "0x13c - MPCBBx vector register"]
    pub mpcbb2_vctr15: MPCBB2_VCTR15,
    #[doc = "0x140 - MPCBBx vector register"]
    pub mpcbb2_vctr16: MPCBB2_VCTR16,
    #[doc = "0x144 - MPCBBx vector register"]
    pub mpcbb2_vctr17: MPCBB2_VCTR17,
    #[doc = "0x148 - MPCBBx vector register"]
    pub mpcbb2_vctr18: MPCBB2_VCTR18,
    #[doc = "0x14c - MPCBBx vector register"]
    pub mpcbb2_vctr19: MPCBB2_VCTR19,
    #[doc = "0x150 - MPCBBx vector register"]
    pub mpcbb2_vctr20: MPCBB2_VCTR20,
    #[doc = "0x154 - MPCBBx vector register"]
    pub mpcbb2_vctr21: MPCBB2_VCTR21,
    #[doc = "0x158 - MPCBBx vector register"]
    pub mpcbb2_vctr22: MPCBB2_VCTR22,
    #[doc = "0x15c - MPCBBx vector register"]
    pub mpcbb2_vctr23: MPCBB2_VCTR23,
    #[doc = "0x160 - MPCBBx vector register"]
    pub mpcbb2_vctr24: MPCBB2_VCTR24,
    #[doc = "0x164 - MPCBBx vector register"]
    pub mpcbb2_vctr25: MPCBB2_VCTR25,
    #[doc = "0x168 - MPCBBx vector register"]
    pub mpcbb2_vctr26: MPCBB2_VCTR26,
    #[doc = "0x16c - MPCBBx vector register"]
    pub mpcbb2_vctr27: MPCBB2_VCTR27,
    #[doc = "0x170 - MPCBBx vector register"]
    pub mpcbb2_vctr28: MPCBB2_VCTR28,
    #[doc = "0x174 - MPCBBx vector register"]
    pub mpcbb2_vctr29: MPCBB2_VCTR29,
    #[doc = "0x178 - MPCBBx vector register"]
    pub mpcbb2_vctr30: MPCBB2_VCTR30,
    #[doc = "0x17c - MPCBBx vector register"]
    pub mpcbb2_vctr31: MPCBB2_VCTR31,
    #[doc = "0x180 - MPCBBx vector register"]
    pub mpcbb2_vctr32: MPCBB2_VCTR32,
    #[doc = "0x184 - MPCBBx vector register"]
    pub mpcbb2_vctr33: MPCBB2_VCTR33,
    #[doc = "0x188 - MPCBBx vector register"]
    pub mpcbb2_vctr34: MPCBB2_VCTR34,
    #[doc = "0x18c - MPCBBx vector register"]
    pub mpcbb2_vctr35: MPCBB2_VCTR35,
    #[doc = "0x190 - MPCBBx vector register"]
    pub mpcbb2_vctr36: MPCBB2_VCTR36,
    #[doc = "0x194 - MPCBBx vector register"]
    pub mpcbb2_vctr37: MPCBB2_VCTR37,
    #[doc = "0x198 - MPCBBx vector register"]
    pub mpcbb2_vctr38: MPCBB2_VCTR38,
    #[doc = "0x19c - MPCBBx vector register"]
    pub mpcbb2_vctr39: MPCBB2_VCTR39,
    #[doc = "0x1a0 - MPCBBx vector register"]
    pub mpcbb2_vctr40: MPCBB2_VCTR40,
    #[doc = "0x1a4 - MPCBBx vector register"]
    pub mpcbb2_vctr41: MPCBB2_VCTR41,
    #[doc = "0x1a8 - MPCBBx vector register"]
    pub mpcbb2_vctr42: MPCBB2_VCTR42,
    #[doc = "0x1ac - MPCBBx vector register"]
    pub mpcbb2_vctr43: MPCBB2_VCTR43,
    #[doc = "0x1b0 - MPCBBx vector register"]
    pub mpcbb2_vctr44: MPCBB2_VCTR44,
    #[doc = "0x1b4 - MPCBBx vector register"]
    pub mpcbb2_vctr45: MPCBB2_VCTR45,
    #[doc = "0x1b8 - MPCBBx vector register"]
    pub mpcbb2_vctr46: MPCBB2_VCTR46,
    #[doc = "0x1bc - MPCBBx vector register"]
    pub mpcbb2_vctr47: MPCBB2_VCTR47,
    #[doc = "0x1c0 - MPCBBx vector register"]
    pub mpcbb2_vctr48: MPCBB2_VCTR48,
    #[doc = "0x1c4 - MPCBBx vector register"]
    pub mpcbb2_vctr49: MPCBB2_VCTR49,
    #[doc = "0x1c8 - MPCBBx vector register"]
    pub mpcbb2_vctr50: MPCBB2_VCTR50,
    #[doc = "0x1cc - MPCBBx vector register"]
    pub mpcbb2_vctr51: MPCBB2_VCTR51,
    #[doc = "0x1d0 - MPCBBx vector register"]
    pub mpcbb2_vctr52: MPCBB2_VCTR52,
    #[doc = "0x1d4 - MPCBBx vector register"]
    pub mpcbb2_vctr53: MPCBB2_VCTR53,
    #[doc = "0x1d8 - MPCBBx vector register"]
    pub mpcbb2_vctr54: MPCBB2_VCTR54,
    #[doc = "0x1dc - MPCBBx vector register"]
    pub mpcbb2_vctr55: MPCBB2_VCTR55,
    #[doc = "0x1e0 - MPCBBx vector register"]
    pub mpcbb2_vctr56: MPCBB2_VCTR56,
    #[doc = "0x1e4 - MPCBBx vector register"]
    pub mpcbb2_vctr57: MPCBB2_VCTR57,
    #[doc = "0x1e8 - MPCBBx vector register"]
    pub mpcbb2_vctr58: MPCBB2_VCTR58,
    #[doc = "0x1ec - MPCBBx vector register"]
    pub mpcbb2_vctr59: MPCBB2_VCTR59,
    #[doc = "0x1f0 - MPCBBx vector register"]
    pub mpcbb2_vctr60: MPCBB2_VCTR60,
    #[doc = "0x1f4 - MPCBBx vector register"]
    pub mpcbb2_vctr61: MPCBB2_VCTR61,
    #[doc = "0x1f8 - MPCBBx vector register"]
    pub mpcbb2_vctr62: MPCBB2_VCTR62,
    #[doc = "0x1fc - MPCBBx vector register"]
    pub mpcbb2_vctr63: MPCBB2_VCTR63,
}
#[doc = "MPCBB2_CR (rw) register accessor: an alias for `Reg<MPCBB2_CR_SPEC>`"]
pub type MPCBB2_CR = crate::Reg<mpcbb2_cr::MPCBB2_CR_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_cr;
#[doc = "MPCBB2_LCKVTR1 (rw) register accessor: an alias for `Reg<MPCBB2_LCKVTR1_SPEC>`"]
pub type MPCBB2_LCKVTR1 = crate::Reg<mpcbb2_lckvtr1::MPCBB2_LCKVTR1_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr1;
#[doc = "MPCBB2_LCKVTR2 (rw) register accessor: an alias for `Reg<MPCBB2_LCKVTR2_SPEC>`"]
pub type MPCBB2_LCKVTR2 = crate::Reg<mpcbb2_lckvtr2::MPCBB2_LCKVTR2_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr2;
#[doc = "MPCBB2_VCTR0 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR0_SPEC>`"]
pub type MPCBB2_VCTR0 = crate::Reg<mpcbb2_vctr0::MPCBB2_VCTR0_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr0;
#[doc = "MPCBB2_VCTR1 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR1_SPEC>`"]
pub type MPCBB2_VCTR1 = crate::Reg<mpcbb2_vctr1::MPCBB2_VCTR1_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr1;
#[doc = "MPCBB2_VCTR2 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR2_SPEC>`"]
pub type MPCBB2_VCTR2 = crate::Reg<mpcbb2_vctr2::MPCBB2_VCTR2_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr2;
#[doc = "MPCBB2_VCTR3 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR3_SPEC>`"]
pub type MPCBB2_VCTR3 = crate::Reg<mpcbb2_vctr3::MPCBB2_VCTR3_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr3;
#[doc = "MPCBB2_VCTR4 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR4_SPEC>`"]
pub type MPCBB2_VCTR4 = crate::Reg<mpcbb2_vctr4::MPCBB2_VCTR4_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr4;
#[doc = "MPCBB2_VCTR5 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR5_SPEC>`"]
pub type MPCBB2_VCTR5 = crate::Reg<mpcbb2_vctr5::MPCBB2_VCTR5_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr5;
#[doc = "MPCBB2_VCTR6 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR6_SPEC>`"]
pub type MPCBB2_VCTR6 = crate::Reg<mpcbb2_vctr6::MPCBB2_VCTR6_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr6;
#[doc = "MPCBB2_VCTR7 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR7_SPEC>`"]
pub type MPCBB2_VCTR7 = crate::Reg<mpcbb2_vctr7::MPCBB2_VCTR7_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr7;
#[doc = "MPCBB2_VCTR8 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR8_SPEC>`"]
pub type MPCBB2_VCTR8 = crate::Reg<mpcbb2_vctr8::MPCBB2_VCTR8_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr8;
#[doc = "MPCBB2_VCTR9 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR9_SPEC>`"]
pub type MPCBB2_VCTR9 = crate::Reg<mpcbb2_vctr9::MPCBB2_VCTR9_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr9;
#[doc = "MPCBB2_VCTR10 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR10_SPEC>`"]
pub type MPCBB2_VCTR10 = crate::Reg<mpcbb2_vctr10::MPCBB2_VCTR10_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr10;
#[doc = "MPCBB2_VCTR11 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR11_SPEC>`"]
pub type MPCBB2_VCTR11 = crate::Reg<mpcbb2_vctr11::MPCBB2_VCTR11_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr11;
#[doc = "MPCBB2_VCTR12 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR12_SPEC>`"]
pub type MPCBB2_VCTR12 = crate::Reg<mpcbb2_vctr12::MPCBB2_VCTR12_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr12;
#[doc = "MPCBB2_VCTR13 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR13_SPEC>`"]
pub type MPCBB2_VCTR13 = crate::Reg<mpcbb2_vctr13::MPCBB2_VCTR13_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr13;
#[doc = "MPCBB2_VCTR14 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR14_SPEC>`"]
pub type MPCBB2_VCTR14 = crate::Reg<mpcbb2_vctr14::MPCBB2_VCTR14_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr14;
#[doc = "MPCBB2_VCTR15 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR15_SPEC>`"]
pub type MPCBB2_VCTR15 = crate::Reg<mpcbb2_vctr15::MPCBB2_VCTR15_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr15;
#[doc = "MPCBB2_VCTR16 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR16_SPEC>`"]
pub type MPCBB2_VCTR16 = crate::Reg<mpcbb2_vctr16::MPCBB2_VCTR16_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr16;
#[doc = "MPCBB2_VCTR17 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR17_SPEC>`"]
pub type MPCBB2_VCTR17 = crate::Reg<mpcbb2_vctr17::MPCBB2_VCTR17_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr17;
#[doc = "MPCBB2_VCTR18 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR18_SPEC>`"]
pub type MPCBB2_VCTR18 = crate::Reg<mpcbb2_vctr18::MPCBB2_VCTR18_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr18;
#[doc = "MPCBB2_VCTR19 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR19_SPEC>`"]
pub type MPCBB2_VCTR19 = crate::Reg<mpcbb2_vctr19::MPCBB2_VCTR19_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr19;
#[doc = "MPCBB2_VCTR20 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR20_SPEC>`"]
pub type MPCBB2_VCTR20 = crate::Reg<mpcbb2_vctr20::MPCBB2_VCTR20_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr20;
#[doc = "MPCBB2_VCTR21 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR21_SPEC>`"]
pub type MPCBB2_VCTR21 = crate::Reg<mpcbb2_vctr21::MPCBB2_VCTR21_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr21;
#[doc = "MPCBB2_VCTR22 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR22_SPEC>`"]
pub type MPCBB2_VCTR22 = crate::Reg<mpcbb2_vctr22::MPCBB2_VCTR22_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr22;
#[doc = "MPCBB2_VCTR23 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR23_SPEC>`"]
pub type MPCBB2_VCTR23 = crate::Reg<mpcbb2_vctr23::MPCBB2_VCTR23_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr23;
#[doc = "MPCBB2_VCTR24 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR24_SPEC>`"]
pub type MPCBB2_VCTR24 = crate::Reg<mpcbb2_vctr24::MPCBB2_VCTR24_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr24;
#[doc = "MPCBB2_VCTR25 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR25_SPEC>`"]
pub type MPCBB2_VCTR25 = crate::Reg<mpcbb2_vctr25::MPCBB2_VCTR25_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr25;
#[doc = "MPCBB2_VCTR26 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR26_SPEC>`"]
pub type MPCBB2_VCTR26 = crate::Reg<mpcbb2_vctr26::MPCBB2_VCTR26_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr26;
#[doc = "MPCBB2_VCTR27 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR27_SPEC>`"]
pub type MPCBB2_VCTR27 = crate::Reg<mpcbb2_vctr27::MPCBB2_VCTR27_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr27;
#[doc = "MPCBB2_VCTR28 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR28_SPEC>`"]
pub type MPCBB2_VCTR28 = crate::Reg<mpcbb2_vctr28::MPCBB2_VCTR28_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr28;
#[doc = "MPCBB2_VCTR29 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR29_SPEC>`"]
pub type MPCBB2_VCTR29 = crate::Reg<mpcbb2_vctr29::MPCBB2_VCTR29_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr29;
#[doc = "MPCBB2_VCTR30 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR30_SPEC>`"]
pub type MPCBB2_VCTR30 = crate::Reg<mpcbb2_vctr30::MPCBB2_VCTR30_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr30;
#[doc = "MPCBB2_VCTR31 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR31_SPEC>`"]
pub type MPCBB2_VCTR31 = crate::Reg<mpcbb2_vctr31::MPCBB2_VCTR31_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr31;
#[doc = "MPCBB2_VCTR32 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR32_SPEC>`"]
pub type MPCBB2_VCTR32 = crate::Reg<mpcbb2_vctr32::MPCBB2_VCTR32_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr32;
#[doc = "MPCBB2_VCTR33 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR33_SPEC>`"]
pub type MPCBB2_VCTR33 = crate::Reg<mpcbb2_vctr33::MPCBB2_VCTR33_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr33;
#[doc = "MPCBB2_VCTR34 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR34_SPEC>`"]
pub type MPCBB2_VCTR34 = crate::Reg<mpcbb2_vctr34::MPCBB2_VCTR34_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr34;
#[doc = "MPCBB2_VCTR35 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR35_SPEC>`"]
pub type MPCBB2_VCTR35 = crate::Reg<mpcbb2_vctr35::MPCBB2_VCTR35_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr35;
#[doc = "MPCBB2_VCTR36 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR36_SPEC>`"]
pub type MPCBB2_VCTR36 = crate::Reg<mpcbb2_vctr36::MPCBB2_VCTR36_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr36;
#[doc = "MPCBB2_VCTR37 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR37_SPEC>`"]
pub type MPCBB2_VCTR37 = crate::Reg<mpcbb2_vctr37::MPCBB2_VCTR37_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr37;
#[doc = "MPCBB2_VCTR38 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR38_SPEC>`"]
pub type MPCBB2_VCTR38 = crate::Reg<mpcbb2_vctr38::MPCBB2_VCTR38_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr38;
#[doc = "MPCBB2_VCTR39 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR39_SPEC>`"]
pub type MPCBB2_VCTR39 = crate::Reg<mpcbb2_vctr39::MPCBB2_VCTR39_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr39;
#[doc = "MPCBB2_VCTR40 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR40_SPEC>`"]
pub type MPCBB2_VCTR40 = crate::Reg<mpcbb2_vctr40::MPCBB2_VCTR40_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr40;
#[doc = "MPCBB2_VCTR41 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR41_SPEC>`"]
pub type MPCBB2_VCTR41 = crate::Reg<mpcbb2_vctr41::MPCBB2_VCTR41_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr41;
#[doc = "MPCBB2_VCTR42 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR42_SPEC>`"]
pub type MPCBB2_VCTR42 = crate::Reg<mpcbb2_vctr42::MPCBB2_VCTR42_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr42;
#[doc = "MPCBB2_VCTR43 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR43_SPEC>`"]
pub type MPCBB2_VCTR43 = crate::Reg<mpcbb2_vctr43::MPCBB2_VCTR43_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr43;
#[doc = "MPCBB2_VCTR44 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR44_SPEC>`"]
pub type MPCBB2_VCTR44 = crate::Reg<mpcbb2_vctr44::MPCBB2_VCTR44_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr44;
#[doc = "MPCBB2_VCTR45 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR45_SPEC>`"]
pub type MPCBB2_VCTR45 = crate::Reg<mpcbb2_vctr45::MPCBB2_VCTR45_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr45;
#[doc = "MPCBB2_VCTR46 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR46_SPEC>`"]
pub type MPCBB2_VCTR46 = crate::Reg<mpcbb2_vctr46::MPCBB2_VCTR46_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr46;
#[doc = "MPCBB2_VCTR47 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR47_SPEC>`"]
pub type MPCBB2_VCTR47 = crate::Reg<mpcbb2_vctr47::MPCBB2_VCTR47_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr47;
#[doc = "MPCBB2_VCTR48 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR48_SPEC>`"]
pub type MPCBB2_VCTR48 = crate::Reg<mpcbb2_vctr48::MPCBB2_VCTR48_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr48;
#[doc = "MPCBB2_VCTR49 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR49_SPEC>`"]
pub type MPCBB2_VCTR49 = crate::Reg<mpcbb2_vctr49::MPCBB2_VCTR49_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr49;
#[doc = "MPCBB2_VCTR50 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR50_SPEC>`"]
pub type MPCBB2_VCTR50 = crate::Reg<mpcbb2_vctr50::MPCBB2_VCTR50_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr50;
#[doc = "MPCBB2_VCTR51 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR51_SPEC>`"]
pub type MPCBB2_VCTR51 = crate::Reg<mpcbb2_vctr51::MPCBB2_VCTR51_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr51;
#[doc = "MPCBB2_VCTR52 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR52_SPEC>`"]
pub type MPCBB2_VCTR52 = crate::Reg<mpcbb2_vctr52::MPCBB2_VCTR52_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr52;
#[doc = "MPCBB2_VCTR53 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR53_SPEC>`"]
pub type MPCBB2_VCTR53 = crate::Reg<mpcbb2_vctr53::MPCBB2_VCTR53_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr53;
#[doc = "MPCBB2_VCTR54 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR54_SPEC>`"]
pub type MPCBB2_VCTR54 = crate::Reg<mpcbb2_vctr54::MPCBB2_VCTR54_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr54;
#[doc = "MPCBB2_VCTR55 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR55_SPEC>`"]
pub type MPCBB2_VCTR55 = crate::Reg<mpcbb2_vctr55::MPCBB2_VCTR55_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr55;
#[doc = "MPCBB2_VCTR56 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR56_SPEC>`"]
pub type MPCBB2_VCTR56 = crate::Reg<mpcbb2_vctr56::MPCBB2_VCTR56_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr56;
#[doc = "MPCBB2_VCTR57 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR57_SPEC>`"]
pub type MPCBB2_VCTR57 = crate::Reg<mpcbb2_vctr57::MPCBB2_VCTR57_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr57;
#[doc = "MPCBB2_VCTR58 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR58_SPEC>`"]
pub type MPCBB2_VCTR58 = crate::Reg<mpcbb2_vctr58::MPCBB2_VCTR58_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr58;
#[doc = "MPCBB2_VCTR59 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR59_SPEC>`"]
pub type MPCBB2_VCTR59 = crate::Reg<mpcbb2_vctr59::MPCBB2_VCTR59_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr59;
#[doc = "MPCBB2_VCTR60 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR60_SPEC>`"]
pub type MPCBB2_VCTR60 = crate::Reg<mpcbb2_vctr60::MPCBB2_VCTR60_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr60;
#[doc = "MPCBB2_VCTR61 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR61_SPEC>`"]
pub type MPCBB2_VCTR61 = crate::Reg<mpcbb2_vctr61::MPCBB2_VCTR61_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr61;
#[doc = "MPCBB2_VCTR62 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR62_SPEC>`"]
pub type MPCBB2_VCTR62 = crate::Reg<mpcbb2_vctr62::MPCBB2_VCTR62_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr62;
#[doc = "MPCBB2_VCTR63 (rw) register accessor: an alias for `Reg<MPCBB2_VCTR63_SPEC>`"]
pub type MPCBB2_VCTR63 = crate::Reg<mpcbb2_vctr63::MPCBB2_VCTR63_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr63;
