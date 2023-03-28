#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub mpcbb1_cr: MPCBB1_CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - MPCBB control register"]
    pub mpcbb1_lckvtr1: MPCBB1_LCKVTR1,
    #[doc = "0x14 - MPCBB control register"]
    pub mpcbb1_lckvtr2: MPCBB1_LCKVTR2,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100 - MPCBBx vector register"]
    pub mpcbb1_vctr0: MPCBB1_VCTR0,
    #[doc = "0x104 - MPCBBx vector register"]
    pub mpcbb1_vctr1: MPCBB1_VCTR1,
    #[doc = "0x108 - MPCBBx vector register"]
    pub mpcbb1_vctr2: MPCBB1_VCTR2,
    #[doc = "0x10c - MPCBBx vector register"]
    pub mpcbb1_vctr3: MPCBB1_VCTR3,
    #[doc = "0x110 - MPCBBx vector register"]
    pub mpcbb1_vctr4: MPCBB1_VCTR4,
    #[doc = "0x114 - MPCBBx vector register"]
    pub mpcbb1_vctr5: MPCBB1_VCTR5,
    #[doc = "0x118 - MPCBBx vector register"]
    pub mpcbb1_vctr6: MPCBB1_VCTR6,
    #[doc = "0x11c - MPCBBx vector register"]
    pub mpcbb1_vctr7: MPCBB1_VCTR7,
    #[doc = "0x120 - MPCBBx vector register"]
    pub mpcbb1_vctr8: MPCBB1_VCTR8,
    #[doc = "0x124 - MPCBBx vector register"]
    pub mpcbb1_vctr9: MPCBB1_VCTR9,
    #[doc = "0x128 - MPCBBx vector register"]
    pub mpcbb1_vctr10: MPCBB1_VCTR10,
    #[doc = "0x12c - MPCBBx vector register"]
    pub mpcbb1_vctr11: MPCBB1_VCTR11,
    #[doc = "0x130 - MPCBBx vector register"]
    pub mpcbb1_vctr12: MPCBB1_VCTR12,
    #[doc = "0x134 - MPCBBx vector register"]
    pub mpcbb1_vctr13: MPCBB1_VCTR13,
    #[doc = "0x138 - MPCBBx vector register"]
    pub mpcbb1_vctr14: MPCBB1_VCTR14,
    #[doc = "0x13c - MPCBBx vector register"]
    pub mpcbb1_vctr15: MPCBB1_VCTR15,
    #[doc = "0x140 - MPCBBx vector register"]
    pub mpcbb1_vctr16: MPCBB1_VCTR16,
    #[doc = "0x144 - MPCBBx vector register"]
    pub mpcbb1_vctr17: MPCBB1_VCTR17,
    #[doc = "0x148 - MPCBBx vector register"]
    pub mpcbb1_vctr18: MPCBB1_VCTR18,
    #[doc = "0x14c - MPCBBx vector register"]
    pub mpcbb1_vctr19: MPCBB1_VCTR19,
    #[doc = "0x150 - MPCBBx vector register"]
    pub mpcbb1_vctr20: MPCBB1_VCTR20,
    #[doc = "0x154 - MPCBBx vector register"]
    pub mpcbb1_vctr21: MPCBB1_VCTR21,
    #[doc = "0x158 - MPCBBx vector register"]
    pub mpcbb1_vctr22: MPCBB1_VCTR22,
    #[doc = "0x15c - MPCBBx vector register"]
    pub mpcbb1_vctr23: MPCBB1_VCTR23,
    #[doc = "0x160 - MPCBBx vector register"]
    pub mpcbb1_vctr24: MPCBB1_VCTR24,
    #[doc = "0x164 - MPCBBx vector register"]
    pub mpcbb1_vctr25: MPCBB1_VCTR25,
    #[doc = "0x168 - MPCBBx vector register"]
    pub mpcbb1_vctr26: MPCBB1_VCTR26,
    #[doc = "0x16c - MPCBBx vector register"]
    pub mpcbb1_vctr27: MPCBB1_VCTR27,
    #[doc = "0x170 - MPCBBx vector register"]
    pub mpcbb1_vctr28: MPCBB1_VCTR28,
    #[doc = "0x174 - MPCBBx vector register"]
    pub mpcbb1_vctr29: MPCBB1_VCTR29,
    #[doc = "0x178 - MPCBBx vector register"]
    pub mpcbb1_vctr30: MPCBB1_VCTR30,
    #[doc = "0x17c - MPCBBx vector register"]
    pub mpcbb1_vctr31: MPCBB1_VCTR31,
    #[doc = "0x180 - MPCBBx vector register"]
    pub mpcbb1_vctr32: MPCBB1_VCTR32,
    #[doc = "0x184 - MPCBBx vector register"]
    pub mpcbb1_vctr33: MPCBB1_VCTR33,
    #[doc = "0x188 - MPCBBx vector register"]
    pub mpcbb1_vctr34: MPCBB1_VCTR34,
    #[doc = "0x18c - MPCBBx vector register"]
    pub mpcbb1_vctr35: MPCBB1_VCTR35,
    #[doc = "0x190 - MPCBBx vector register"]
    pub mpcbb1_vctr36: MPCBB1_VCTR36,
    #[doc = "0x194 - MPCBBx vector register"]
    pub mpcbb1_vctr37: MPCBB1_VCTR37,
    #[doc = "0x198 - MPCBBx vector register"]
    pub mpcbb1_vctr38: MPCBB1_VCTR38,
    #[doc = "0x19c - MPCBBx vector register"]
    pub mpcbb1_vctr39: MPCBB1_VCTR39,
    #[doc = "0x1a0 - MPCBBx vector register"]
    pub mpcbb1_vctr40: MPCBB1_VCTR40,
    #[doc = "0x1a4 - MPCBBx vector register"]
    pub mpcbb1_vctr41: MPCBB1_VCTR41,
    #[doc = "0x1a8 - MPCBBx vector register"]
    pub mpcbb1_vctr42: MPCBB1_VCTR42,
    #[doc = "0x1ac - MPCBBx vector register"]
    pub mpcbb1_vctr43: MPCBB1_VCTR43,
    #[doc = "0x1b0 - MPCBBx vector register"]
    pub mpcbb1_vctr44: MPCBB1_VCTR44,
    #[doc = "0x1b4 - MPCBBx vector register"]
    pub mpcbb1_vctr45: MPCBB1_VCTR45,
    #[doc = "0x1b8 - MPCBBx vector register"]
    pub mpcbb1_vctr46: MPCBB1_VCTR46,
    #[doc = "0x1bc - MPCBBx vector register"]
    pub mpcbb1_vctr47: MPCBB1_VCTR47,
    #[doc = "0x1c0 - MPCBBx vector register"]
    pub mpcbb1_vctr48: MPCBB1_VCTR48,
    #[doc = "0x1c4 - MPCBBx vector register"]
    pub mpcbb1_vctr49: MPCBB1_VCTR49,
    #[doc = "0x1c8 - MPCBBx vector register"]
    pub mpcbb1_vctr50: MPCBB1_VCTR50,
    #[doc = "0x1cc - MPCBBx vector register"]
    pub mpcbb1_vctr51: MPCBB1_VCTR51,
    #[doc = "0x1d0 - MPCBBx vector register"]
    pub mpcbb1_vctr52: MPCBB1_VCTR52,
    #[doc = "0x1d4 - MPCBBx vector register"]
    pub mpcbb1_vctr53: MPCBB1_VCTR53,
    #[doc = "0x1d8 - MPCBBx vector register"]
    pub mpcbb1_vctr54: MPCBB1_VCTR54,
    #[doc = "0x1dc - MPCBBx vector register"]
    pub mpcbb1_vctr55: MPCBB1_VCTR55,
    #[doc = "0x1e0 - MPCBBx vector register"]
    pub mpcbb1_vctr56: MPCBB1_VCTR56,
    #[doc = "0x1e4 - MPCBBx vector register"]
    pub mpcbb1_vctr57: MPCBB1_VCTR57,
    #[doc = "0x1e8 - MPCBBx vector register"]
    pub mpcbb1_vctr58: MPCBB1_VCTR58,
    #[doc = "0x1ec - MPCBBx vector register"]
    pub mpcbb1_vctr59: MPCBB1_VCTR59,
    #[doc = "0x1f0 - MPCBBx vector register"]
    pub mpcbb1_vctr60: MPCBB1_VCTR60,
    #[doc = "0x1f4 - MPCBBx vector register"]
    pub mpcbb1_vctr61: MPCBB1_VCTR61,
    #[doc = "0x1f8 - MPCBBx vector register"]
    pub mpcbb1_vctr62: MPCBB1_VCTR62,
    #[doc = "0x1fc - MPCBBx vector register"]
    pub mpcbb1_vctr63: MPCBB1_VCTR63,
}
#[doc = "MPCBB1_CR (rw) register accessor: an alias for `Reg<MPCBB1_CR_SPEC>`"]
pub type MPCBB1_CR = crate::Reg<mpcbb1_cr::MPCBB1_CR_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_cr;
#[doc = "MPCBB1_LCKVTR1 (rw) register accessor: an alias for `Reg<MPCBB1_LCKVTR1_SPEC>`"]
pub type MPCBB1_LCKVTR1 = crate::Reg<mpcbb1_lckvtr1::MPCBB1_LCKVTR1_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr1;
#[doc = "MPCBB1_LCKVTR2 (rw) register accessor: an alias for `Reg<MPCBB1_LCKVTR2_SPEC>`"]
pub type MPCBB1_LCKVTR2 = crate::Reg<mpcbb1_lckvtr2::MPCBB1_LCKVTR2_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr2;
#[doc = "MPCBB1_VCTR0 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR0_SPEC>`"]
pub type MPCBB1_VCTR0 = crate::Reg<mpcbb1_vctr0::MPCBB1_VCTR0_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr0;
#[doc = "MPCBB1_VCTR1 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR1_SPEC>`"]
pub type MPCBB1_VCTR1 = crate::Reg<mpcbb1_vctr1::MPCBB1_VCTR1_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr1;
#[doc = "MPCBB1_VCTR2 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR2_SPEC>`"]
pub type MPCBB1_VCTR2 = crate::Reg<mpcbb1_vctr2::MPCBB1_VCTR2_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr2;
#[doc = "MPCBB1_VCTR3 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR3_SPEC>`"]
pub type MPCBB1_VCTR3 = crate::Reg<mpcbb1_vctr3::MPCBB1_VCTR3_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr3;
#[doc = "MPCBB1_VCTR4 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR4_SPEC>`"]
pub type MPCBB1_VCTR4 = crate::Reg<mpcbb1_vctr4::MPCBB1_VCTR4_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr4;
#[doc = "MPCBB1_VCTR5 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR5_SPEC>`"]
pub type MPCBB1_VCTR5 = crate::Reg<mpcbb1_vctr5::MPCBB1_VCTR5_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr5;
#[doc = "MPCBB1_VCTR6 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR6_SPEC>`"]
pub type MPCBB1_VCTR6 = crate::Reg<mpcbb1_vctr6::MPCBB1_VCTR6_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr6;
#[doc = "MPCBB1_VCTR7 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR7_SPEC>`"]
pub type MPCBB1_VCTR7 = crate::Reg<mpcbb1_vctr7::MPCBB1_VCTR7_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr7;
#[doc = "MPCBB1_VCTR8 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR8_SPEC>`"]
pub type MPCBB1_VCTR8 = crate::Reg<mpcbb1_vctr8::MPCBB1_VCTR8_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr8;
#[doc = "MPCBB1_VCTR9 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR9_SPEC>`"]
pub type MPCBB1_VCTR9 = crate::Reg<mpcbb1_vctr9::MPCBB1_VCTR9_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr9;
#[doc = "MPCBB1_VCTR10 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR10_SPEC>`"]
pub type MPCBB1_VCTR10 = crate::Reg<mpcbb1_vctr10::MPCBB1_VCTR10_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr10;
#[doc = "MPCBB1_VCTR11 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR11_SPEC>`"]
pub type MPCBB1_VCTR11 = crate::Reg<mpcbb1_vctr11::MPCBB1_VCTR11_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr11;
#[doc = "MPCBB1_VCTR12 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR12_SPEC>`"]
pub type MPCBB1_VCTR12 = crate::Reg<mpcbb1_vctr12::MPCBB1_VCTR12_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr12;
#[doc = "MPCBB1_VCTR13 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR13_SPEC>`"]
pub type MPCBB1_VCTR13 = crate::Reg<mpcbb1_vctr13::MPCBB1_VCTR13_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr13;
#[doc = "MPCBB1_VCTR14 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR14_SPEC>`"]
pub type MPCBB1_VCTR14 = crate::Reg<mpcbb1_vctr14::MPCBB1_VCTR14_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr14;
#[doc = "MPCBB1_VCTR15 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR15_SPEC>`"]
pub type MPCBB1_VCTR15 = crate::Reg<mpcbb1_vctr15::MPCBB1_VCTR15_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr15;
#[doc = "MPCBB1_VCTR16 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR16_SPEC>`"]
pub type MPCBB1_VCTR16 = crate::Reg<mpcbb1_vctr16::MPCBB1_VCTR16_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr16;
#[doc = "MPCBB1_VCTR17 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR17_SPEC>`"]
pub type MPCBB1_VCTR17 = crate::Reg<mpcbb1_vctr17::MPCBB1_VCTR17_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr17;
#[doc = "MPCBB1_VCTR18 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR18_SPEC>`"]
pub type MPCBB1_VCTR18 = crate::Reg<mpcbb1_vctr18::MPCBB1_VCTR18_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr18;
#[doc = "MPCBB1_VCTR19 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR19_SPEC>`"]
pub type MPCBB1_VCTR19 = crate::Reg<mpcbb1_vctr19::MPCBB1_VCTR19_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr19;
#[doc = "MPCBB1_VCTR20 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR20_SPEC>`"]
pub type MPCBB1_VCTR20 = crate::Reg<mpcbb1_vctr20::MPCBB1_VCTR20_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr20;
#[doc = "MPCBB1_VCTR21 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR21_SPEC>`"]
pub type MPCBB1_VCTR21 = crate::Reg<mpcbb1_vctr21::MPCBB1_VCTR21_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr21;
#[doc = "MPCBB1_VCTR22 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR22_SPEC>`"]
pub type MPCBB1_VCTR22 = crate::Reg<mpcbb1_vctr22::MPCBB1_VCTR22_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr22;
#[doc = "MPCBB1_VCTR23 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR23_SPEC>`"]
pub type MPCBB1_VCTR23 = crate::Reg<mpcbb1_vctr23::MPCBB1_VCTR23_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr23;
#[doc = "MPCBB1_VCTR24 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR24_SPEC>`"]
pub type MPCBB1_VCTR24 = crate::Reg<mpcbb1_vctr24::MPCBB1_VCTR24_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr24;
#[doc = "MPCBB1_VCTR25 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR25_SPEC>`"]
pub type MPCBB1_VCTR25 = crate::Reg<mpcbb1_vctr25::MPCBB1_VCTR25_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr25;
#[doc = "MPCBB1_VCTR26 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR26_SPEC>`"]
pub type MPCBB1_VCTR26 = crate::Reg<mpcbb1_vctr26::MPCBB1_VCTR26_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr26;
#[doc = "MPCBB1_VCTR27 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR27_SPEC>`"]
pub type MPCBB1_VCTR27 = crate::Reg<mpcbb1_vctr27::MPCBB1_VCTR27_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr27;
#[doc = "MPCBB1_VCTR28 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR28_SPEC>`"]
pub type MPCBB1_VCTR28 = crate::Reg<mpcbb1_vctr28::MPCBB1_VCTR28_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr28;
#[doc = "MPCBB1_VCTR29 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR29_SPEC>`"]
pub type MPCBB1_VCTR29 = crate::Reg<mpcbb1_vctr29::MPCBB1_VCTR29_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr29;
#[doc = "MPCBB1_VCTR30 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR30_SPEC>`"]
pub type MPCBB1_VCTR30 = crate::Reg<mpcbb1_vctr30::MPCBB1_VCTR30_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr30;
#[doc = "MPCBB1_VCTR31 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR31_SPEC>`"]
pub type MPCBB1_VCTR31 = crate::Reg<mpcbb1_vctr31::MPCBB1_VCTR31_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr31;
#[doc = "MPCBB1_VCTR32 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR32_SPEC>`"]
pub type MPCBB1_VCTR32 = crate::Reg<mpcbb1_vctr32::MPCBB1_VCTR32_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr32;
#[doc = "MPCBB1_VCTR33 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR33_SPEC>`"]
pub type MPCBB1_VCTR33 = crate::Reg<mpcbb1_vctr33::MPCBB1_VCTR33_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr33;
#[doc = "MPCBB1_VCTR34 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR34_SPEC>`"]
pub type MPCBB1_VCTR34 = crate::Reg<mpcbb1_vctr34::MPCBB1_VCTR34_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr34;
#[doc = "MPCBB1_VCTR35 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR35_SPEC>`"]
pub type MPCBB1_VCTR35 = crate::Reg<mpcbb1_vctr35::MPCBB1_VCTR35_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr35;
#[doc = "MPCBB1_VCTR36 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR36_SPEC>`"]
pub type MPCBB1_VCTR36 = crate::Reg<mpcbb1_vctr36::MPCBB1_VCTR36_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr36;
#[doc = "MPCBB1_VCTR37 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR37_SPEC>`"]
pub type MPCBB1_VCTR37 = crate::Reg<mpcbb1_vctr37::MPCBB1_VCTR37_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr37;
#[doc = "MPCBB1_VCTR38 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR38_SPEC>`"]
pub type MPCBB1_VCTR38 = crate::Reg<mpcbb1_vctr38::MPCBB1_VCTR38_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr38;
#[doc = "MPCBB1_VCTR39 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR39_SPEC>`"]
pub type MPCBB1_VCTR39 = crate::Reg<mpcbb1_vctr39::MPCBB1_VCTR39_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr39;
#[doc = "MPCBB1_VCTR40 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR40_SPEC>`"]
pub type MPCBB1_VCTR40 = crate::Reg<mpcbb1_vctr40::MPCBB1_VCTR40_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr40;
#[doc = "MPCBB1_VCTR41 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR41_SPEC>`"]
pub type MPCBB1_VCTR41 = crate::Reg<mpcbb1_vctr41::MPCBB1_VCTR41_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr41;
#[doc = "MPCBB1_VCTR42 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR42_SPEC>`"]
pub type MPCBB1_VCTR42 = crate::Reg<mpcbb1_vctr42::MPCBB1_VCTR42_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr42;
#[doc = "MPCBB1_VCTR43 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR43_SPEC>`"]
pub type MPCBB1_VCTR43 = crate::Reg<mpcbb1_vctr43::MPCBB1_VCTR43_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr43;
#[doc = "MPCBB1_VCTR44 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR44_SPEC>`"]
pub type MPCBB1_VCTR44 = crate::Reg<mpcbb1_vctr44::MPCBB1_VCTR44_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr44;
#[doc = "MPCBB1_VCTR45 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR45_SPEC>`"]
pub type MPCBB1_VCTR45 = crate::Reg<mpcbb1_vctr45::MPCBB1_VCTR45_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr45;
#[doc = "MPCBB1_VCTR46 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR46_SPEC>`"]
pub type MPCBB1_VCTR46 = crate::Reg<mpcbb1_vctr46::MPCBB1_VCTR46_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr46;
#[doc = "MPCBB1_VCTR47 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR47_SPEC>`"]
pub type MPCBB1_VCTR47 = crate::Reg<mpcbb1_vctr47::MPCBB1_VCTR47_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr47;
#[doc = "MPCBB1_VCTR48 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR48_SPEC>`"]
pub type MPCBB1_VCTR48 = crate::Reg<mpcbb1_vctr48::MPCBB1_VCTR48_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr48;
#[doc = "MPCBB1_VCTR49 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR49_SPEC>`"]
pub type MPCBB1_VCTR49 = crate::Reg<mpcbb1_vctr49::MPCBB1_VCTR49_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr49;
#[doc = "MPCBB1_VCTR50 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR50_SPEC>`"]
pub type MPCBB1_VCTR50 = crate::Reg<mpcbb1_vctr50::MPCBB1_VCTR50_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr50;
#[doc = "MPCBB1_VCTR51 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR51_SPEC>`"]
pub type MPCBB1_VCTR51 = crate::Reg<mpcbb1_vctr51::MPCBB1_VCTR51_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr51;
#[doc = "MPCBB1_VCTR52 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR52_SPEC>`"]
pub type MPCBB1_VCTR52 = crate::Reg<mpcbb1_vctr52::MPCBB1_VCTR52_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr52;
#[doc = "MPCBB1_VCTR53 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR53_SPEC>`"]
pub type MPCBB1_VCTR53 = crate::Reg<mpcbb1_vctr53::MPCBB1_VCTR53_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr53;
#[doc = "MPCBB1_VCTR54 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR54_SPEC>`"]
pub type MPCBB1_VCTR54 = crate::Reg<mpcbb1_vctr54::MPCBB1_VCTR54_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr54;
#[doc = "MPCBB1_VCTR55 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR55_SPEC>`"]
pub type MPCBB1_VCTR55 = crate::Reg<mpcbb1_vctr55::MPCBB1_VCTR55_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr55;
#[doc = "MPCBB1_VCTR56 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR56_SPEC>`"]
pub type MPCBB1_VCTR56 = crate::Reg<mpcbb1_vctr56::MPCBB1_VCTR56_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr56;
#[doc = "MPCBB1_VCTR57 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR57_SPEC>`"]
pub type MPCBB1_VCTR57 = crate::Reg<mpcbb1_vctr57::MPCBB1_VCTR57_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr57;
#[doc = "MPCBB1_VCTR58 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR58_SPEC>`"]
pub type MPCBB1_VCTR58 = crate::Reg<mpcbb1_vctr58::MPCBB1_VCTR58_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr58;
#[doc = "MPCBB1_VCTR59 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR59_SPEC>`"]
pub type MPCBB1_VCTR59 = crate::Reg<mpcbb1_vctr59::MPCBB1_VCTR59_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr59;
#[doc = "MPCBB1_VCTR60 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR60_SPEC>`"]
pub type MPCBB1_VCTR60 = crate::Reg<mpcbb1_vctr60::MPCBB1_VCTR60_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr60;
#[doc = "MPCBB1_VCTR61 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR61_SPEC>`"]
pub type MPCBB1_VCTR61 = crate::Reg<mpcbb1_vctr61::MPCBB1_VCTR61_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr61;
#[doc = "MPCBB1_VCTR62 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR62_SPEC>`"]
pub type MPCBB1_VCTR62 = crate::Reg<mpcbb1_vctr62::MPCBB1_VCTR62_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr62;
#[doc = "MPCBB1_VCTR63 (rw) register accessor: an alias for `Reg<MPCBB1_VCTR63_SPEC>`"]
pub type MPCBB1_VCTR63 = crate::Reg<mpcbb1_vctr63::MPCBB1_VCTR63_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr63;
