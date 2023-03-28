#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HASH control register"]
    pub hash_cr: HASH_CR,
    #[doc = "0x04 - HASH data input register"]
    pub hash_din: HASH_DIN,
    #[doc = "0x08 - HASH start register"]
    pub hash_str: HASH_STR,
    #[doc = "0x0c - HASH aliased digest register 0"]
    pub hash_hra0: HASH_HRA0,
    #[doc = "0x10 - HASH aliased digest register 1"]
    pub hash_hra1: HASH_HRA1,
    #[doc = "0x14 - HASH aliased digest register 2"]
    pub hash_hra2: HASH_HRA2,
    #[doc = "0x18 - HASH aliased digest register 3"]
    pub hash_hra3: HASH_HRA3,
    #[doc = "0x1c - HASH aliased digest register 4"]
    pub hash_hra4: HASH_HRA4,
    #[doc = "0x20 - HASH interrupt enable register"]
    pub hash_imr: HASH_IMR,
    #[doc = "0x24 - HASH status register"]
    pub hash_sr: HASH_SR,
    _reserved10: [u8; 0xd0],
    #[doc = "0xf8 - HASH context swap register 0"]
    pub hash_csr0: HASH_CSR0,
    #[doc = "0xfc - HASH context swap register 1"]
    pub hash_csr1: HASH_CSR1,
    #[doc = "0x100 - HASH context swap register 2"]
    pub hash_csr2: HASH_CSR2,
    #[doc = "0x104 - HASH context swap register 3"]
    pub hash_csr3: HASH_CSR3,
    #[doc = "0x108 - HASH context swap register 4"]
    pub hash_csr4: HASH_CSR4,
    #[doc = "0x10c - HASH context swap register 5"]
    pub hash_csr5: HASH_CSR5,
    #[doc = "0x110 - HASH context swap register 6"]
    pub hash_csr6: HASH_CSR6,
    #[doc = "0x114 - HASH context swap register 7"]
    pub hash_csr7: HASH_CSR7,
    #[doc = "0x118 - HASH context swap register 8"]
    pub hash_csr8: HASH_CSR8,
    #[doc = "0x11c - HASH context swap register 9"]
    pub hash_csr9: HASH_CSR9,
    #[doc = "0x120 - HASH context swap register 10"]
    pub hash_csr10: HASH_CSR10,
    #[doc = "0x124 - HASH context swap register 11"]
    pub hash_csr11: HASH_CSR11,
    #[doc = "0x128 - HASH context swap register 12"]
    pub hash_csr12: HASH_CSR12,
    #[doc = "0x12c - HASH context swap register 13"]
    pub hash_csr13: HASH_CSR13,
    #[doc = "0x130 - HASH context swap register 14"]
    pub hash_csr14: HASH_CSR14,
    #[doc = "0x134 - HASH context swap register 15"]
    pub hash_csr15: HASH_CSR15,
    #[doc = "0x138 - HASH context swap register 16"]
    pub hash_csr16: HASH_CSR16,
    #[doc = "0x13c - HASH context swap register 17"]
    pub hash_csr17: HASH_CSR17,
    #[doc = "0x140 - HASH context swap register 18"]
    pub hash_csr18: HASH_CSR18,
    #[doc = "0x144 - HASH context swap register 19"]
    pub hash_csr19: HASH_CSR19,
    #[doc = "0x148 - HASH context swap register 20"]
    pub hash_csr20: HASH_CSR20,
    #[doc = "0x14c - HASH context swap register 21"]
    pub hash_csr21: HASH_CSR21,
    #[doc = "0x150 - HASH context swap register 22"]
    pub hash_csr22: HASH_CSR22,
    #[doc = "0x154 - HASH context swap register 23"]
    pub hash_csr23: HASH_CSR23,
    #[doc = "0x158 - HASH context swap register 24"]
    pub hash_csr24: HASH_CSR24,
    #[doc = "0x15c - HASH context swap register 25"]
    pub hash_csr25: HASH_CSR25,
    #[doc = "0x160 - HASH context swap register 26"]
    pub hash_csr26: HASH_CSR26,
    #[doc = "0x164 - HASH context swap register 27"]
    pub hash_csr27: HASH_CSR27,
    #[doc = "0x168 - HASH context swap register 28"]
    pub hash_csr28: HASH_CSR28,
    #[doc = "0x16c - HASH context swap register 29"]
    pub hash_csr29: HASH_CSR29,
    #[doc = "0x170 - HASH context swap register 30"]
    pub hash_csr30: HASH_CSR30,
    #[doc = "0x174 - HASH context swap register 31"]
    pub hash_csr31: HASH_CSR31,
    #[doc = "0x178 - HASH context swap register 32"]
    pub hash_csr32: HASH_CSR32,
    #[doc = "0x17c - HASH context swap register 33"]
    pub hash_csr33: HASH_CSR33,
    #[doc = "0x180 - HASH context swap register 34"]
    pub hash_csr34: HASH_CSR34,
    #[doc = "0x184 - HASH context swap register 35"]
    pub hash_csr35: HASH_CSR35,
    #[doc = "0x188 - HASH context swap register 36"]
    pub hash_csr36: HASH_CSR36,
    #[doc = "0x18c - HASH context swap register 37"]
    pub hash_csr37: HASH_CSR37,
    #[doc = "0x190 - HASH context swap register 38"]
    pub hash_csr38: HASH_CSR38,
    #[doc = "0x194 - HASH context swap register 39"]
    pub hash_csr39: HASH_CSR39,
    #[doc = "0x198 - HASH context swap register 40"]
    pub hash_csr40: HASH_CSR40,
    #[doc = "0x19c - HASH context swap register 41"]
    pub hash_csr41: HASH_CSR41,
    #[doc = "0x1a0 - HASH context swap register 42"]
    pub hash_csr42: HASH_CSR42,
    #[doc = "0x1a4 - HASH context swap register 43"]
    pub hash_csr43: HASH_CSR43,
    #[doc = "0x1a8 - HASH context swap register 44"]
    pub hash_csr44: HASH_CSR44,
    #[doc = "0x1ac - HASH context swap register 45"]
    pub hash_csr45: HASH_CSR45,
    #[doc = "0x1b0 - HASH context swap register 46"]
    pub hash_csr46: HASH_CSR46,
    #[doc = "0x1b4 - HASH context swap register 47"]
    pub hash_csr47: HASH_CSR47,
    #[doc = "0x1b8 - HASH context swap register 48"]
    pub hash_csr48: HASH_CSR48,
    #[doc = "0x1bc - HASH context swap register 49"]
    pub hash_csr49: HASH_CSR49,
    #[doc = "0x1c0 - HASH context swap register 50"]
    pub hash_csr50: HASH_CSR50,
    #[doc = "0x1c4 - HASH context swap register 51"]
    pub hash_csr51: HASH_CSR51,
    #[doc = "0x1c8 - HASH context swap register 52"]
    pub hash_csr52: HASH_CSR52,
    #[doc = "0x1cc - HASH context swap register 53"]
    pub hash_csr53: HASH_CSR53,
    #[doc = "0x1d0 - HASH context swap register 54"]
    pub hash_csr54: HASH_CSR54,
    #[doc = "0x1d4 - HASH context swap register 55"]
    pub hash_csr55: HASH_CSR55,
    #[doc = "0x1d8 - HASH context swap register 56"]
    pub hash_csr56: HASH_CSR56,
    #[doc = "0x1dc - HASH context swap register 57"]
    pub hash_csr57: HASH_CSR57,
    #[doc = "0x1e0 - HASH context swap register 58"]
    pub hash_csr58: HASH_CSR58,
    #[doc = "0x1e4 - HASH context swap register 59"]
    pub hash_csr59: HASH_CSR59,
    #[doc = "0x1e8 - HASH context swap register 60"]
    pub hash_csr60: HASH_CSR60,
    #[doc = "0x1ec - HASH context swap register 61"]
    pub hash_csr61: HASH_CSR61,
    #[doc = "0x1f0 - HASH context swap register 62"]
    pub hash_csr62: HASH_CSR62,
    #[doc = "0x1f4 - HASH context swap register 63"]
    pub hash_csr63: HASH_CSR63,
    #[doc = "0x1f8 - HASH context swap register 64"]
    pub hash_csr64: HASH_CSR64,
    #[doc = "0x1fc - HASH context swap register 65"]
    pub hash_csr65: HASH_CSR65,
    #[doc = "0x200 - HASH context swap register 66"]
    pub hash_csr66: HASH_CSR66,
    #[doc = "0x204 - HASH context swap register 67"]
    pub hash_csr67: HASH_CSR67,
    #[doc = "0x208 - HASH context swap register 68"]
    pub hash_csr68: HASH_CSR68,
    #[doc = "0x20c - HASH context swap register 69"]
    pub hash_csr69: HASH_CSR69,
    #[doc = "0x210 - HASH context swap register 70"]
    pub hash_csr70: HASH_CSR70,
    #[doc = "0x214 - HASH context swap register 71"]
    pub hash_csr71: HASH_CSR71,
    #[doc = "0x218 - HASH context swap register 72"]
    pub hash_csr72: HASH_CSR72,
    #[doc = "0x21c - HASH context swap register 73"]
    pub hash_csr73: HASH_CSR73,
    #[doc = "0x220 - HASH context swap register 74"]
    pub hash_csr74: HASH_CSR74,
    #[doc = "0x224 - HASH context swap register 75"]
    pub hash_csr75: HASH_CSR75,
    #[doc = "0x228 - HASH context swap register 76"]
    pub hash_csr76: HASH_CSR76,
    #[doc = "0x22c - HASH context swap register 77"]
    pub hash_csr77: HASH_CSR77,
    #[doc = "0x230 - HASH context swap register 78"]
    pub hash_csr78: HASH_CSR78,
    #[doc = "0x234 - HASH context swap register 79"]
    pub hash_csr79: HASH_CSR79,
    #[doc = "0x238 - HASH context swap register 80"]
    pub hash_csr80: HASH_CSR80,
    #[doc = "0x23c - HASH context swap register 81"]
    pub hash_csr81: HASH_CSR81,
    #[doc = "0x240 - HASH context swap register 82"]
    pub hash_csr82: HASH_CSR82,
    #[doc = "0x244 - HASH context swap register 83"]
    pub hash_csr83: HASH_CSR83,
    #[doc = "0x248 - HASH context swap register 84"]
    pub hash_csr84: HASH_CSR84,
    #[doc = "0x24c - HASH context swap register 85"]
    pub hash_csr85: HASH_CSR85,
    #[doc = "0x250 - HASH context swap register 86"]
    pub hash_csr86: HASH_CSR86,
    #[doc = "0x254 - HASH context swap register 87"]
    pub hash_csr87: HASH_CSR87,
    #[doc = "0x258 - HASH context swap register 88"]
    pub hash_csr88: HASH_CSR88,
    #[doc = "0x25c - HASH context swap register 89"]
    pub hash_csr89: HASH_CSR89,
    #[doc = "0x260 - HASH context swap register 90"]
    pub hash_csr90: HASH_CSR90,
    #[doc = "0x264 - HASH context swap register 91"]
    pub hash_csr91: HASH_CSR91,
    #[doc = "0x268 - HASH context swap register 92"]
    pub hash_csr92: HASH_CSR92,
    #[doc = "0x26c - HASH context swap register 93"]
    pub hash_csr93: HASH_CSR93,
    #[doc = "0x270 - HASH context swap register 94"]
    pub hash_csr94: HASH_CSR94,
    #[doc = "0x274 - HASH context swap register 95"]
    pub hash_csr95: HASH_CSR95,
    #[doc = "0x278 - HASH context swap register 96"]
    pub hash_csr96: HASH_CSR96,
    #[doc = "0x27c - HASH context swap register 97"]
    pub hash_csr97: HASH_CSR97,
    #[doc = "0x280 - HASH context swap register 98"]
    pub hash_csr98: HASH_CSR98,
    #[doc = "0x284 - HASH context swap register 99"]
    pub hash_csr99: HASH_CSR99,
    #[doc = "0x288 - HASH context swap register 100"]
    pub hash_csr100: HASH_CSR100,
    #[doc = "0x28c - HASH context swap register 101"]
    pub hash_csr101: HASH_CSR101,
    #[doc = "0x290 - HASH context swap register 102"]
    pub hash_csr102: HASH_CSR102,
    _reserved113: [u8; 0x7c],
    #[doc = "0x310 - HASH digest register"]
    pub hash_hr0: HASH_HR0,
    #[doc = "0x314 - HASH digest register"]
    pub hash_hr1: HASH_HR1,
    #[doc = "0x318 - HASH digest register"]
    pub hash_hr2: HASH_HR2,
    #[doc = "0x31c - HASH digest register"]
    pub hash_hr3: HASH_HR3,
    #[doc = "0x320 - HASH digest register"]
    pub hash_hr4: HASH_HR4,
    #[doc = "0x324 - HASH digest register"]
    pub hash_hr5: HASH_HR5,
    #[doc = "0x328 - HASH digest register"]
    pub hash_hr6: HASH_HR6,
    #[doc = "0x32c - HASH digest register"]
    pub hash_hr7: HASH_HR7,
    #[doc = "0x330 - HASH digest register"]
    pub hash_hr8: HASH_HR8,
    #[doc = "0x334 - HASH digest register"]
    pub hash_hr9: HASH_HR9,
    #[doc = "0x338 - HASH digest register"]
    pub hash_hr10: HASH_HR10,
    #[doc = "0x33c - HASH digest register"]
    pub hash_hr11: HASH_HR11,
    #[doc = "0x340 - HASH digest register"]
    pub hash_hr12: HASH_HR12,
    #[doc = "0x344 - HASH digest register"]
    pub hash_hr13: HASH_HR13,
    #[doc = "0x348 - HASH digest register"]
    pub hash_hr14: HASH_HR14,
    #[doc = "0x34c - HASH digest register"]
    pub hash_hr15: HASH_HR15,
}
#[doc = "HASH_CR (rw) register accessor: an alias for `Reg<HASH_CR_SPEC>`"]
pub type HASH_CR = crate::Reg<hash_cr::HASH_CR_SPEC>;
#[doc = "HASH control register"]
pub mod hash_cr;
#[doc = "HASH_DIN (w) register accessor: an alias for `Reg<HASH_DIN_SPEC>`"]
pub type HASH_DIN = crate::Reg<hash_din::HASH_DIN_SPEC>;
#[doc = "HASH data input register"]
pub mod hash_din;
#[doc = "HASH_STR (rw) register accessor: an alias for `Reg<HASH_STR_SPEC>`"]
pub type HASH_STR = crate::Reg<hash_str::HASH_STR_SPEC>;
#[doc = "HASH start register"]
pub mod hash_str;
#[doc = "HASH_HRA0 (r) register accessor: an alias for `Reg<HASH_HRA0_SPEC>`"]
pub type HASH_HRA0 = crate::Reg<hash_hra0::HASH_HRA0_SPEC>;
#[doc = "HASH aliased digest register 0"]
pub mod hash_hra0;
#[doc = "HASH_HRA1 (r) register accessor: an alias for `Reg<HASH_HRA1_SPEC>`"]
pub type HASH_HRA1 = crate::Reg<hash_hra1::HASH_HRA1_SPEC>;
#[doc = "HASH aliased digest register 1"]
pub mod hash_hra1;
#[doc = "HASH_HRA2 (r) register accessor: an alias for `Reg<HASH_HRA2_SPEC>`"]
pub type HASH_HRA2 = crate::Reg<hash_hra2::HASH_HRA2_SPEC>;
#[doc = "HASH aliased digest register 2"]
pub mod hash_hra2;
#[doc = "HASH_HRA3 (r) register accessor: an alias for `Reg<HASH_HRA3_SPEC>`"]
pub type HASH_HRA3 = crate::Reg<hash_hra3::HASH_HRA3_SPEC>;
#[doc = "HASH aliased digest register 3"]
pub mod hash_hra3;
#[doc = "HASH_HRA4 (r) register accessor: an alias for `Reg<HASH_HRA4_SPEC>`"]
pub type HASH_HRA4 = crate::Reg<hash_hra4::HASH_HRA4_SPEC>;
#[doc = "HASH aliased digest register 4"]
pub mod hash_hra4;
#[doc = "HASH_IMR (rw) register accessor: an alias for `Reg<HASH_IMR_SPEC>`"]
pub type HASH_IMR = crate::Reg<hash_imr::HASH_IMR_SPEC>;
#[doc = "HASH interrupt enable register"]
pub mod hash_imr;
#[doc = "HASH_SR (rw) register accessor: an alias for `Reg<HASH_SR_SPEC>`"]
pub type HASH_SR = crate::Reg<hash_sr::HASH_SR_SPEC>;
#[doc = "HASH status register"]
pub mod hash_sr;
#[doc = "HASH_CSR0 (rw) register accessor: an alias for `Reg<HASH_CSR0_SPEC>`"]
pub type HASH_CSR0 = crate::Reg<hash_csr0::HASH_CSR0_SPEC>;
#[doc = "HASH context swap register 0"]
pub mod hash_csr0;
#[doc = "HASH_CSR1 (rw) register accessor: an alias for `Reg<HASH_CSR1_SPEC>`"]
pub type HASH_CSR1 = crate::Reg<hash_csr1::HASH_CSR1_SPEC>;
#[doc = "HASH context swap register 1"]
pub mod hash_csr1;
#[doc = "HASH_CSR2 (rw) register accessor: an alias for `Reg<HASH_CSR2_SPEC>`"]
pub type HASH_CSR2 = crate::Reg<hash_csr2::HASH_CSR2_SPEC>;
#[doc = "HASH context swap register 2"]
pub mod hash_csr2;
#[doc = "HASH_CSR3 (rw) register accessor: an alias for `Reg<HASH_CSR3_SPEC>`"]
pub type HASH_CSR3 = crate::Reg<hash_csr3::HASH_CSR3_SPEC>;
#[doc = "HASH context swap register 3"]
pub mod hash_csr3;
#[doc = "HASH_CSR4 (rw) register accessor: an alias for `Reg<HASH_CSR4_SPEC>`"]
pub type HASH_CSR4 = crate::Reg<hash_csr4::HASH_CSR4_SPEC>;
#[doc = "HASH context swap register 4"]
pub mod hash_csr4;
#[doc = "HASH_CSR5 (rw) register accessor: an alias for `Reg<HASH_CSR5_SPEC>`"]
pub type HASH_CSR5 = crate::Reg<hash_csr5::HASH_CSR5_SPEC>;
#[doc = "HASH context swap register 5"]
pub mod hash_csr5;
#[doc = "HASH_CSR6 (rw) register accessor: an alias for `Reg<HASH_CSR6_SPEC>`"]
pub type HASH_CSR6 = crate::Reg<hash_csr6::HASH_CSR6_SPEC>;
#[doc = "HASH context swap register 6"]
pub mod hash_csr6;
#[doc = "HASH_CSR7 (rw) register accessor: an alias for `Reg<HASH_CSR7_SPEC>`"]
pub type HASH_CSR7 = crate::Reg<hash_csr7::HASH_CSR7_SPEC>;
#[doc = "HASH context swap register 7"]
pub mod hash_csr7;
#[doc = "HASH_CSR8 (rw) register accessor: an alias for `Reg<HASH_CSR8_SPEC>`"]
pub type HASH_CSR8 = crate::Reg<hash_csr8::HASH_CSR8_SPEC>;
#[doc = "HASH context swap register 8"]
pub mod hash_csr8;
#[doc = "HASH_CSR9 (rw) register accessor: an alias for `Reg<HASH_CSR9_SPEC>`"]
pub type HASH_CSR9 = crate::Reg<hash_csr9::HASH_CSR9_SPEC>;
#[doc = "HASH context swap register 9"]
pub mod hash_csr9;
#[doc = "HASH_CSR10 (rw) register accessor: an alias for `Reg<HASH_CSR10_SPEC>`"]
pub type HASH_CSR10 = crate::Reg<hash_csr10::HASH_CSR10_SPEC>;
#[doc = "HASH context swap register 10"]
pub mod hash_csr10;
#[doc = "HASH_CSR11 (rw) register accessor: an alias for `Reg<HASH_CSR11_SPEC>`"]
pub type HASH_CSR11 = crate::Reg<hash_csr11::HASH_CSR11_SPEC>;
#[doc = "HASH context swap register 11"]
pub mod hash_csr11;
#[doc = "HASH_CSR12 (rw) register accessor: an alias for `Reg<HASH_CSR12_SPEC>`"]
pub type HASH_CSR12 = crate::Reg<hash_csr12::HASH_CSR12_SPEC>;
#[doc = "HASH context swap register 12"]
pub mod hash_csr12;
#[doc = "HASH_CSR13 (rw) register accessor: an alias for `Reg<HASH_CSR13_SPEC>`"]
pub type HASH_CSR13 = crate::Reg<hash_csr13::HASH_CSR13_SPEC>;
#[doc = "HASH context swap register 13"]
pub mod hash_csr13;
#[doc = "HASH_CSR14 (rw) register accessor: an alias for `Reg<HASH_CSR14_SPEC>`"]
pub type HASH_CSR14 = crate::Reg<hash_csr14::HASH_CSR14_SPEC>;
#[doc = "HASH context swap register 14"]
pub mod hash_csr14;
#[doc = "HASH_CSR15 (rw) register accessor: an alias for `Reg<HASH_CSR15_SPEC>`"]
pub type HASH_CSR15 = crate::Reg<hash_csr15::HASH_CSR15_SPEC>;
#[doc = "HASH context swap register 15"]
pub mod hash_csr15;
#[doc = "HASH_CSR16 (rw) register accessor: an alias for `Reg<HASH_CSR16_SPEC>`"]
pub type HASH_CSR16 = crate::Reg<hash_csr16::HASH_CSR16_SPEC>;
#[doc = "HASH context swap register 16"]
pub mod hash_csr16;
#[doc = "HASH_CSR17 (rw) register accessor: an alias for `Reg<HASH_CSR17_SPEC>`"]
pub type HASH_CSR17 = crate::Reg<hash_csr17::HASH_CSR17_SPEC>;
#[doc = "HASH context swap register 17"]
pub mod hash_csr17;
#[doc = "HASH_CSR18 (rw) register accessor: an alias for `Reg<HASH_CSR18_SPEC>`"]
pub type HASH_CSR18 = crate::Reg<hash_csr18::HASH_CSR18_SPEC>;
#[doc = "HASH context swap register 18"]
pub mod hash_csr18;
#[doc = "HASH_CSR19 (rw) register accessor: an alias for `Reg<HASH_CSR19_SPEC>`"]
pub type HASH_CSR19 = crate::Reg<hash_csr19::HASH_CSR19_SPEC>;
#[doc = "HASH context swap register 19"]
pub mod hash_csr19;
#[doc = "HASH_CSR20 (rw) register accessor: an alias for `Reg<HASH_CSR20_SPEC>`"]
pub type HASH_CSR20 = crate::Reg<hash_csr20::HASH_CSR20_SPEC>;
#[doc = "HASH context swap register 20"]
pub mod hash_csr20;
#[doc = "HASH_CSR21 (rw) register accessor: an alias for `Reg<HASH_CSR21_SPEC>`"]
pub type HASH_CSR21 = crate::Reg<hash_csr21::HASH_CSR21_SPEC>;
#[doc = "HASH context swap register 21"]
pub mod hash_csr21;
#[doc = "HASH_CSR22 (rw) register accessor: an alias for `Reg<HASH_CSR22_SPEC>`"]
pub type HASH_CSR22 = crate::Reg<hash_csr22::HASH_CSR22_SPEC>;
#[doc = "HASH context swap register 22"]
pub mod hash_csr22;
#[doc = "HASH_CSR23 (rw) register accessor: an alias for `Reg<HASH_CSR23_SPEC>`"]
pub type HASH_CSR23 = crate::Reg<hash_csr23::HASH_CSR23_SPEC>;
#[doc = "HASH context swap register 23"]
pub mod hash_csr23;
#[doc = "HASH_CSR24 (rw) register accessor: an alias for `Reg<HASH_CSR24_SPEC>`"]
pub type HASH_CSR24 = crate::Reg<hash_csr24::HASH_CSR24_SPEC>;
#[doc = "HASH context swap register 24"]
pub mod hash_csr24;
#[doc = "HASH_CSR25 (rw) register accessor: an alias for `Reg<HASH_CSR25_SPEC>`"]
pub type HASH_CSR25 = crate::Reg<hash_csr25::HASH_CSR25_SPEC>;
#[doc = "HASH context swap register 25"]
pub mod hash_csr25;
#[doc = "HASH_CSR26 (rw) register accessor: an alias for `Reg<HASH_CSR26_SPEC>`"]
pub type HASH_CSR26 = crate::Reg<hash_csr26::HASH_CSR26_SPEC>;
#[doc = "HASH context swap register 26"]
pub mod hash_csr26;
#[doc = "HASH_CSR27 (rw) register accessor: an alias for `Reg<HASH_CSR27_SPEC>`"]
pub type HASH_CSR27 = crate::Reg<hash_csr27::HASH_CSR27_SPEC>;
#[doc = "HASH context swap register 27"]
pub mod hash_csr27;
#[doc = "HASH_CSR28 (rw) register accessor: an alias for `Reg<HASH_CSR28_SPEC>`"]
pub type HASH_CSR28 = crate::Reg<hash_csr28::HASH_CSR28_SPEC>;
#[doc = "HASH context swap register 28"]
pub mod hash_csr28;
#[doc = "HASH_CSR29 (rw) register accessor: an alias for `Reg<HASH_CSR29_SPEC>`"]
pub type HASH_CSR29 = crate::Reg<hash_csr29::HASH_CSR29_SPEC>;
#[doc = "HASH context swap register 29"]
pub mod hash_csr29;
#[doc = "HASH_CSR30 (rw) register accessor: an alias for `Reg<HASH_CSR30_SPEC>`"]
pub type HASH_CSR30 = crate::Reg<hash_csr30::HASH_CSR30_SPEC>;
#[doc = "HASH context swap register 30"]
pub mod hash_csr30;
#[doc = "HASH_CSR31 (rw) register accessor: an alias for `Reg<HASH_CSR31_SPEC>`"]
pub type HASH_CSR31 = crate::Reg<hash_csr31::HASH_CSR31_SPEC>;
#[doc = "HASH context swap register 31"]
pub mod hash_csr31;
#[doc = "HASH_CSR32 (rw) register accessor: an alias for `Reg<HASH_CSR32_SPEC>`"]
pub type HASH_CSR32 = crate::Reg<hash_csr32::HASH_CSR32_SPEC>;
#[doc = "HASH context swap register 32"]
pub mod hash_csr32;
#[doc = "HASH_CSR33 (rw) register accessor: an alias for `Reg<HASH_CSR33_SPEC>`"]
pub type HASH_CSR33 = crate::Reg<hash_csr33::HASH_CSR33_SPEC>;
#[doc = "HASH context swap register 33"]
pub mod hash_csr33;
#[doc = "HASH_CSR34 (rw) register accessor: an alias for `Reg<HASH_CSR34_SPEC>`"]
pub type HASH_CSR34 = crate::Reg<hash_csr34::HASH_CSR34_SPEC>;
#[doc = "HASH context swap register 34"]
pub mod hash_csr34;
#[doc = "HASH_CSR35 (rw) register accessor: an alias for `Reg<HASH_CSR35_SPEC>`"]
pub type HASH_CSR35 = crate::Reg<hash_csr35::HASH_CSR35_SPEC>;
#[doc = "HASH context swap register 35"]
pub mod hash_csr35;
#[doc = "HASH_CSR36 (rw) register accessor: an alias for `Reg<HASH_CSR36_SPEC>`"]
pub type HASH_CSR36 = crate::Reg<hash_csr36::HASH_CSR36_SPEC>;
#[doc = "HASH context swap register 36"]
pub mod hash_csr36;
#[doc = "HASH_CSR37 (rw) register accessor: an alias for `Reg<HASH_CSR37_SPEC>`"]
pub type HASH_CSR37 = crate::Reg<hash_csr37::HASH_CSR37_SPEC>;
#[doc = "HASH context swap register 37"]
pub mod hash_csr37;
#[doc = "HASH_CSR38 (rw) register accessor: an alias for `Reg<HASH_CSR38_SPEC>`"]
pub type HASH_CSR38 = crate::Reg<hash_csr38::HASH_CSR38_SPEC>;
#[doc = "HASH context swap register 38"]
pub mod hash_csr38;
#[doc = "HASH_CSR39 (rw) register accessor: an alias for `Reg<HASH_CSR39_SPEC>`"]
pub type HASH_CSR39 = crate::Reg<hash_csr39::HASH_CSR39_SPEC>;
#[doc = "HASH context swap register 39"]
pub mod hash_csr39;
#[doc = "HASH_CSR40 (rw) register accessor: an alias for `Reg<HASH_CSR40_SPEC>`"]
pub type HASH_CSR40 = crate::Reg<hash_csr40::HASH_CSR40_SPEC>;
#[doc = "HASH context swap register 40"]
pub mod hash_csr40;
#[doc = "HASH_CSR41 (rw) register accessor: an alias for `Reg<HASH_CSR41_SPEC>`"]
pub type HASH_CSR41 = crate::Reg<hash_csr41::HASH_CSR41_SPEC>;
#[doc = "HASH context swap register 41"]
pub mod hash_csr41;
#[doc = "HASH_CSR42 (rw) register accessor: an alias for `Reg<HASH_CSR42_SPEC>`"]
pub type HASH_CSR42 = crate::Reg<hash_csr42::HASH_CSR42_SPEC>;
#[doc = "HASH context swap register 42"]
pub mod hash_csr42;
#[doc = "HASH_CSR43 (rw) register accessor: an alias for `Reg<HASH_CSR43_SPEC>`"]
pub type HASH_CSR43 = crate::Reg<hash_csr43::HASH_CSR43_SPEC>;
#[doc = "HASH context swap register 43"]
pub mod hash_csr43;
#[doc = "HASH_CSR44 (rw) register accessor: an alias for `Reg<HASH_CSR44_SPEC>`"]
pub type HASH_CSR44 = crate::Reg<hash_csr44::HASH_CSR44_SPEC>;
#[doc = "HASH context swap register 44"]
pub mod hash_csr44;
#[doc = "HASH_CSR45 (rw) register accessor: an alias for `Reg<HASH_CSR45_SPEC>`"]
pub type HASH_CSR45 = crate::Reg<hash_csr45::HASH_CSR45_SPEC>;
#[doc = "HASH context swap register 45"]
pub mod hash_csr45;
#[doc = "HASH_CSR46 (rw) register accessor: an alias for `Reg<HASH_CSR46_SPEC>`"]
pub type HASH_CSR46 = crate::Reg<hash_csr46::HASH_CSR46_SPEC>;
#[doc = "HASH context swap register 46"]
pub mod hash_csr46;
#[doc = "HASH_CSR47 (rw) register accessor: an alias for `Reg<HASH_CSR47_SPEC>`"]
pub type HASH_CSR47 = crate::Reg<hash_csr47::HASH_CSR47_SPEC>;
#[doc = "HASH context swap register 47"]
pub mod hash_csr47;
#[doc = "HASH_CSR48 (rw) register accessor: an alias for `Reg<HASH_CSR48_SPEC>`"]
pub type HASH_CSR48 = crate::Reg<hash_csr48::HASH_CSR48_SPEC>;
#[doc = "HASH context swap register 48"]
pub mod hash_csr48;
#[doc = "HASH_CSR49 (rw) register accessor: an alias for `Reg<HASH_CSR49_SPEC>`"]
pub type HASH_CSR49 = crate::Reg<hash_csr49::HASH_CSR49_SPEC>;
#[doc = "HASH context swap register 49"]
pub mod hash_csr49;
#[doc = "HASH_CSR50 (rw) register accessor: an alias for `Reg<HASH_CSR50_SPEC>`"]
pub type HASH_CSR50 = crate::Reg<hash_csr50::HASH_CSR50_SPEC>;
#[doc = "HASH context swap register 50"]
pub mod hash_csr50;
#[doc = "HASH_CSR51 (rw) register accessor: an alias for `Reg<HASH_CSR51_SPEC>`"]
pub type HASH_CSR51 = crate::Reg<hash_csr51::HASH_CSR51_SPEC>;
#[doc = "HASH context swap register 51"]
pub mod hash_csr51;
#[doc = "HASH_CSR52 (rw) register accessor: an alias for `Reg<HASH_CSR52_SPEC>`"]
pub type HASH_CSR52 = crate::Reg<hash_csr52::HASH_CSR52_SPEC>;
#[doc = "HASH context swap register 52"]
pub mod hash_csr52;
#[doc = "HASH_CSR53 (rw) register accessor: an alias for `Reg<HASH_CSR53_SPEC>`"]
pub type HASH_CSR53 = crate::Reg<hash_csr53::HASH_CSR53_SPEC>;
#[doc = "HASH context swap register 53"]
pub mod hash_csr53;
#[doc = "HASH_CSR54 (rw) register accessor: an alias for `Reg<HASH_CSR54_SPEC>`"]
pub type HASH_CSR54 = crate::Reg<hash_csr54::HASH_CSR54_SPEC>;
#[doc = "HASH context swap register 54"]
pub mod hash_csr54;
#[doc = "HASH_CSR55 (rw) register accessor: an alias for `Reg<HASH_CSR55_SPEC>`"]
pub type HASH_CSR55 = crate::Reg<hash_csr55::HASH_CSR55_SPEC>;
#[doc = "HASH context swap register 55"]
pub mod hash_csr55;
#[doc = "HASH_CSR56 (rw) register accessor: an alias for `Reg<HASH_CSR56_SPEC>`"]
pub type HASH_CSR56 = crate::Reg<hash_csr56::HASH_CSR56_SPEC>;
#[doc = "HASH context swap register 56"]
pub mod hash_csr56;
#[doc = "HASH_CSR57 (rw) register accessor: an alias for `Reg<HASH_CSR57_SPEC>`"]
pub type HASH_CSR57 = crate::Reg<hash_csr57::HASH_CSR57_SPEC>;
#[doc = "HASH context swap register 57"]
pub mod hash_csr57;
#[doc = "HASH_CSR58 (rw) register accessor: an alias for `Reg<HASH_CSR58_SPEC>`"]
pub type HASH_CSR58 = crate::Reg<hash_csr58::HASH_CSR58_SPEC>;
#[doc = "HASH context swap register 58"]
pub mod hash_csr58;
#[doc = "HASH_CSR59 (rw) register accessor: an alias for `Reg<HASH_CSR59_SPEC>`"]
pub type HASH_CSR59 = crate::Reg<hash_csr59::HASH_CSR59_SPEC>;
#[doc = "HASH context swap register 59"]
pub mod hash_csr59;
#[doc = "HASH_CSR60 (rw) register accessor: an alias for `Reg<HASH_CSR60_SPEC>`"]
pub type HASH_CSR60 = crate::Reg<hash_csr60::HASH_CSR60_SPEC>;
#[doc = "HASH context swap register 60"]
pub mod hash_csr60;
#[doc = "HASH_CSR61 (rw) register accessor: an alias for `Reg<HASH_CSR61_SPEC>`"]
pub type HASH_CSR61 = crate::Reg<hash_csr61::HASH_CSR61_SPEC>;
#[doc = "HASH context swap register 61"]
pub mod hash_csr61;
#[doc = "HASH_CSR62 (rw) register accessor: an alias for `Reg<HASH_CSR62_SPEC>`"]
pub type HASH_CSR62 = crate::Reg<hash_csr62::HASH_CSR62_SPEC>;
#[doc = "HASH context swap register 62"]
pub mod hash_csr62;
#[doc = "HASH_CSR63 (rw) register accessor: an alias for `Reg<HASH_CSR63_SPEC>`"]
pub type HASH_CSR63 = crate::Reg<hash_csr63::HASH_CSR63_SPEC>;
#[doc = "HASH context swap register 63"]
pub mod hash_csr63;
#[doc = "HASH_CSR64 (rw) register accessor: an alias for `Reg<HASH_CSR64_SPEC>`"]
pub type HASH_CSR64 = crate::Reg<hash_csr64::HASH_CSR64_SPEC>;
#[doc = "HASH context swap register 64"]
pub mod hash_csr64;
#[doc = "HASH_CSR65 (rw) register accessor: an alias for `Reg<HASH_CSR65_SPEC>`"]
pub type HASH_CSR65 = crate::Reg<hash_csr65::HASH_CSR65_SPEC>;
#[doc = "HASH context swap register 65"]
pub mod hash_csr65;
#[doc = "HASH_CSR66 (rw) register accessor: an alias for `Reg<HASH_CSR66_SPEC>`"]
pub type HASH_CSR66 = crate::Reg<hash_csr66::HASH_CSR66_SPEC>;
#[doc = "HASH context swap register 66"]
pub mod hash_csr66;
#[doc = "HASH_CSR67 (rw) register accessor: an alias for `Reg<HASH_CSR67_SPEC>`"]
pub type HASH_CSR67 = crate::Reg<hash_csr67::HASH_CSR67_SPEC>;
#[doc = "HASH context swap register 67"]
pub mod hash_csr67;
#[doc = "HASH_CSR68 (rw) register accessor: an alias for `Reg<HASH_CSR68_SPEC>`"]
pub type HASH_CSR68 = crate::Reg<hash_csr68::HASH_CSR68_SPEC>;
#[doc = "HASH context swap register 68"]
pub mod hash_csr68;
#[doc = "HASH_CSR69 (rw) register accessor: an alias for `Reg<HASH_CSR69_SPEC>`"]
pub type HASH_CSR69 = crate::Reg<hash_csr69::HASH_CSR69_SPEC>;
#[doc = "HASH context swap register 69"]
pub mod hash_csr69;
#[doc = "HASH_CSR70 (rw) register accessor: an alias for `Reg<HASH_CSR70_SPEC>`"]
pub type HASH_CSR70 = crate::Reg<hash_csr70::HASH_CSR70_SPEC>;
#[doc = "HASH context swap register 70"]
pub mod hash_csr70;
#[doc = "HASH_CSR71 (rw) register accessor: an alias for `Reg<HASH_CSR71_SPEC>`"]
pub type HASH_CSR71 = crate::Reg<hash_csr71::HASH_CSR71_SPEC>;
#[doc = "HASH context swap register 71"]
pub mod hash_csr71;
#[doc = "HASH_CSR72 (rw) register accessor: an alias for `Reg<HASH_CSR72_SPEC>`"]
pub type HASH_CSR72 = crate::Reg<hash_csr72::HASH_CSR72_SPEC>;
#[doc = "HASH context swap register 72"]
pub mod hash_csr72;
#[doc = "HASH_CSR73 (rw) register accessor: an alias for `Reg<HASH_CSR73_SPEC>`"]
pub type HASH_CSR73 = crate::Reg<hash_csr73::HASH_CSR73_SPEC>;
#[doc = "HASH context swap register 73"]
pub mod hash_csr73;
#[doc = "HASH_CSR74 (rw) register accessor: an alias for `Reg<HASH_CSR74_SPEC>`"]
pub type HASH_CSR74 = crate::Reg<hash_csr74::HASH_CSR74_SPEC>;
#[doc = "HASH context swap register 74"]
pub mod hash_csr74;
#[doc = "HASH_CSR75 (rw) register accessor: an alias for `Reg<HASH_CSR75_SPEC>`"]
pub type HASH_CSR75 = crate::Reg<hash_csr75::HASH_CSR75_SPEC>;
#[doc = "HASH context swap register 75"]
pub mod hash_csr75;
#[doc = "HASH_CSR76 (rw) register accessor: an alias for `Reg<HASH_CSR76_SPEC>`"]
pub type HASH_CSR76 = crate::Reg<hash_csr76::HASH_CSR76_SPEC>;
#[doc = "HASH context swap register 76"]
pub mod hash_csr76;
#[doc = "HASH_CSR77 (rw) register accessor: an alias for `Reg<HASH_CSR77_SPEC>`"]
pub type HASH_CSR77 = crate::Reg<hash_csr77::HASH_CSR77_SPEC>;
#[doc = "HASH context swap register 77"]
pub mod hash_csr77;
#[doc = "HASH_CSR78 (rw) register accessor: an alias for `Reg<HASH_CSR78_SPEC>`"]
pub type HASH_CSR78 = crate::Reg<hash_csr78::HASH_CSR78_SPEC>;
#[doc = "HASH context swap register 78"]
pub mod hash_csr78;
#[doc = "HASH_CSR79 (rw) register accessor: an alias for `Reg<HASH_CSR79_SPEC>`"]
pub type HASH_CSR79 = crate::Reg<hash_csr79::HASH_CSR79_SPEC>;
#[doc = "HASH context swap register 79"]
pub mod hash_csr79;
#[doc = "HASH_CSR80 (rw) register accessor: an alias for `Reg<HASH_CSR80_SPEC>`"]
pub type HASH_CSR80 = crate::Reg<hash_csr80::HASH_CSR80_SPEC>;
#[doc = "HASH context swap register 80"]
pub mod hash_csr80;
#[doc = "HASH_CSR81 (rw) register accessor: an alias for `Reg<HASH_CSR81_SPEC>`"]
pub type HASH_CSR81 = crate::Reg<hash_csr81::HASH_CSR81_SPEC>;
#[doc = "HASH context swap register 81"]
pub mod hash_csr81;
#[doc = "HASH_CSR82 (rw) register accessor: an alias for `Reg<HASH_CSR82_SPEC>`"]
pub type HASH_CSR82 = crate::Reg<hash_csr82::HASH_CSR82_SPEC>;
#[doc = "HASH context swap register 82"]
pub mod hash_csr82;
#[doc = "HASH_CSR83 (rw) register accessor: an alias for `Reg<HASH_CSR83_SPEC>`"]
pub type HASH_CSR83 = crate::Reg<hash_csr83::HASH_CSR83_SPEC>;
#[doc = "HASH context swap register 83"]
pub mod hash_csr83;
#[doc = "HASH_CSR84 (rw) register accessor: an alias for `Reg<HASH_CSR84_SPEC>`"]
pub type HASH_CSR84 = crate::Reg<hash_csr84::HASH_CSR84_SPEC>;
#[doc = "HASH context swap register 84"]
pub mod hash_csr84;
#[doc = "HASH_CSR85 (rw) register accessor: an alias for `Reg<HASH_CSR85_SPEC>`"]
pub type HASH_CSR85 = crate::Reg<hash_csr85::HASH_CSR85_SPEC>;
#[doc = "HASH context swap register 85"]
pub mod hash_csr85;
#[doc = "HASH_CSR86 (rw) register accessor: an alias for `Reg<HASH_CSR86_SPEC>`"]
pub type HASH_CSR86 = crate::Reg<hash_csr86::HASH_CSR86_SPEC>;
#[doc = "HASH context swap register 86"]
pub mod hash_csr86;
#[doc = "HASH_CSR87 (rw) register accessor: an alias for `Reg<HASH_CSR87_SPEC>`"]
pub type HASH_CSR87 = crate::Reg<hash_csr87::HASH_CSR87_SPEC>;
#[doc = "HASH context swap register 87"]
pub mod hash_csr87;
#[doc = "HASH_CSR88 (rw) register accessor: an alias for `Reg<HASH_CSR88_SPEC>`"]
pub type HASH_CSR88 = crate::Reg<hash_csr88::HASH_CSR88_SPEC>;
#[doc = "HASH context swap register 88"]
pub mod hash_csr88;
#[doc = "HASH_CSR89 (rw) register accessor: an alias for `Reg<HASH_CSR89_SPEC>`"]
pub type HASH_CSR89 = crate::Reg<hash_csr89::HASH_CSR89_SPEC>;
#[doc = "HASH context swap register 89"]
pub mod hash_csr89;
#[doc = "HASH_CSR90 (rw) register accessor: an alias for `Reg<HASH_CSR90_SPEC>`"]
pub type HASH_CSR90 = crate::Reg<hash_csr90::HASH_CSR90_SPEC>;
#[doc = "HASH context swap register 90"]
pub mod hash_csr90;
#[doc = "HASH_CSR91 (rw) register accessor: an alias for `Reg<HASH_CSR91_SPEC>`"]
pub type HASH_CSR91 = crate::Reg<hash_csr91::HASH_CSR91_SPEC>;
#[doc = "HASH context swap register 91"]
pub mod hash_csr91;
#[doc = "HASH_CSR92 (rw) register accessor: an alias for `Reg<HASH_CSR92_SPEC>`"]
pub type HASH_CSR92 = crate::Reg<hash_csr92::HASH_CSR92_SPEC>;
#[doc = "HASH context swap register 92"]
pub mod hash_csr92;
#[doc = "HASH_CSR93 (rw) register accessor: an alias for `Reg<HASH_CSR93_SPEC>`"]
pub type HASH_CSR93 = crate::Reg<hash_csr93::HASH_CSR93_SPEC>;
#[doc = "HASH context swap register 93"]
pub mod hash_csr93;
#[doc = "HASH_CSR94 (rw) register accessor: an alias for `Reg<HASH_CSR94_SPEC>`"]
pub type HASH_CSR94 = crate::Reg<hash_csr94::HASH_CSR94_SPEC>;
#[doc = "HASH context swap register 94"]
pub mod hash_csr94;
#[doc = "HASH_CSR95 (rw) register accessor: an alias for `Reg<HASH_CSR95_SPEC>`"]
pub type HASH_CSR95 = crate::Reg<hash_csr95::HASH_CSR95_SPEC>;
#[doc = "HASH context swap register 95"]
pub mod hash_csr95;
#[doc = "HASH_CSR96 (rw) register accessor: an alias for `Reg<HASH_CSR96_SPEC>`"]
pub type HASH_CSR96 = crate::Reg<hash_csr96::HASH_CSR96_SPEC>;
#[doc = "HASH context swap register 96"]
pub mod hash_csr96;
#[doc = "HASH_CSR97 (rw) register accessor: an alias for `Reg<HASH_CSR97_SPEC>`"]
pub type HASH_CSR97 = crate::Reg<hash_csr97::HASH_CSR97_SPEC>;
#[doc = "HASH context swap register 97"]
pub mod hash_csr97;
#[doc = "HASH_CSR98 (rw) register accessor: an alias for `Reg<HASH_CSR98_SPEC>`"]
pub type HASH_CSR98 = crate::Reg<hash_csr98::HASH_CSR98_SPEC>;
#[doc = "HASH context swap register 98"]
pub mod hash_csr98;
#[doc = "HASH_CSR99 (rw) register accessor: an alias for `Reg<HASH_CSR99_SPEC>`"]
pub type HASH_CSR99 = crate::Reg<hash_csr99::HASH_CSR99_SPEC>;
#[doc = "HASH context swap register 99"]
pub mod hash_csr99;
#[doc = "HASH_CSR100 (rw) register accessor: an alias for `Reg<HASH_CSR100_SPEC>`"]
pub type HASH_CSR100 = crate::Reg<hash_csr100::HASH_CSR100_SPEC>;
#[doc = "HASH context swap register 100"]
pub mod hash_csr100;
#[doc = "HASH_CSR101 (rw) register accessor: an alias for `Reg<HASH_CSR101_SPEC>`"]
pub type HASH_CSR101 = crate::Reg<hash_csr101::HASH_CSR101_SPEC>;
#[doc = "HASH context swap register 101"]
pub mod hash_csr101;
#[doc = "HASH_CSR102 (rw) register accessor: an alias for `Reg<HASH_CSR102_SPEC>`"]
pub type HASH_CSR102 = crate::Reg<hash_csr102::HASH_CSR102_SPEC>;
#[doc = "HASH context swap register 102"]
pub mod hash_csr102;
#[doc = "HASH_HR0 (r) register accessor: an alias for `Reg<HASH_HR0_SPEC>`"]
pub type HASH_HR0 = crate::Reg<hash_hr0::HASH_HR0_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr0;
#[doc = "HASH_HR1 (r) register accessor: an alias for `Reg<HASH_HR1_SPEC>`"]
pub type HASH_HR1 = crate::Reg<hash_hr1::HASH_HR1_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr1;
#[doc = "HASH_HR2 (r) register accessor: an alias for `Reg<HASH_HR2_SPEC>`"]
pub type HASH_HR2 = crate::Reg<hash_hr2::HASH_HR2_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr2;
#[doc = "HASH_HR3 (r) register accessor: an alias for `Reg<HASH_HR3_SPEC>`"]
pub type HASH_HR3 = crate::Reg<hash_hr3::HASH_HR3_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr3;
#[doc = "HASH_HR4 (r) register accessor: an alias for `Reg<HASH_HR4_SPEC>`"]
pub type HASH_HR4 = crate::Reg<hash_hr4::HASH_HR4_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr4;
#[doc = "HASH_HR5 (r) register accessor: an alias for `Reg<HASH_HR5_SPEC>`"]
pub type HASH_HR5 = crate::Reg<hash_hr5::HASH_HR5_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr5;
#[doc = "HASH_HR6 (r) register accessor: an alias for `Reg<HASH_HR6_SPEC>`"]
pub type HASH_HR6 = crate::Reg<hash_hr6::HASH_HR6_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr6;
#[doc = "HASH_HR7 (r) register accessor: an alias for `Reg<HASH_HR7_SPEC>`"]
pub type HASH_HR7 = crate::Reg<hash_hr7::HASH_HR7_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr7;
#[doc = "HASH_HR8 (r) register accessor: an alias for `Reg<HASH_HR8_SPEC>`"]
pub type HASH_HR8 = crate::Reg<hash_hr8::HASH_HR8_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr8;
#[doc = "HASH_HR9 (r) register accessor: an alias for `Reg<HASH_HR9_SPEC>`"]
pub type HASH_HR9 = crate::Reg<hash_hr9::HASH_HR9_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr9;
#[doc = "HASH_HR10 (r) register accessor: an alias for `Reg<HASH_HR10_SPEC>`"]
pub type HASH_HR10 = crate::Reg<hash_hr10::HASH_HR10_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr10;
#[doc = "HASH_HR11 (r) register accessor: an alias for `Reg<HASH_HR11_SPEC>`"]
pub type HASH_HR11 = crate::Reg<hash_hr11::HASH_HR11_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr11;
#[doc = "HASH_HR12 (r) register accessor: an alias for `Reg<HASH_HR12_SPEC>`"]
pub type HASH_HR12 = crate::Reg<hash_hr12::HASH_HR12_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr12;
#[doc = "HASH_HR13 (r) register accessor: an alias for `Reg<HASH_HR13_SPEC>`"]
pub type HASH_HR13 = crate::Reg<hash_hr13::HASH_HR13_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr13;
#[doc = "HASH_HR14 (r) register accessor: an alias for `Reg<HASH_HR14_SPEC>`"]
pub type HASH_HR14 = crate::Reg<hash_hr14::HASH_HR14_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr14;
#[doc = "HASH_HR15 (r) register accessor: an alias for `Reg<HASH_HR15_SPEC>`"]
pub type HASH_HR15 = crate::Reg<hash_hr15::HASH_HR15_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr15;
