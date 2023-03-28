#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DDRCTRL master register 0"]
    pub ddrctrl_mstr: DDRCTRL_MSTR,
    #[doc = "0x04 - DDRCTRL operating mode status register"]
    pub ddrctrl_stat: DDRCTRL_STAT,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
    pub ddrctrl_mrctrl0: DDRCTRL_MRCTRL0,
    #[doc = "0x14 - DDRCTRL mode register read/write control register 1"]
    pub ddrctrl_mrctrl1: DDRCTRL_MRCTRL1,
    #[doc = "0x18 - DDRCTRL mode register read/write status register"]
    pub ddrctrl_mrstat: DDRCTRL_MRSTAT,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - DDRCTRL temperature derate enable register"]
    pub ddrctrl_derateen: DDRCTRL_DERATEEN,
    #[doc = "0x24 - DDRCTRL temperature derate interval register"]
    pub ddrctrl_derateint: DDRCTRL_DERATEINT,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - DDRCTRL low power control register"]
    pub ddrctrl_pwrctl: DDRCTRL_PWRCTL,
    #[doc = "0x34 - DDRCTRL low power timing register"]
    pub ddrctrl_pwrtmg: DDRCTRL_PWRTMG,
    #[doc = "0x38 - DDRCTRL hardware low power control register"]
    pub ddrctrl_hwlpctl: DDRCTRL_HWLPCTL,
    _reserved10: [u8; 0x14],
    #[doc = "0x50 - DDRCTRL refresh control register 0"]
    pub ddrctrl_rfshctl0: DDRCTRL_RFSHCTL0,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - DDRCTRL refresh control register 3"]
    pub ddrctrl_rfshctl3: DDRCTRL_RFSHCTL3,
    #[doc = "0x64 - DDRCTRL refresh timing register"]
    pub ddrctrl_rfshtmg: DDRCTRL_RFSHTMG,
    _reserved13: [u8; 0x58],
    #[doc = "0xc0 - DDRCTRL CRC parity control register 0"]
    pub ddrctrl_crcparctl0: DDRCTRL_CRCPARCTL0,
    _reserved14: [u8; 0x08],
    #[doc = "0xcc - DDRCTRL CRC parity status register"]
    pub ddrctrl_crcparstat: DDRCTRL_CRCPARSTAT,
    #[doc = "0xd0 - DDRCTRL SDRAM initialization register 0"]
    pub ddrctrl_init0: DDRCTRL_INIT0,
    #[doc = "0xd4 - DDRCTRL SDRAM initialization register 1"]
    pub ddrctrl_init1: DDRCTRL_INIT1,
    #[doc = "0xd8 - DDRCTRL SDRAM initialization register 2"]
    pub ddrctrl_init2: DDRCTRL_INIT2,
    #[doc = "0xdc - DDRCTRL SDRAM initialization register 3"]
    pub ddrctrl_init3: DDRCTRL_INIT3,
    #[doc = "0xe0 - DDRCTRL SDRAM initialization register 4"]
    pub ddrctrl_init4: DDRCTRL_INIT4,
    #[doc = "0xe4 - DDRCTRL SDRAM initialization register 5"]
    pub ddrctrl_init5: DDRCTRL_INIT5,
    _reserved21: [u8; 0x08],
    #[doc = "0xf0 - DDRCTRL DIMM control register"]
    pub ddrctrl_dimmctl: DDRCTRL_DIMMCTL,
    _reserved22: [u8; 0x0c],
    #[doc = "0x100 - DDRCTRL SDRAM timing register 0"]
    pub ddrctrl_dramtmg0: DDRCTRL_DRAMTMG0,
    #[doc = "0x104 - DDRCTRL SDRAM timing register 1"]
    pub ddrctrl_dramtmg1: DDRCTRL_DRAMTMG1,
    #[doc = "0x108 - DDRCTRL SDRAM timing register 2"]
    pub ddrctrl_dramtmg2: DDRCTRL_DRAMTMG2,
    #[doc = "0x10c - DDRCTRL SDRAM timing register 3"]
    pub ddrctrl_dramtmg3: DDRCTRL_DRAMTMG3,
    #[doc = "0x110 - DDRCTRL SDRAM timing register 4"]
    pub ddrctrl_dramtmg4: DDRCTRL_DRAMTMG4,
    #[doc = "0x114 - DDRCTRL SDRAM timing register 5"]
    pub ddrctrl_dramtmg5: DDRCTRL_DRAMTMG5,
    #[doc = "0x118 - DDRCTRL SDRAM timing register 6"]
    pub ddrctrl_dramtmg6: DDRCTRL_DRAMTMG6,
    #[doc = "0x11c - DDRCTRL SDRAM timing register 7"]
    pub ddrctrl_dramtmg7: DDRCTRL_DRAMTMG7,
    #[doc = "0x120 - DDRCTRL SDRAM timing register 8"]
    pub ddrctrl_dramtmg8: DDRCTRL_DRAMTMG8,
    _reserved31: [u8; 0x14],
    #[doc = "0x138 - DDRCTRL SDRAM timing register 14"]
    pub ddrctrl_dramtmg14: DDRCTRL_DRAMTMG14,
    #[doc = "0x13c - DDRCTRL SDRAM timing register 15"]
    pub ddrctrl_dramtmg15: DDRCTRL_DRAMTMG15,
    _reserved33: [u8; 0x40],
    #[doc = "0x180 - DDRCTRL ZQ control register 0"]
    pub ddrctrl_zqctl0: DDRCTRL_ZQCTL0,
    #[doc = "0x184 - DDRCTRL ZQ control register 1"]
    pub ddrctrl_zqctl1: DDRCTRL_ZQCTL1,
    #[doc = "0x188 - DDRCTRL ZQ control register 2"]
    pub ddrctrl_zqctl2: DDRCTRL_ZQCTL2,
    #[doc = "0x18c - DDRCTRL ZQ status register"]
    pub ddrctrl_zqstat: DDRCTRL_ZQSTAT,
    #[doc = "0x190 - DDRCTRL DFI timing register 0"]
    pub ddrctrl_dfitmg0: DDRCTRL_DFITMG0,
    #[doc = "0x194 - DDRCTRL DFI timing register 1"]
    pub ddrctrl_dfitmg1: DDRCTRL_DFITMG1,
    #[doc = "0x198 - DDRCTRL low power configuration register 0"]
    pub ddrctrl_dfilpcfg0: DDRCTRL_DFILPCFG0,
    _reserved40: [u8; 0x04],
    #[doc = "0x1a0 - DDRCTRL DFI update register 0"]
    pub ddrctrl_dfiupd0: DDRCTRL_DFIUPD0,
    #[doc = "0x1a4 - DDRCTRL DFI update register 1"]
    pub ddrctrl_dfiupd1: DDRCTRL_DFIUPD1,
    #[doc = "0x1a8 - DDRCTRL DFI update register 2"]
    pub ddrctrl_dfiupd2: DDRCTRL_DFIUPD2,
    _reserved43: [u8; 0x04],
    #[doc = "0x1b0 - DDRCTRL DFI miscellaneous control register"]
    pub ddrctrl_dfimisc: DDRCTRL_DFIMISC,
    _reserved44: [u8; 0x08],
    #[doc = "0x1bc - DDRCTRL DFI status register"]
    pub ddrctrl_dfistat: DDRCTRL_DFISTAT,
    _reserved45: [u8; 0x04],
    #[doc = "0x1c4 - DDRCTRL DFI PHY master register"]
    pub ddrctrl_dfiphymstr: DDRCTRL_DFIPHYMSTR,
    _reserved46: [u8; 0x3c],
    #[doc = "0x204 - DDRCTRL address map register 1"]
    pub ddrctrl_addrmap1: DDRCTRL_ADDRMAP1,
    #[doc = "0x208 - DDRCTRL address map register 2"]
    pub ddrctrl_addrmap2: DDRCTRL_ADDRMAP2,
    #[doc = "0x20c - DDRCTRL address map register 3"]
    pub ddrctrl_addrmap3: DDRCTRL_ADDRMAP3,
    #[doc = "0x210 - DDRCTRL address map register 4"]
    pub ddrctrl_addrmap4: DDRCTRL_ADDRMAP4,
    #[doc = "0x214 - DDRCTRL address map register 5"]
    pub ddrctrl_addrmap5: DDRCTRL_ADDRMAP5,
    #[doc = "0x218 - DDRCTRL address register 6"]
    pub ddrctrl_addrmap6: DDRCTRL_ADDRMAP6,
    _reserved52: [u8; 0x08],
    #[doc = "0x224 - DDRCTRL address map register 9"]
    pub ddrctrl_addrmap9: DDRCTRL_ADDRMAP9,
    #[doc = "0x228 - DDRCTRL address map register 10"]
    pub ddrctrl_addrmap10: DDRCTRL_ADDRMAP10,
    #[doc = "0x22c - DDRCTRL address map register 11"]
    pub ddrctrl_addrmap11: DDRCTRL_ADDRMAP11,
    _reserved55: [u8; 0x10],
    #[doc = "0x240 - DDRCTRL ODT configuration register"]
    pub ddrctrl_odtcfg: DDRCTRL_ODTCFG,
    #[doc = "0x244 - DDRCTRL ODT/Rank map register"]
    pub ddrctrl_odtmap: DDRCTRL_ODTMAP,
    _reserved57: [u8; 0x08],
    #[doc = "0x250 - DDRCTRL scheduler control register"]
    pub ddrctrl_sched: DDRCTRL_SCHED,
    #[doc = "0x254 - DDRCTRL scheduler control register 1"]
    pub ddrctrl_sched1: DDRCTRL_SCHED1,
    _reserved59: [u8; 0x04],
    #[doc = "0x25c - DDRCTRL high priority read CAM register 1"]
    pub ddrctrl_perfhpr1: DDRCTRL_PERFHPR1,
    _reserved60: [u8; 0x04],
    #[doc = "0x264 - DDRCTRL low priority read CAM register 1"]
    pub ddrctrl_perflpr1: DDRCTRL_PERFLPR1,
    _reserved61: [u8; 0x04],
    #[doc = "0x26c - DDRCTRL write CAM register 1"]
    pub ddrctrl_perfwr1: DDRCTRL_PERFWR1,
    _reserved62: [u8; 0x90],
    #[doc = "0x300 - DDRCTRL debug register 0"]
    pub ddrctrl_dbg0: DDRCTRL_DBG0,
    #[doc = "0x304 - DDRCTRL debug register 1"]
    pub ddrctrl_dbg1: DDRCTRL_DBG1,
    #[doc = "0x308 - DDRCTRL CAM debug register"]
    pub ddrctrl_dbgcam: DDRCTRL_DBGCAM,
    #[doc = "0x30c - DDRCTRL command debug register"]
    pub ddrctrl_dbgcmd: DDRCTRL_DBGCMD,
    #[doc = "0x310 - DDRCTRL status debug register"]
    pub ddrctrl_dbgstat: DDRCTRL_DBGSTAT,
    _reserved67: [u8; 0x0c],
    #[doc = "0x320 - DDRCTRL software register programming control enable"]
    pub ddrctrl_swctl: DDRCTRL_SWCTL,
    #[doc = "0x324 - DDRCTRL software register programming control status"]
    pub ddrctrl_swstat: DDRCTRL_SWSTAT,
    _reserved69: [u8; 0x44],
    #[doc = "0x36c - AXI Poison configuration register common for all AXI ports."]
    pub ddrctrl_poisoncfg: DDRCTRL_POISONCFG,
    #[doc = "0x370 - DDRCTRL AXI Poison status register"]
    pub ddrctrl_poisonstat: DDRCTRL_POISONSTAT,
    _reserved71: [u8; 0x88],
    #[doc = "0x3fc - DDRCTRL port status register"]
    pub ddrctrl_pstat: DDRCTRL_PSTAT,
    #[doc = "0x400 - DDRCTRL port common configuration register"]
    pub ddrctrl_pccfg: DDRCTRL_PCCFG,
    #[doc = "0x404 - DDRCTRL port 0 configuration read register"]
    pub ddrctrl_pcfgr_0: DDRCTRL_PCFGR_0,
    #[doc = "0x408 - DDRCTRL port 0 configuration write register"]
    pub ddrctrl_pcfgw_0: DDRCTRL_PCFGW_0,
    _reserved75: [u8; 0x84],
    #[doc = "0x490 - DDRCTRL port 0 control register"]
    pub ddrctrl_pctrl_0: DDRCTRL_PCTRL_0,
    #[doc = "0x494 - DDRCTRL port 0 read Q0S configuration register 0"]
    pub ddrctrl_pcfgqos0_0: DDRCTRL_PCFGQOS0_0,
    #[doc = "0x498 - DDRCTRL port 0 read Q0S configuration register 1"]
    pub ddrctrl_pcfgqos1_0: DDRCTRL_PCFGQOS1_0,
    #[doc = "0x49c - DDRCTRL port 0 write Q0S configuration register 0"]
    pub ddrctrl_pcfgwqos0_0: DDRCTRL_PCFGWQOS0_0,
    #[doc = "0x4a0 - DDRCTRL port 0 write Q0S configuration register 1"]
    pub ddrctrl_pcfgwqos1_0: DDRCTRL_PCFGWQOS1_0,
    _reserved80: [u8; 0x10],
    #[doc = "0x4b4 - DDRCTRL port 1 configuration read register"]
    pub ddrctrl_pcfgr_1: DDRCTRL_PCFGR_1,
    #[doc = "0x4b8 - DDRCTRL port 1 configuration write register"]
    pub ddrctrl_pcfgw_1: DDRCTRL_PCFGW_1,
    _reserved82: [u8; 0x84],
    #[doc = "0x540 - DDRCTRL port 1 control register"]
    pub ddrctrl_pctrl_1: DDRCTRL_PCTRL_1,
    #[doc = "0x544 - DDRCTRL port 1 read Q0S configuration register 0"]
    pub ddrctrl_pcfgqos0_1: DDRCTRL_PCFGQOS0_1,
    #[doc = "0x548 - DDRCTRL port 1 read Q0S configuration register 1"]
    pub ddrctrl_pcfgqos1_1: DDRCTRL_PCFGQOS1_1,
    #[doc = "0x54c - DDRCTRL port 1 write Q0S configuration register 0"]
    pub ddrctrl_pcfgwqos0_1: DDRCTRL_PCFGWQOS0_1,
    #[doc = "0x550 - DDRCTRL port 1 write Q0S configuration register 1"]
    pub ddrctrl_pcfgwqos1_1: DDRCTRL_PCFGWQOS1_1,
}
#[doc = "DDRCTRL_MSTR (rw) register accessor: an alias for `Reg<DDRCTRL_MSTR_SPEC>`"]
pub type DDRCTRL_MSTR = crate::Reg<ddrctrl_mstr::DDRCTRL_MSTR_SPEC>;
#[doc = "DDRCTRL master register 0"]
pub mod ddrctrl_mstr;
#[doc = "DDRCTRL_STAT (r) register accessor: an alias for `Reg<DDRCTRL_STAT_SPEC>`"]
pub type DDRCTRL_STAT = crate::Reg<ddrctrl_stat::DDRCTRL_STAT_SPEC>;
#[doc = "DDRCTRL operating mode status register"]
pub mod ddrctrl_stat;
#[doc = "DDRCTRL_MRCTRL0 (rw) register accessor: an alias for `Reg<DDRCTRL_MRCTRL0_SPEC>`"]
pub type DDRCTRL_MRCTRL0 = crate::Reg<ddrctrl_mrctrl0::DDRCTRL_MRCTRL0_SPEC>;
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
pub mod ddrctrl_mrctrl0;
#[doc = "DDRCTRL_MRCTRL1 (rw) register accessor: an alias for `Reg<DDRCTRL_MRCTRL1_SPEC>`"]
pub type DDRCTRL_MRCTRL1 = crate::Reg<ddrctrl_mrctrl1::DDRCTRL_MRCTRL1_SPEC>;
#[doc = "DDRCTRL mode register read/write control register 1"]
pub mod ddrctrl_mrctrl1;
#[doc = "DDRCTRL_MRSTAT (r) register accessor: an alias for `Reg<DDRCTRL_MRSTAT_SPEC>`"]
pub type DDRCTRL_MRSTAT = crate::Reg<ddrctrl_mrstat::DDRCTRL_MRSTAT_SPEC>;
#[doc = "DDRCTRL mode register read/write status register"]
pub mod ddrctrl_mrstat;
#[doc = "DDRCTRL_DERATEEN (rw) register accessor: an alias for `Reg<DDRCTRL_DERATEEN_SPEC>`"]
pub type DDRCTRL_DERATEEN = crate::Reg<ddrctrl_derateen::DDRCTRL_DERATEEN_SPEC>;
#[doc = "DDRCTRL temperature derate enable register"]
pub mod ddrctrl_derateen;
#[doc = "DDRCTRL_DERATEINT (rw) register accessor: an alias for `Reg<DDRCTRL_DERATEINT_SPEC>`"]
pub type DDRCTRL_DERATEINT = crate::Reg<ddrctrl_derateint::DDRCTRL_DERATEINT_SPEC>;
#[doc = "DDRCTRL temperature derate interval register"]
pub mod ddrctrl_derateint;
#[doc = "DDRCTRL_PWRCTL (rw) register accessor: an alias for `Reg<DDRCTRL_PWRCTL_SPEC>`"]
pub type DDRCTRL_PWRCTL = crate::Reg<ddrctrl_pwrctl::DDRCTRL_PWRCTL_SPEC>;
#[doc = "DDRCTRL low power control register"]
pub mod ddrctrl_pwrctl;
#[doc = "DDRCTRL_PWRTMG (rw) register accessor: an alias for `Reg<DDRCTRL_PWRTMG_SPEC>`"]
pub type DDRCTRL_PWRTMG = crate::Reg<ddrctrl_pwrtmg::DDRCTRL_PWRTMG_SPEC>;
#[doc = "DDRCTRL low power timing register"]
pub mod ddrctrl_pwrtmg;
#[doc = "DDRCTRL_HWLPCTL (rw) register accessor: an alias for `Reg<DDRCTRL_HWLPCTL_SPEC>`"]
pub type DDRCTRL_HWLPCTL = crate::Reg<ddrctrl_hwlpctl::DDRCTRL_HWLPCTL_SPEC>;
#[doc = "DDRCTRL hardware low power control register"]
pub mod ddrctrl_hwlpctl;
#[doc = "DDRCTRL_RFSHCTL0 (rw) register accessor: an alias for `Reg<DDRCTRL_RFSHCTL0_SPEC>`"]
pub type DDRCTRL_RFSHCTL0 = crate::Reg<ddrctrl_rfshctl0::DDRCTRL_RFSHCTL0_SPEC>;
#[doc = "DDRCTRL refresh control register 0"]
pub mod ddrctrl_rfshctl0;
#[doc = "DDRCTRL_RFSHCTL3 (rw) register accessor: an alias for `Reg<DDRCTRL_RFSHCTL3_SPEC>`"]
pub type DDRCTRL_RFSHCTL3 = crate::Reg<ddrctrl_rfshctl3::DDRCTRL_RFSHCTL3_SPEC>;
#[doc = "DDRCTRL refresh control register 3"]
pub mod ddrctrl_rfshctl3;
#[doc = "DDRCTRL_RFSHTMG (rw) register accessor: an alias for `Reg<DDRCTRL_RFSHTMG_SPEC>`"]
pub type DDRCTRL_RFSHTMG = crate::Reg<ddrctrl_rfshtmg::DDRCTRL_RFSHTMG_SPEC>;
#[doc = "DDRCTRL refresh timing register"]
pub mod ddrctrl_rfshtmg;
#[doc = "DDRCTRL_CRCPARCTL0 (rw) register accessor: an alias for `Reg<DDRCTRL_CRCPARCTL0_SPEC>`"]
pub type DDRCTRL_CRCPARCTL0 = crate::Reg<ddrctrl_crcparctl0::DDRCTRL_CRCPARCTL0_SPEC>;
#[doc = "DDRCTRL CRC parity control register 0"]
pub mod ddrctrl_crcparctl0;
#[doc = "DDRCTRL_CRCPARSTAT (r) register accessor: an alias for `Reg<DDRCTRL_CRCPARSTAT_SPEC>`"]
pub type DDRCTRL_CRCPARSTAT = crate::Reg<ddrctrl_crcparstat::DDRCTRL_CRCPARSTAT_SPEC>;
#[doc = "DDRCTRL CRC parity status register"]
pub mod ddrctrl_crcparstat;
#[doc = "DDRCTRL_INIT0 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT0_SPEC>`"]
pub type DDRCTRL_INIT0 = crate::Reg<ddrctrl_init0::DDRCTRL_INIT0_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 0"]
pub mod ddrctrl_init0;
#[doc = "DDRCTRL_INIT1 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT1_SPEC>`"]
pub type DDRCTRL_INIT1 = crate::Reg<ddrctrl_init1::DDRCTRL_INIT1_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 1"]
pub mod ddrctrl_init1;
#[doc = "DDRCTRL_INIT2 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT2_SPEC>`"]
pub type DDRCTRL_INIT2 = crate::Reg<ddrctrl_init2::DDRCTRL_INIT2_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 2"]
pub mod ddrctrl_init2;
#[doc = "DDRCTRL_INIT3 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT3_SPEC>`"]
pub type DDRCTRL_INIT3 = crate::Reg<ddrctrl_init3::DDRCTRL_INIT3_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 3"]
pub mod ddrctrl_init3;
#[doc = "DDRCTRL_INIT4 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT4_SPEC>`"]
pub type DDRCTRL_INIT4 = crate::Reg<ddrctrl_init4::DDRCTRL_INIT4_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 4"]
pub mod ddrctrl_init4;
#[doc = "DDRCTRL_INIT5 (rw) register accessor: an alias for `Reg<DDRCTRL_INIT5_SPEC>`"]
pub type DDRCTRL_INIT5 = crate::Reg<ddrctrl_init5::DDRCTRL_INIT5_SPEC>;
#[doc = "DDRCTRL SDRAM initialization register 5"]
pub mod ddrctrl_init5;
#[doc = "DDRCTRL_DIMMCTL (rw) register accessor: an alias for `Reg<DDRCTRL_DIMMCTL_SPEC>`"]
pub type DDRCTRL_DIMMCTL = crate::Reg<ddrctrl_dimmctl::DDRCTRL_DIMMCTL_SPEC>;
#[doc = "DDRCTRL DIMM control register"]
pub mod ddrctrl_dimmctl;
#[doc = "DDRCTRL_DRAMTMG0 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG0_SPEC>`"]
pub type DDRCTRL_DRAMTMG0 = crate::Reg<ddrctrl_dramtmg0::DDRCTRL_DRAMTMG0_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 0"]
pub mod ddrctrl_dramtmg0;
#[doc = "DDRCTRL_DRAMTMG1 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG1_SPEC>`"]
pub type DDRCTRL_DRAMTMG1 = crate::Reg<ddrctrl_dramtmg1::DDRCTRL_DRAMTMG1_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 1"]
pub mod ddrctrl_dramtmg1;
#[doc = "DDRCTRL_DRAMTMG2 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG2_SPEC>`"]
pub type DDRCTRL_DRAMTMG2 = crate::Reg<ddrctrl_dramtmg2::DDRCTRL_DRAMTMG2_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 2"]
pub mod ddrctrl_dramtmg2;
#[doc = "DDRCTRL_DRAMTMG3 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG3_SPEC>`"]
pub type DDRCTRL_DRAMTMG3 = crate::Reg<ddrctrl_dramtmg3::DDRCTRL_DRAMTMG3_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 3"]
pub mod ddrctrl_dramtmg3;
#[doc = "DDRCTRL_DRAMTMG4 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG4_SPEC>`"]
pub type DDRCTRL_DRAMTMG4 = crate::Reg<ddrctrl_dramtmg4::DDRCTRL_DRAMTMG4_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 4"]
pub mod ddrctrl_dramtmg4;
#[doc = "DDRCTRL_DRAMTMG5 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG5_SPEC>`"]
pub type DDRCTRL_DRAMTMG5 = crate::Reg<ddrctrl_dramtmg5::DDRCTRL_DRAMTMG5_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 5"]
pub mod ddrctrl_dramtmg5;
#[doc = "DDRCTRL_DRAMTMG6 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG6_SPEC>`"]
pub type DDRCTRL_DRAMTMG6 = crate::Reg<ddrctrl_dramtmg6::DDRCTRL_DRAMTMG6_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 6"]
pub mod ddrctrl_dramtmg6;
#[doc = "DDRCTRL_DRAMTMG7 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG7_SPEC>`"]
pub type DDRCTRL_DRAMTMG7 = crate::Reg<ddrctrl_dramtmg7::DDRCTRL_DRAMTMG7_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 7"]
pub mod ddrctrl_dramtmg7;
#[doc = "DDRCTRL_DRAMTMG8 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG8_SPEC>`"]
pub type DDRCTRL_DRAMTMG8 = crate::Reg<ddrctrl_dramtmg8::DDRCTRL_DRAMTMG8_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 8"]
pub mod ddrctrl_dramtmg8;
#[doc = "DDRCTRL_DRAMTMG14 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG14_SPEC>`"]
pub type DDRCTRL_DRAMTMG14 = crate::Reg<ddrctrl_dramtmg14::DDRCTRL_DRAMTMG14_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 14"]
pub mod ddrctrl_dramtmg14;
#[doc = "DDRCTRL_DRAMTMG15 (rw) register accessor: an alias for `Reg<DDRCTRL_DRAMTMG15_SPEC>`"]
pub type DDRCTRL_DRAMTMG15 = crate::Reg<ddrctrl_dramtmg15::DDRCTRL_DRAMTMG15_SPEC>;
#[doc = "DDRCTRL SDRAM timing register 15"]
pub mod ddrctrl_dramtmg15;
#[doc = "DDRCTRL_ZQCTL0 (rw) register accessor: an alias for `Reg<DDRCTRL_ZQCTL0_SPEC>`"]
pub type DDRCTRL_ZQCTL0 = crate::Reg<ddrctrl_zqctl0::DDRCTRL_ZQCTL0_SPEC>;
#[doc = "DDRCTRL ZQ control register 0"]
pub mod ddrctrl_zqctl0;
#[doc = "DDRCTRL_ZQCTL1 (rw) register accessor: an alias for `Reg<DDRCTRL_ZQCTL1_SPEC>`"]
pub type DDRCTRL_ZQCTL1 = crate::Reg<ddrctrl_zqctl1::DDRCTRL_ZQCTL1_SPEC>;
#[doc = "DDRCTRL ZQ control register 1"]
pub mod ddrctrl_zqctl1;
#[doc = "DDRCTRL_ZQCTL2 (rw) register accessor: an alias for `Reg<DDRCTRL_ZQCTL2_SPEC>`"]
pub type DDRCTRL_ZQCTL2 = crate::Reg<ddrctrl_zqctl2::DDRCTRL_ZQCTL2_SPEC>;
#[doc = "DDRCTRL ZQ control register 2"]
pub mod ddrctrl_zqctl2;
#[doc = "DDRCTRL_ZQSTAT (r) register accessor: an alias for `Reg<DDRCTRL_ZQSTAT_SPEC>`"]
pub type DDRCTRL_ZQSTAT = crate::Reg<ddrctrl_zqstat::DDRCTRL_ZQSTAT_SPEC>;
#[doc = "DDRCTRL ZQ status register"]
pub mod ddrctrl_zqstat;
#[doc = "DDRCTRL_DFITMG0 (rw) register accessor: an alias for `Reg<DDRCTRL_DFITMG0_SPEC>`"]
pub type DDRCTRL_DFITMG0 = crate::Reg<ddrctrl_dfitmg0::DDRCTRL_DFITMG0_SPEC>;
#[doc = "DDRCTRL DFI timing register 0"]
pub mod ddrctrl_dfitmg0;
#[doc = "DDRCTRL_DFITMG1 (rw) register accessor: an alias for `Reg<DDRCTRL_DFITMG1_SPEC>`"]
pub type DDRCTRL_DFITMG1 = crate::Reg<ddrctrl_dfitmg1::DDRCTRL_DFITMG1_SPEC>;
#[doc = "DDRCTRL DFI timing register 1"]
pub mod ddrctrl_dfitmg1;
#[doc = "DDRCTRL_DFILPCFG0 (rw) register accessor: an alias for `Reg<DDRCTRL_DFILPCFG0_SPEC>`"]
pub type DDRCTRL_DFILPCFG0 = crate::Reg<ddrctrl_dfilpcfg0::DDRCTRL_DFILPCFG0_SPEC>;
#[doc = "DDRCTRL low power configuration register 0"]
pub mod ddrctrl_dfilpcfg0;
#[doc = "DDRCTRL_DFIUPD0 (rw) register accessor: an alias for `Reg<DDRCTRL_DFIUPD0_SPEC>`"]
pub type DDRCTRL_DFIUPD0 = crate::Reg<ddrctrl_dfiupd0::DDRCTRL_DFIUPD0_SPEC>;
#[doc = "DDRCTRL DFI update register 0"]
pub mod ddrctrl_dfiupd0;
#[doc = "DDRCTRL_DFIUPD1 (rw) register accessor: an alias for `Reg<DDRCTRL_DFIUPD1_SPEC>`"]
pub type DDRCTRL_DFIUPD1 = crate::Reg<ddrctrl_dfiupd1::DDRCTRL_DFIUPD1_SPEC>;
#[doc = "DDRCTRL DFI update register 1"]
pub mod ddrctrl_dfiupd1;
#[doc = "DDRCTRL_DFIUPD2 (rw) register accessor: an alias for `Reg<DDRCTRL_DFIUPD2_SPEC>`"]
pub type DDRCTRL_DFIUPD2 = crate::Reg<ddrctrl_dfiupd2::DDRCTRL_DFIUPD2_SPEC>;
#[doc = "DDRCTRL DFI update register 2"]
pub mod ddrctrl_dfiupd2;
#[doc = "DDRCTRL_DFIMISC (rw) register accessor: an alias for `Reg<DDRCTRL_DFIMISC_SPEC>`"]
pub type DDRCTRL_DFIMISC = crate::Reg<ddrctrl_dfimisc::DDRCTRL_DFIMISC_SPEC>;
#[doc = "DDRCTRL DFI miscellaneous control register"]
pub mod ddrctrl_dfimisc;
#[doc = "DDRCTRL_DFISTAT (r) register accessor: an alias for `Reg<DDRCTRL_DFISTAT_SPEC>`"]
pub type DDRCTRL_DFISTAT = crate::Reg<ddrctrl_dfistat::DDRCTRL_DFISTAT_SPEC>;
#[doc = "DDRCTRL DFI status register"]
pub mod ddrctrl_dfistat;
#[doc = "DDRCTRL_DFIPHYMSTR (rw) register accessor: an alias for `Reg<DDRCTRL_DFIPHYMSTR_SPEC>`"]
pub type DDRCTRL_DFIPHYMSTR = crate::Reg<ddrctrl_dfiphymstr::DDRCTRL_DFIPHYMSTR_SPEC>;
#[doc = "DDRCTRL DFI PHY master register"]
pub mod ddrctrl_dfiphymstr;
#[doc = "DDRCTRL_ADDRMAP1 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP1_SPEC>`"]
pub type DDRCTRL_ADDRMAP1 = crate::Reg<ddrctrl_addrmap1::DDRCTRL_ADDRMAP1_SPEC>;
#[doc = "DDRCTRL address map register 1"]
pub mod ddrctrl_addrmap1;
#[doc = "DDRCTRL_ADDRMAP2 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP2_SPEC>`"]
pub type DDRCTRL_ADDRMAP2 = crate::Reg<ddrctrl_addrmap2::DDRCTRL_ADDRMAP2_SPEC>;
#[doc = "DDRCTRL address map register 2"]
pub mod ddrctrl_addrmap2;
#[doc = "DDRCTRL_ADDRMAP3 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP3_SPEC>`"]
pub type DDRCTRL_ADDRMAP3 = crate::Reg<ddrctrl_addrmap3::DDRCTRL_ADDRMAP3_SPEC>;
#[doc = "DDRCTRL address map register 3"]
pub mod ddrctrl_addrmap3;
#[doc = "DDRCTRL_ADDRMAP4 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP4_SPEC>`"]
pub type DDRCTRL_ADDRMAP4 = crate::Reg<ddrctrl_addrmap4::DDRCTRL_ADDRMAP4_SPEC>;
#[doc = "DDRCTRL address map register 4"]
pub mod ddrctrl_addrmap4;
#[doc = "DDRCTRL_ADDRMAP5 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP5_SPEC>`"]
pub type DDRCTRL_ADDRMAP5 = crate::Reg<ddrctrl_addrmap5::DDRCTRL_ADDRMAP5_SPEC>;
#[doc = "DDRCTRL address map register 5"]
pub mod ddrctrl_addrmap5;
#[doc = "DDRCTRL_ADDRMAP6 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP6_SPEC>`"]
pub type DDRCTRL_ADDRMAP6 = crate::Reg<ddrctrl_addrmap6::DDRCTRL_ADDRMAP6_SPEC>;
#[doc = "DDRCTRL address register 6"]
pub mod ddrctrl_addrmap6;
#[doc = "DDRCTRL_ADDRMAP9 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP9_SPEC>`"]
pub type DDRCTRL_ADDRMAP9 = crate::Reg<ddrctrl_addrmap9::DDRCTRL_ADDRMAP9_SPEC>;
#[doc = "DDRCTRL address map register 9"]
pub mod ddrctrl_addrmap9;
#[doc = "DDRCTRL_ADDRMAP10 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP10_SPEC>`"]
pub type DDRCTRL_ADDRMAP10 = crate::Reg<ddrctrl_addrmap10::DDRCTRL_ADDRMAP10_SPEC>;
#[doc = "DDRCTRL address map register 10"]
pub mod ddrctrl_addrmap10;
#[doc = "DDRCTRL_ADDRMAP11 (rw) register accessor: an alias for `Reg<DDRCTRL_ADDRMAP11_SPEC>`"]
pub type DDRCTRL_ADDRMAP11 = crate::Reg<ddrctrl_addrmap11::DDRCTRL_ADDRMAP11_SPEC>;
#[doc = "DDRCTRL address map register 11"]
pub mod ddrctrl_addrmap11;
#[doc = "DDRCTRL_ODTCFG (rw) register accessor: an alias for `Reg<DDRCTRL_ODTCFG_SPEC>`"]
pub type DDRCTRL_ODTCFG = crate::Reg<ddrctrl_odtcfg::DDRCTRL_ODTCFG_SPEC>;
#[doc = "DDRCTRL ODT configuration register"]
pub mod ddrctrl_odtcfg;
#[doc = "DDRCTRL_ODTMAP (rw) register accessor: an alias for `Reg<DDRCTRL_ODTMAP_SPEC>`"]
pub type DDRCTRL_ODTMAP = crate::Reg<ddrctrl_odtmap::DDRCTRL_ODTMAP_SPEC>;
#[doc = "DDRCTRL ODT/Rank map register"]
pub mod ddrctrl_odtmap;
#[doc = "DDRCTRL_SCHED (rw) register accessor: an alias for `Reg<DDRCTRL_SCHED_SPEC>`"]
pub type DDRCTRL_SCHED = crate::Reg<ddrctrl_sched::DDRCTRL_SCHED_SPEC>;
#[doc = "DDRCTRL scheduler control register"]
pub mod ddrctrl_sched;
#[doc = "DDRCTRL_SCHED1 (rw) register accessor: an alias for `Reg<DDRCTRL_SCHED1_SPEC>`"]
pub type DDRCTRL_SCHED1 = crate::Reg<ddrctrl_sched1::DDRCTRL_SCHED1_SPEC>;
#[doc = "DDRCTRL scheduler control register 1"]
pub mod ddrctrl_sched1;
#[doc = "DDRCTRL_PERFHPR1 (rw) register accessor: an alias for `Reg<DDRCTRL_PERFHPR1_SPEC>`"]
pub type DDRCTRL_PERFHPR1 = crate::Reg<ddrctrl_perfhpr1::DDRCTRL_PERFHPR1_SPEC>;
#[doc = "DDRCTRL high priority read CAM register 1"]
pub mod ddrctrl_perfhpr1;
#[doc = "DDRCTRL_PERFLPR1 (rw) register accessor: an alias for `Reg<DDRCTRL_PERFLPR1_SPEC>`"]
pub type DDRCTRL_PERFLPR1 = crate::Reg<ddrctrl_perflpr1::DDRCTRL_PERFLPR1_SPEC>;
#[doc = "DDRCTRL low priority read CAM register 1"]
pub mod ddrctrl_perflpr1;
#[doc = "DDRCTRL_PERFWR1 (rw) register accessor: an alias for `Reg<DDRCTRL_PERFWR1_SPEC>`"]
pub type DDRCTRL_PERFWR1 = crate::Reg<ddrctrl_perfwr1::DDRCTRL_PERFWR1_SPEC>;
#[doc = "DDRCTRL write CAM register 1"]
pub mod ddrctrl_perfwr1;
#[doc = "DDRCTRL_DBG0 (rw) register accessor: an alias for `Reg<DDRCTRL_DBG0_SPEC>`"]
pub type DDRCTRL_DBG0 = crate::Reg<ddrctrl_dbg0::DDRCTRL_DBG0_SPEC>;
#[doc = "DDRCTRL debug register 0"]
pub mod ddrctrl_dbg0;
#[doc = "DDRCTRL_DBG1 (rw) register accessor: an alias for `Reg<DDRCTRL_DBG1_SPEC>`"]
pub type DDRCTRL_DBG1 = crate::Reg<ddrctrl_dbg1::DDRCTRL_DBG1_SPEC>;
#[doc = "DDRCTRL debug register 1"]
pub mod ddrctrl_dbg1;
#[doc = "DDRCTRL_DBGCAM (r) register accessor: an alias for `Reg<DDRCTRL_DBGCAM_SPEC>`"]
pub type DDRCTRL_DBGCAM = crate::Reg<ddrctrl_dbgcam::DDRCTRL_DBGCAM_SPEC>;
#[doc = "DDRCTRL CAM debug register"]
pub mod ddrctrl_dbgcam;
#[doc = "DDRCTRL_DBGCMD (rw) register accessor: an alias for `Reg<DDRCTRL_DBGCMD_SPEC>`"]
pub type DDRCTRL_DBGCMD = crate::Reg<ddrctrl_dbgcmd::DDRCTRL_DBGCMD_SPEC>;
#[doc = "DDRCTRL command debug register"]
pub mod ddrctrl_dbgcmd;
#[doc = "DDRCTRL_DBGSTAT (r) register accessor: an alias for `Reg<DDRCTRL_DBGSTAT_SPEC>`"]
pub type DDRCTRL_DBGSTAT = crate::Reg<ddrctrl_dbgstat::DDRCTRL_DBGSTAT_SPEC>;
#[doc = "DDRCTRL status debug register"]
pub mod ddrctrl_dbgstat;
#[doc = "DDRCTRL_SWCTL (rw) register accessor: an alias for `Reg<DDRCTRL_SWCTL_SPEC>`"]
pub type DDRCTRL_SWCTL = crate::Reg<ddrctrl_swctl::DDRCTRL_SWCTL_SPEC>;
#[doc = "DDRCTRL software register programming control enable"]
pub mod ddrctrl_swctl;
#[doc = "DDRCTRL_SWSTAT (r) register accessor: an alias for `Reg<DDRCTRL_SWSTAT_SPEC>`"]
pub type DDRCTRL_SWSTAT = crate::Reg<ddrctrl_swstat::DDRCTRL_SWSTAT_SPEC>;
#[doc = "DDRCTRL software register programming control status"]
pub mod ddrctrl_swstat;
#[doc = "DDRCTRL_POISONCFG (rw) register accessor: an alias for `Reg<DDRCTRL_POISONCFG_SPEC>`"]
pub type DDRCTRL_POISONCFG = crate::Reg<ddrctrl_poisoncfg::DDRCTRL_POISONCFG_SPEC>;
#[doc = "AXI Poison configuration register common for all AXI ports."]
pub mod ddrctrl_poisoncfg;
#[doc = "DDRCTRL_POISONSTAT (r) register accessor: an alias for `Reg<DDRCTRL_POISONSTAT_SPEC>`"]
pub type DDRCTRL_POISONSTAT = crate::Reg<ddrctrl_poisonstat::DDRCTRL_POISONSTAT_SPEC>;
#[doc = "DDRCTRL AXI Poison status register"]
pub mod ddrctrl_poisonstat;
#[doc = "DDRCTRL_PSTAT (r) register accessor: an alias for `Reg<DDRCTRL_PSTAT_SPEC>`"]
pub type DDRCTRL_PSTAT = crate::Reg<ddrctrl_pstat::DDRCTRL_PSTAT_SPEC>;
#[doc = "DDRCTRL port status register"]
pub mod ddrctrl_pstat;
#[doc = "DDRCTRL_PCCFG (rw) register accessor: an alias for `Reg<DDRCTRL_PCCFG_SPEC>`"]
pub type DDRCTRL_PCCFG = crate::Reg<ddrctrl_pccfg::DDRCTRL_PCCFG_SPEC>;
#[doc = "DDRCTRL port common configuration register"]
pub mod ddrctrl_pccfg;
#[doc = "DDRCTRL_PCFGR_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGR_0_SPEC>`"]
pub type DDRCTRL_PCFGR_0 = crate::Reg<ddrctrl_pcfgr_0::DDRCTRL_PCFGR_0_SPEC>;
#[doc = "DDRCTRL port 0 configuration read register"]
pub mod ddrctrl_pcfgr_0;
#[doc = "DDRCTRL_PCFGW_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGW_0_SPEC>`"]
pub type DDRCTRL_PCFGW_0 = crate::Reg<ddrctrl_pcfgw_0::DDRCTRL_PCFGW_0_SPEC>;
#[doc = "DDRCTRL port 0 configuration write register"]
pub mod ddrctrl_pcfgw_0;
#[doc = "DDRCTRL_PCTRL_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCTRL_0_SPEC>`"]
pub type DDRCTRL_PCTRL_0 = crate::Reg<ddrctrl_pctrl_0::DDRCTRL_PCTRL_0_SPEC>;
#[doc = "DDRCTRL port 0 control register"]
pub mod ddrctrl_pctrl_0;
#[doc = "DDRCTRL_PCFGQOS0_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGQOS0_0_SPEC>`"]
pub type DDRCTRL_PCFGQOS0_0 = crate::Reg<ddrctrl_pcfgqos0_0::DDRCTRL_PCFGQOS0_0_SPEC>;
#[doc = "DDRCTRL port 0 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_0;
#[doc = "DDRCTRL_PCFGQOS1_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGQOS1_0_SPEC>`"]
pub type DDRCTRL_PCFGQOS1_0 = crate::Reg<ddrctrl_pcfgqos1_0::DDRCTRL_PCFGQOS1_0_SPEC>;
#[doc = "DDRCTRL port 0 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_0;
#[doc = "DDRCTRL_PCFGWQOS0_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGWQOS0_0_SPEC>`"]
pub type DDRCTRL_PCFGWQOS0_0 = crate::Reg<ddrctrl_pcfgwqos0_0::DDRCTRL_PCFGWQOS0_0_SPEC>;
#[doc = "DDRCTRL port 0 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_0;
#[doc = "DDRCTRL_PCFGWQOS1_0 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGWQOS1_0_SPEC>`"]
pub type DDRCTRL_PCFGWQOS1_0 = crate::Reg<ddrctrl_pcfgwqos1_0::DDRCTRL_PCFGWQOS1_0_SPEC>;
#[doc = "DDRCTRL port 0 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_0;
#[doc = "DDRCTRL_PCFGR_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGR_1_SPEC>`"]
pub type DDRCTRL_PCFGR_1 = crate::Reg<ddrctrl_pcfgr_1::DDRCTRL_PCFGR_1_SPEC>;
#[doc = "DDRCTRL port 1 configuration read register"]
pub mod ddrctrl_pcfgr_1;
#[doc = "DDRCTRL_PCFGW_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGW_1_SPEC>`"]
pub type DDRCTRL_PCFGW_1 = crate::Reg<ddrctrl_pcfgw_1::DDRCTRL_PCFGW_1_SPEC>;
#[doc = "DDRCTRL port 1 configuration write register"]
pub mod ddrctrl_pcfgw_1;
#[doc = "DDRCTRL_PCTRL_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCTRL_1_SPEC>`"]
pub type DDRCTRL_PCTRL_1 = crate::Reg<ddrctrl_pctrl_1::DDRCTRL_PCTRL_1_SPEC>;
#[doc = "DDRCTRL port 1 control register"]
pub mod ddrctrl_pctrl_1;
#[doc = "DDRCTRL_PCFGQOS0_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGQOS0_1_SPEC>`"]
pub type DDRCTRL_PCFGQOS0_1 = crate::Reg<ddrctrl_pcfgqos0_1::DDRCTRL_PCFGQOS0_1_SPEC>;
#[doc = "DDRCTRL port 1 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_1;
#[doc = "DDRCTRL_PCFGQOS1_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGQOS1_1_SPEC>`"]
pub type DDRCTRL_PCFGQOS1_1 = crate::Reg<ddrctrl_pcfgqos1_1::DDRCTRL_PCFGQOS1_1_SPEC>;
#[doc = "DDRCTRL port 1 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_1;
#[doc = "DDRCTRL_PCFGWQOS0_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGWQOS0_1_SPEC>`"]
pub type DDRCTRL_PCFGWQOS0_1 = crate::Reg<ddrctrl_pcfgwqos0_1::DDRCTRL_PCFGWQOS0_1_SPEC>;
#[doc = "DDRCTRL port 1 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_1;
#[doc = "DDRCTRL_PCFGWQOS1_1 (rw) register accessor: an alias for `Reg<DDRCTRL_PCFGWQOS1_1_SPEC>`"]
pub type DDRCTRL_PCFGWQOS1_1 = crate::Reg<ddrctrl_pcfgwqos1_1::DDRCTRL_PCFGWQOS1_1_SPEC>;
#[doc = "DDRCTRL port 1 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_1;
