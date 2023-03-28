#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HASH control register"]
    pub hash_cr: HASH_CR,
    #[doc = "0x04 - HASH_DIN is the data input register."]
    pub hash_din: HASH_DIN,
    #[doc = "0x08 - The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
    pub hash_str: HASH_STR,
    #[doc = "0x0c - HASH digest register 0"]
    pub hash_hr0: HASH_HR0,
    #[doc = "0x10 - HASH digest register 1"]
    pub hash_hr1: HASH_HR1,
    #[doc = "0x14 - HASH digest register 2"]
    pub hash_hr2: HASH_HR2,
    #[doc = "0x18 - HASH digest register 3"]
    pub hash_hr3: HASH_HR3,
    #[doc = "0x1c - HASH digest register 4"]
    pub hash_hr4: HASH_HR4,
    #[doc = "0x20 - HASH interrupt enable register"]
    pub hash_imr: HASH_IMR,
    #[doc = "0x24 - HASH status register"]
    pub hash_sr: HASH_SR,
    _reserved10: [u8; 0xd0],
    #[doc = "0xf8 - These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
    pub hash_csr0: HASH_CSR0,
    #[doc = "0xfc - HASH context swap registers"]
    pub hash_csr1: HASH_CSR1,
    #[doc = "0x100 - HASH context swap registers"]
    pub hash_csr2: HASH_CSR2,
    #[doc = "0x104 - HASH context swap registers"]
    pub hash_csr3: HASH_CSR3,
    #[doc = "0x108 - HASH context swap registers"]
    pub hash_csr4: HASH_CSR4,
    #[doc = "0x10c - HASH context swap registers"]
    pub hash_csr5: HASH_CSR5,
    #[doc = "0x110 - HASH context swap registers"]
    pub hash_csr6: HASH_CSR6,
    #[doc = "0x114 - HASH context swap registers"]
    pub hash_csr7: HASH_CSR7,
    #[doc = "0x118 - HASH context swap registers"]
    pub hash_csr8: HASH_CSR8,
    #[doc = "0x11c - HASH context swap registers"]
    pub hash_csr9: HASH_CSR9,
    #[doc = "0x120 - HASH context swap registers"]
    pub hash_csr10: HASH_CSR10,
    #[doc = "0x124 - HASH context swap registers"]
    pub hash_csr11: HASH_CSR11,
    #[doc = "0x128 - HASH context swap registers"]
    pub hash_csr12: HASH_CSR12,
    #[doc = "0x12c - HASH context swap registers"]
    pub hash_csr13: HASH_CSR13,
    #[doc = "0x130 - HASH context swap registers"]
    pub hash_csr14: HASH_CSR14,
    #[doc = "0x134 - HASH context swap registers"]
    pub hash_csr15: HASH_CSR15,
    #[doc = "0x138 - HASH context swap registers"]
    pub hash_csr16: HASH_CSR16,
    #[doc = "0x13c - HASH context swap registers"]
    pub hash_csr17: HASH_CSR17,
    #[doc = "0x140 - HASH context swap registers"]
    pub hash_csr18: HASH_CSR18,
    #[doc = "0x144 - HASH context swap registers"]
    pub hash_csr19: HASH_CSR19,
    #[doc = "0x148 - HASH context swap registers"]
    pub hash_csr20: HASH_CSR20,
    #[doc = "0x14c - HASH context swap registers"]
    pub hash_csr21: HASH_CSR21,
    #[doc = "0x150 - HASH context swap registers"]
    pub hash_csr22: HASH_CSR22,
    #[doc = "0x154 - HASH context swap registers"]
    pub hash_csr23: HASH_CSR23,
    #[doc = "0x158 - HASH context swap registers"]
    pub hash_csr24: HASH_CSR24,
    #[doc = "0x15c - HASH context swap registers"]
    pub hash_csr25: HASH_CSR25,
    #[doc = "0x160 - HASH context swap registers"]
    pub hash_csr26: HASH_CSR26,
    #[doc = "0x164 - HASH context swap registers"]
    pub hash_csr27: HASH_CSR27,
    #[doc = "0x168 - HASH context swap registers"]
    pub hash_csr28: HASH_CSR28,
    #[doc = "0x16c - HASH context swap registers"]
    pub hash_csr29: HASH_CSR29,
    #[doc = "0x170 - HASH context swap registers"]
    pub hash_csr30: HASH_CSR30,
    #[doc = "0x174 - HASH context swap registers"]
    pub hash_csr31: HASH_CSR31,
    #[doc = "0x178 - HASH context swap registers"]
    pub hash_csr32: HASH_CSR32,
    #[doc = "0x17c - HASH context swap registers"]
    pub hash_csr33: HASH_CSR33,
    #[doc = "0x180 - HASH context swap registers"]
    pub hash_csr34: HASH_CSR34,
    #[doc = "0x184 - HASH context swap registers"]
    pub hash_csr35: HASH_CSR35,
    #[doc = "0x188 - HASH context swap registers"]
    pub hash_csr36: HASH_CSR36,
    #[doc = "0x18c - HASH context swap registers"]
    pub hash_csr37: HASH_CSR37,
    #[doc = "0x190 - HASH context swap registers"]
    pub hash_csr38: HASH_CSR38,
    #[doc = "0x194 - HASH context swap registers"]
    pub hash_csr39: HASH_CSR39,
    #[doc = "0x198 - HASH context swap registers"]
    pub hash_csr40: HASH_CSR40,
    #[doc = "0x19c - HASH context swap registers"]
    pub hash_csr41: HASH_CSR41,
    #[doc = "0x1a0 - HASH context swap registers"]
    pub hash_csr42: HASH_CSR42,
    #[doc = "0x1a4 - HASH context swap registers"]
    pub hash_csr43: HASH_CSR43,
    #[doc = "0x1a8 - HASH context swap registers"]
    pub hash_csr44: HASH_CSR44,
    #[doc = "0x1ac - HASH context swap registers"]
    pub hash_csr45: HASH_CSR45,
    #[doc = "0x1b0 - HASH context swap registers"]
    pub hash_csr46: HASH_CSR46,
    #[doc = "0x1b4 - HASH context swap registers"]
    pub hash_csr47: HASH_CSR47,
    #[doc = "0x1b8 - HASH context swap registers"]
    pub hash_csr48: HASH_CSR48,
    #[doc = "0x1bc - HASH context swap registers"]
    pub hash_csr49: HASH_CSR49,
    #[doc = "0x1c0 - HASH context swap registers"]
    pub hash_csr50: HASH_CSR50,
    #[doc = "0x1c4 - HASH context swap registers"]
    pub hash_csr51: HASH_CSR51,
    #[doc = "0x1c8 - HASH context swap registers"]
    pub hash_csr52: HASH_CSR52,
    #[doc = "0x1cc - HASH context swap registers"]
    pub hash_csr53: HASH_CSR53,
    _reserved64: [u8; 0x0154],
    #[doc = "0x324 - HASH digest register 5"]
    pub hash_hr5: HASH_HR5,
    #[doc = "0x328 - HASH digest register 6"]
    pub hash_hr6: HASH_HR6,
    #[doc = "0x32c - HASH digest register 7"]
    pub hash_hr7: HASH_HR7,
    _reserved67: [u8; 0xc0],
    #[doc = "0x3f0 - HASH Hardware Configuration Register"]
    pub hash_hwcfgr: HASH_HWCFGR,
    #[doc = "0x3f4 - HASH Version Register"]
    pub hash_verr: HASH_VERR,
    #[doc = "0x3f8 - HASH Identification"]
    pub hash_ipidr: HASH_IPIDR,
    #[doc = "0x3fc - HASH Hardware Magic ID"]
    pub hash_mid: HASH_MID,
}
#[doc = "HASH_CR (rw) register accessor: an alias for `Reg<HASH_CR_SPEC>`"]
pub type HASH_CR = crate::Reg<hash_cr::HASH_CR_SPEC>;
#[doc = "HASH control register"]
pub mod hash_cr;
#[doc = "HASH_DIN (rw) register accessor: an alias for `Reg<HASH_DIN_SPEC>`"]
pub type HASH_DIN = crate::Reg<hash_din::HASH_DIN_SPEC>;
#[doc = "HASH_DIN is the data input register."]
pub mod hash_din;
#[doc = "HASH_STR (rw) register accessor: an alias for `Reg<HASH_STR_SPEC>`"]
pub type HASH_STR = crate::Reg<hash_str::HASH_STR_SPEC>;
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
pub mod hash_str;
#[doc = "HASH_HR0 (r) register accessor: an alias for `Reg<HASH_HR0_SPEC>`"]
pub type HASH_HR0 = crate::Reg<hash_hr0::HASH_HR0_SPEC>;
#[doc = "HASH digest register 0"]
pub mod hash_hr0;
#[doc = "HASH_HR1 (r) register accessor: an alias for `Reg<HASH_HR1_SPEC>`"]
pub type HASH_HR1 = crate::Reg<hash_hr1::HASH_HR1_SPEC>;
#[doc = "HASH digest register 1"]
pub mod hash_hr1;
#[doc = "HASH_HR2 (r) register accessor: an alias for `Reg<HASH_HR2_SPEC>`"]
pub type HASH_HR2 = crate::Reg<hash_hr2::HASH_HR2_SPEC>;
#[doc = "HASH digest register 2"]
pub mod hash_hr2;
#[doc = "HASH_HR3 (r) register accessor: an alias for `Reg<HASH_HR3_SPEC>`"]
pub type HASH_HR3 = crate::Reg<hash_hr3::HASH_HR3_SPEC>;
#[doc = "HASH digest register 3"]
pub mod hash_hr3;
#[doc = "HASH_HR4 (r) register accessor: an alias for `Reg<HASH_HR4_SPEC>`"]
pub type HASH_HR4 = crate::Reg<hash_hr4::HASH_HR4_SPEC>;
#[doc = "HASH digest register 4"]
pub mod hash_hr4;
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
#[doc = "These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
pub mod hash_csr0;
#[doc = "HASH_CSR1 (rw) register accessor: an alias for `Reg<HASH_CSR1_SPEC>`"]
pub type HASH_CSR1 = crate::Reg<hash_csr1::HASH_CSR1_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr1;
#[doc = "HASH_CSR2 (rw) register accessor: an alias for `Reg<HASH_CSR2_SPEC>`"]
pub type HASH_CSR2 = crate::Reg<hash_csr2::HASH_CSR2_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr2;
#[doc = "HASH_CSR3 (rw) register accessor: an alias for `Reg<HASH_CSR3_SPEC>`"]
pub type HASH_CSR3 = crate::Reg<hash_csr3::HASH_CSR3_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr3;
#[doc = "HASH_CSR4 (rw) register accessor: an alias for `Reg<HASH_CSR4_SPEC>`"]
pub type HASH_CSR4 = crate::Reg<hash_csr4::HASH_CSR4_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr4;
#[doc = "HASH_CSR5 (rw) register accessor: an alias for `Reg<HASH_CSR5_SPEC>`"]
pub type HASH_CSR5 = crate::Reg<hash_csr5::HASH_CSR5_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr5;
#[doc = "HASH_CSR6 (rw) register accessor: an alias for `Reg<HASH_CSR6_SPEC>`"]
pub type HASH_CSR6 = crate::Reg<hash_csr6::HASH_CSR6_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr6;
#[doc = "HASH_CSR7 (rw) register accessor: an alias for `Reg<HASH_CSR7_SPEC>`"]
pub type HASH_CSR7 = crate::Reg<hash_csr7::HASH_CSR7_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr7;
#[doc = "HASH_CSR8 (rw) register accessor: an alias for `Reg<HASH_CSR8_SPEC>`"]
pub type HASH_CSR8 = crate::Reg<hash_csr8::HASH_CSR8_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr8;
#[doc = "HASH_CSR9 (rw) register accessor: an alias for `Reg<HASH_CSR9_SPEC>`"]
pub type HASH_CSR9 = crate::Reg<hash_csr9::HASH_CSR9_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr9;
#[doc = "HASH_CSR10 (rw) register accessor: an alias for `Reg<HASH_CSR10_SPEC>`"]
pub type HASH_CSR10 = crate::Reg<hash_csr10::HASH_CSR10_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr10;
#[doc = "HASH_CSR11 (rw) register accessor: an alias for `Reg<HASH_CSR11_SPEC>`"]
pub type HASH_CSR11 = crate::Reg<hash_csr11::HASH_CSR11_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr11;
#[doc = "HASH_CSR12 (rw) register accessor: an alias for `Reg<HASH_CSR12_SPEC>`"]
pub type HASH_CSR12 = crate::Reg<hash_csr12::HASH_CSR12_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr12;
#[doc = "HASH_CSR13 (rw) register accessor: an alias for `Reg<HASH_CSR13_SPEC>`"]
pub type HASH_CSR13 = crate::Reg<hash_csr13::HASH_CSR13_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr13;
#[doc = "HASH_CSR14 (rw) register accessor: an alias for `Reg<HASH_CSR14_SPEC>`"]
pub type HASH_CSR14 = crate::Reg<hash_csr14::HASH_CSR14_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr14;
#[doc = "HASH_CSR15 (rw) register accessor: an alias for `Reg<HASH_CSR15_SPEC>`"]
pub type HASH_CSR15 = crate::Reg<hash_csr15::HASH_CSR15_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr15;
#[doc = "HASH_CSR16 (rw) register accessor: an alias for `Reg<HASH_CSR16_SPEC>`"]
pub type HASH_CSR16 = crate::Reg<hash_csr16::HASH_CSR16_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr16;
#[doc = "HASH_CSR17 (rw) register accessor: an alias for `Reg<HASH_CSR17_SPEC>`"]
pub type HASH_CSR17 = crate::Reg<hash_csr17::HASH_CSR17_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr17;
#[doc = "HASH_CSR18 (rw) register accessor: an alias for `Reg<HASH_CSR18_SPEC>`"]
pub type HASH_CSR18 = crate::Reg<hash_csr18::HASH_CSR18_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr18;
#[doc = "HASH_CSR19 (rw) register accessor: an alias for `Reg<HASH_CSR19_SPEC>`"]
pub type HASH_CSR19 = crate::Reg<hash_csr19::HASH_CSR19_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr19;
#[doc = "HASH_CSR20 (rw) register accessor: an alias for `Reg<HASH_CSR20_SPEC>`"]
pub type HASH_CSR20 = crate::Reg<hash_csr20::HASH_CSR20_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr20;
#[doc = "HASH_CSR21 (rw) register accessor: an alias for `Reg<HASH_CSR21_SPEC>`"]
pub type HASH_CSR21 = crate::Reg<hash_csr21::HASH_CSR21_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr21;
#[doc = "HASH_CSR22 (rw) register accessor: an alias for `Reg<HASH_CSR22_SPEC>`"]
pub type HASH_CSR22 = crate::Reg<hash_csr22::HASH_CSR22_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr22;
#[doc = "HASH_CSR23 (rw) register accessor: an alias for `Reg<HASH_CSR23_SPEC>`"]
pub type HASH_CSR23 = crate::Reg<hash_csr23::HASH_CSR23_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr23;
#[doc = "HASH_CSR24 (rw) register accessor: an alias for `Reg<HASH_CSR24_SPEC>`"]
pub type HASH_CSR24 = crate::Reg<hash_csr24::HASH_CSR24_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr24;
#[doc = "HASH_CSR25 (rw) register accessor: an alias for `Reg<HASH_CSR25_SPEC>`"]
pub type HASH_CSR25 = crate::Reg<hash_csr25::HASH_CSR25_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr25;
#[doc = "HASH_CSR26 (rw) register accessor: an alias for `Reg<HASH_CSR26_SPEC>`"]
pub type HASH_CSR26 = crate::Reg<hash_csr26::HASH_CSR26_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr26;
#[doc = "HASH_CSR27 (rw) register accessor: an alias for `Reg<HASH_CSR27_SPEC>`"]
pub type HASH_CSR27 = crate::Reg<hash_csr27::HASH_CSR27_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr27;
#[doc = "HASH_CSR28 (rw) register accessor: an alias for `Reg<HASH_CSR28_SPEC>`"]
pub type HASH_CSR28 = crate::Reg<hash_csr28::HASH_CSR28_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr28;
#[doc = "HASH_CSR29 (rw) register accessor: an alias for `Reg<HASH_CSR29_SPEC>`"]
pub type HASH_CSR29 = crate::Reg<hash_csr29::HASH_CSR29_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr29;
#[doc = "HASH_CSR30 (rw) register accessor: an alias for `Reg<HASH_CSR30_SPEC>`"]
pub type HASH_CSR30 = crate::Reg<hash_csr30::HASH_CSR30_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr30;
#[doc = "HASH_CSR31 (rw) register accessor: an alias for `Reg<HASH_CSR31_SPEC>`"]
pub type HASH_CSR31 = crate::Reg<hash_csr31::HASH_CSR31_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr31;
#[doc = "HASH_CSR32 (rw) register accessor: an alias for `Reg<HASH_CSR32_SPEC>`"]
pub type HASH_CSR32 = crate::Reg<hash_csr32::HASH_CSR32_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr32;
#[doc = "HASH_CSR33 (rw) register accessor: an alias for `Reg<HASH_CSR33_SPEC>`"]
pub type HASH_CSR33 = crate::Reg<hash_csr33::HASH_CSR33_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr33;
#[doc = "HASH_CSR34 (rw) register accessor: an alias for `Reg<HASH_CSR34_SPEC>`"]
pub type HASH_CSR34 = crate::Reg<hash_csr34::HASH_CSR34_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr34;
#[doc = "HASH_CSR35 (rw) register accessor: an alias for `Reg<HASH_CSR35_SPEC>`"]
pub type HASH_CSR35 = crate::Reg<hash_csr35::HASH_CSR35_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr35;
#[doc = "HASH_CSR36 (rw) register accessor: an alias for `Reg<HASH_CSR36_SPEC>`"]
pub type HASH_CSR36 = crate::Reg<hash_csr36::HASH_CSR36_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr36;
#[doc = "HASH_CSR37 (rw) register accessor: an alias for `Reg<HASH_CSR37_SPEC>`"]
pub type HASH_CSR37 = crate::Reg<hash_csr37::HASH_CSR37_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr37;
#[doc = "HASH_CSR38 (rw) register accessor: an alias for `Reg<HASH_CSR38_SPEC>`"]
pub type HASH_CSR38 = crate::Reg<hash_csr38::HASH_CSR38_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr38;
#[doc = "HASH_CSR39 (rw) register accessor: an alias for `Reg<HASH_CSR39_SPEC>`"]
pub type HASH_CSR39 = crate::Reg<hash_csr39::HASH_CSR39_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr39;
#[doc = "HASH_CSR40 (rw) register accessor: an alias for `Reg<HASH_CSR40_SPEC>`"]
pub type HASH_CSR40 = crate::Reg<hash_csr40::HASH_CSR40_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr40;
#[doc = "HASH_CSR41 (rw) register accessor: an alias for `Reg<HASH_CSR41_SPEC>`"]
pub type HASH_CSR41 = crate::Reg<hash_csr41::HASH_CSR41_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr41;
#[doc = "HASH_CSR42 (rw) register accessor: an alias for `Reg<HASH_CSR42_SPEC>`"]
pub type HASH_CSR42 = crate::Reg<hash_csr42::HASH_CSR42_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr42;
#[doc = "HASH_CSR43 (rw) register accessor: an alias for `Reg<HASH_CSR43_SPEC>`"]
pub type HASH_CSR43 = crate::Reg<hash_csr43::HASH_CSR43_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr43;
#[doc = "HASH_CSR44 (rw) register accessor: an alias for `Reg<HASH_CSR44_SPEC>`"]
pub type HASH_CSR44 = crate::Reg<hash_csr44::HASH_CSR44_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr44;
#[doc = "HASH_CSR45 (rw) register accessor: an alias for `Reg<HASH_CSR45_SPEC>`"]
pub type HASH_CSR45 = crate::Reg<hash_csr45::HASH_CSR45_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr45;
#[doc = "HASH_CSR46 (rw) register accessor: an alias for `Reg<HASH_CSR46_SPEC>`"]
pub type HASH_CSR46 = crate::Reg<hash_csr46::HASH_CSR46_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr46;
#[doc = "HASH_CSR47 (rw) register accessor: an alias for `Reg<HASH_CSR47_SPEC>`"]
pub type HASH_CSR47 = crate::Reg<hash_csr47::HASH_CSR47_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr47;
#[doc = "HASH_CSR48 (rw) register accessor: an alias for `Reg<HASH_CSR48_SPEC>`"]
pub type HASH_CSR48 = crate::Reg<hash_csr48::HASH_CSR48_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr48;
#[doc = "HASH_CSR49 (rw) register accessor: an alias for `Reg<HASH_CSR49_SPEC>`"]
pub type HASH_CSR49 = crate::Reg<hash_csr49::HASH_CSR49_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr49;
#[doc = "HASH_CSR50 (rw) register accessor: an alias for `Reg<HASH_CSR50_SPEC>`"]
pub type HASH_CSR50 = crate::Reg<hash_csr50::HASH_CSR50_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr50;
#[doc = "HASH_CSR51 (rw) register accessor: an alias for `Reg<HASH_CSR51_SPEC>`"]
pub type HASH_CSR51 = crate::Reg<hash_csr51::HASH_CSR51_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr51;
#[doc = "HASH_CSR52 (rw) register accessor: an alias for `Reg<HASH_CSR52_SPEC>`"]
pub type HASH_CSR52 = crate::Reg<hash_csr52::HASH_CSR52_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr52;
#[doc = "HASH_CSR53 (rw) register accessor: an alias for `Reg<HASH_CSR53_SPEC>`"]
pub type HASH_CSR53 = crate::Reg<hash_csr53::HASH_CSR53_SPEC>;
#[doc = "HASH context swap registers"]
pub mod hash_csr53;
#[doc = "HASH_HR5 (r) register accessor: an alias for `Reg<HASH_HR5_SPEC>`"]
pub type HASH_HR5 = crate::Reg<hash_hr5::HASH_HR5_SPEC>;
#[doc = "HASH digest register 5"]
pub mod hash_hr5;
#[doc = "HASH_HR6 (r) register accessor: an alias for `Reg<HASH_HR6_SPEC>`"]
pub type HASH_HR6 = crate::Reg<hash_hr6::HASH_HR6_SPEC>;
#[doc = "HASH digest register 6"]
pub mod hash_hr6;
#[doc = "HASH_HR7 (r) register accessor: an alias for `Reg<HASH_HR7_SPEC>`"]
pub type HASH_HR7 = crate::Reg<hash_hr7::HASH_HR7_SPEC>;
#[doc = "HASH digest register 7"]
pub mod hash_hr7;
#[doc = "HASH_HWCFGR (r) register accessor: an alias for `Reg<HASH_HWCFGR_SPEC>`"]
pub type HASH_HWCFGR = crate::Reg<hash_hwcfgr::HASH_HWCFGR_SPEC>;
#[doc = "HASH Hardware Configuration Register"]
pub mod hash_hwcfgr;
#[doc = "HASH_VERR (r) register accessor: an alias for `Reg<HASH_VERR_SPEC>`"]
pub type HASH_VERR = crate::Reg<hash_verr::HASH_VERR_SPEC>;
#[doc = "HASH Version Register"]
pub mod hash_verr;
#[doc = "HASH_IPIDR (r) register accessor: an alias for `Reg<HASH_IPIDR_SPEC>`"]
pub type HASH_IPIDR = crate::Reg<hash_ipidr::HASH_IPIDR_SPEC>;
#[doc = "HASH Identification"]
pub mod hash_ipidr;
#[doc = "HASH_MID (r) register accessor: an alias for `Reg<HASH_MID_SPEC>`"]
pub type HASH_MID = crate::Reg<hash_mid::HASH_MID_SPEC>;
#[doc = "HASH Hardware Magic ID"]
pub mod hash_mid;
