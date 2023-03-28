#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    pub ch0cfgr1: CH0CFGR1,
    #[doc = "0x04 - channel configuration y register"]
    pub ch0cfgr2: CH0CFGR2,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub ch0awscdr: CH0AWSCDR,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub ch0wdatr: CH0WDATR,
    #[doc = "0x10 - channel data input register"]
    pub ch0datinr: CH0DATINR,
    #[doc = "0x14 - channel y delay register"]
    pub ch0dlyr: CH0DLYR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - CH1CFGR1"]
    pub ch1cfgr1: CH1CFGR1,
    #[doc = "0x24 - CH1CFGR2"]
    pub ch1cfgr2: CH1CFGR2,
    #[doc = "0x28 - CH1AWSCDR"]
    pub ch1awscdr: CH1AWSCDR,
    #[doc = "0x2c - CH1WDATR"]
    pub ch1wdatr: CH1WDATR,
    #[doc = "0x30 - CH1DATINR"]
    pub ch1datinr: CH1DATINR,
    #[doc = "0x34 - channel y delay register"]
    pub ch1dlyr: CH1DLYR,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - CH2CFGR1"]
    pub ch2cfgr1: CH2CFGR1,
    #[doc = "0x44 - CH2CFGR2"]
    pub ch2cfgr2: CH2CFGR2,
    #[doc = "0x48 - CH2AWSCDR"]
    pub ch2awscdr: CH2AWSCDR,
    #[doc = "0x4c - CH2WDATR"]
    pub ch2wdatr: CH2WDATR,
    #[doc = "0x50 - CH2DATINR"]
    pub ch2datinr: CH2DATINR,
    #[doc = "0x54 - channel y delay register"]
    pub ch2dlyr: CH2DLYR,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - CH3CFGR1"]
    pub ch3cfgr1: CH3CFGR1,
    #[doc = "0x64 - CH3CFGR2"]
    pub ch3cfgr2: CH3CFGR2,
    #[doc = "0x68 - CH3AWSCDR"]
    pub ch3awscdr: CH3AWSCDR,
    #[doc = "0x6c - CH3WDATR"]
    pub ch3wdatr: CH3WDATR,
    #[doc = "0x70 - CH3DATINR"]
    pub ch3datinr: CH3DATINR,
    #[doc = "0x74 - channel y delay register"]
    pub ch3dlyr: CH3DLYR,
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - CH4CFGR1"]
    pub ch4cfgr1: CH4CFGR1,
    #[doc = "0x84 - CH4CFGR2"]
    pub ch4cfgr2: CH4CFGR2,
    #[doc = "0x88 - CH4AWSCDR"]
    pub ch4awscdr: CH4AWSCDR,
    #[doc = "0x8c - CH4WDATR"]
    pub ch4wdatr: CH4WDATR,
    #[doc = "0x90 - CH4DATINR"]
    pub ch4datinr: CH4DATINR,
    #[doc = "0x94 - channel y delay register"]
    pub ch4dlyr: CH4DLYR,
    _reserved30: [u8; 0x08],
    #[doc = "0xa0 - CH5CFGR1"]
    pub ch5cfgr1: CH5CFGR1,
    #[doc = "0xa4 - CH5CFGR2"]
    pub ch5cfgr2: CH5CFGR2,
    #[doc = "0xa8 - CH5AWSCDR"]
    pub ch5awscdr: CH5AWSCDR,
    #[doc = "0xac - CH5WDATR"]
    pub ch5wdatr: CH5WDATR,
    #[doc = "0xb0 - CH5DATINR"]
    pub ch5datinr: CH5DATINR,
    #[doc = "0xb4 - channel y delay register"]
    pub ch5dlyr: CH5DLYR,
    _reserved36: [u8; 0x08],
    #[doc = "0xc0 - CH6CFGR1"]
    pub ch6cfgr1: CH6CFGR1,
    #[doc = "0xc4 - CH6CFGR2"]
    pub ch6cfgr2: CH6CFGR2,
    #[doc = "0xc8 - CH6AWSCDR"]
    pub ch6awscdr: CH6AWSCDR,
    #[doc = "0xcc - CH6WDATR"]
    pub ch6wdatr: CH6WDATR,
    #[doc = "0xd0 - CH6DATINR"]
    pub ch6datinr: CH6DATINR,
    #[doc = "0xd4 - channel y delay register"]
    pub ch6dlyr: CH6DLYR,
    _reserved42: [u8; 0x08],
    #[doc = "0xe0 - CH7CFGR1"]
    pub ch7cfgr1: CH7CFGR1,
    #[doc = "0xe4 - CH7CFGR2"]
    pub ch7cfgr2: CH7CFGR2,
    #[doc = "0xe8 - CH7AWSCDR"]
    pub ch7awscdr: CH7AWSCDR,
    #[doc = "0xec - CH7WDATR"]
    pub ch7wdatr: CH7WDATR,
    #[doc = "0xf0 - CH7DATINR"]
    pub ch7datinr: CH7DATINR,
    #[doc = "0xf4 - channel y delay register"]
    pub ch7dlyr: CH7DLYR,
    _reserved48: [u8; 0x08],
    #[doc = "0x100 - control register 1"]
    pub dfsdm_flt0cr1: DFSDM_FLT0CR1,
    #[doc = "0x104 - control register 2"]
    pub dfsdm_flt0cr2: DFSDM_FLT0CR2,
    #[doc = "0x108 - interrupt and status register"]
    pub dfsdm_flt0isr: DFSDM_FLT0ISR,
    #[doc = "0x10c - interrupt flag clear register"]
    pub dfsdm_flt0icr: DFSDM_FLT0ICR,
    #[doc = "0x110 - injected channel group selection register"]
    pub dfsdm_flt0jchgr: DFSDM_FLT0JCHGR,
    #[doc = "0x114 - filter control register"]
    pub dfsdm_flt0fcr: DFSDM_FLT0FCR,
    #[doc = "0x118 - data register for injected group"]
    pub dfsdm_flt0jdatar: DFSDM_FLT0JDATAR,
    #[doc = "0x11c - data register for the regular channel"]
    pub dfsdm_flt0rdatar: DFSDM_FLT0RDATAR,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub dfsdm_flt0awhtr: DFSDM_FLT0AWHTR,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub dfsdm_flt0awltr: DFSDM_FLT0AWLTR,
    #[doc = "0x128 - analog watchdog status register"]
    pub dfsdm_flt0awsr: DFSDM_FLT0AWSR,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub dfsdm_flt0awcfr: DFSDM_FLT0AWCFR,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub dfsdm_flt0exmax: DFSDM_FLT0EXMAX,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub dfsdm_flt0exmin: DFSDM_FLT0EXMIN,
    #[doc = "0x138 - conversion timer register"]
    pub dfsdm_flt0cnvtimr: DFSDM_FLT0CNVTIMR,
    _reserved63: [u8; 0x44],
    #[doc = "0x180 - control register 1"]
    pub dfsdm_flt1cr1: DFSDM_FLT1CR1,
    #[doc = "0x184 - control register 2"]
    pub dfsdm_flt1cr2: DFSDM_FLT1CR2,
    #[doc = "0x188 - interrupt and status register"]
    pub dfsdm_flt1isr: DFSDM_FLT1ISR,
    #[doc = "0x18c - interrupt flag clear register"]
    pub dfsdm_flt1icr: DFSDM_FLT1ICR,
    #[doc = "0x190 - injected channel group selection register"]
    pub dfsdm_flt1chgr: DFSDM_FLT1CHGR,
    #[doc = "0x194 - filter control register"]
    pub dfsdm_flt1fcr: DFSDM_FLT1FCR,
    #[doc = "0x198 - data register for injected group"]
    pub dfsdm_flt1jdatar: DFSDM_FLT1JDATAR,
    #[doc = "0x19c - data register for the regular channel"]
    pub dfsdm_flt1rdatar: DFSDM_FLT1RDATAR,
    #[doc = "0x1a0 - analog watchdog high threshold register"]
    pub dfsdm_flt1awhtr: DFSDM_FLT1AWHTR,
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    pub dfsdm_flt1awltr: DFSDM_FLT1AWLTR,
    #[doc = "0x1a8 - analog watchdog status register"]
    pub dfsdm_flt1awsr: DFSDM_FLT1AWSR,
    #[doc = "0x1ac - analog watchdog clear flag register"]
    pub dfsdm_flt1awcfr: DFSDM_FLT1AWCFR,
    #[doc = "0x1b0 - Extremes detector maximum register"]
    pub dfsdm_flt1exmax: DFSDM_FLT1EXMAX,
    #[doc = "0x1b4 - Extremes detector minimum register"]
    pub dfsdm_flt1exmin: DFSDM_FLT1EXMIN,
    #[doc = "0x1b8 - conversion timer register"]
    pub dfsdm_flt1cnvtimr: DFSDM_FLT1CNVTIMR,
    _reserved78: [u8; 0x44],
    #[doc = "0x200 - control register 1"]
    pub dfsdm_flt2cr1: DFSDM_FLT2CR1,
    #[doc = "0x204 - control register 2"]
    pub dfsdm_flt2cr2: DFSDM_FLT2CR2,
    #[doc = "0x208 - interrupt and status register"]
    pub dfsdm_flt2isr: DFSDM_FLT2ISR,
    #[doc = "0x20c - interrupt flag clear register"]
    pub dfsdm_flt2icr: DFSDM_FLT2ICR,
    #[doc = "0x210 - injected channel group selection register"]
    pub dfsdm_flt2jchgr: DFSDM_FLT2JCHGR,
    #[doc = "0x214 - filter control register"]
    pub dfsdm_flt2fcr: DFSDM_FLT2FCR,
    #[doc = "0x218 - data register for injected group"]
    pub dfsdm_flt2jdatar: DFSDM_FLT2JDATAR,
    #[doc = "0x21c - data register for the regular channel"]
    pub dfsdm_flt2rdatar: DFSDM_FLT2RDATAR,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub dfsdm_flt2awhtr: DFSDM_FLT2AWHTR,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub dfsdm_flt2awltr: DFSDM_FLT2AWLTR,
    #[doc = "0x228 - analog watchdog status register"]
    pub dfsdm_flt2awsr: DFSDM_FLT2AWSR,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub dfsdm_flt2awcfr: DFSDM_FLT2AWCFR,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub dfsdm_flt2exmax: DFSDM_FLT2EXMAX,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub dfsdm_flt2exmin: DFSDM_FLT2EXMIN,
    #[doc = "0x238 - conversion timer register"]
    pub dfsdm_flt2cnvtimr: DFSDM_FLT2CNVTIMR,
    _reserved93: [u8; 0x44],
    #[doc = "0x280 - control register 1"]
    pub dfsdm_flt3cr1: DFSDM_FLT3CR1,
    #[doc = "0x284 - control register 2"]
    pub dfsdm_flt3cr2: DFSDM_FLT3CR2,
    #[doc = "0x288 - interrupt and status register"]
    pub dfsdm_flt3isr: DFSDM_FLT3ISR,
    #[doc = "0x28c - interrupt flag clear register"]
    pub dfsdm_flt3icr: DFSDM_FLT3ICR,
    #[doc = "0x290 - injected channel group selection register"]
    pub dfsdm_flt3jchgr: DFSDM_FLT3JCHGR,
    #[doc = "0x294 - filter control register"]
    pub dfsdm_flt3fcr: DFSDM_FLT3FCR,
    #[doc = "0x298 - data register for injected group"]
    pub dfsdm_flt3jdatar: DFSDM_FLT3JDATAR,
    #[doc = "0x29c - data register for the regular channel"]
    pub dfsdm_flt3rdatar: DFSDM_FLT3RDATAR,
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    pub dfsdm_flt3awhtr: DFSDM_FLT3AWHTR,
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    pub dfsdm_flt3awltr: DFSDM_FLT3AWLTR,
    #[doc = "0x2a8 - analog watchdog status register"]
    pub dfsdm_flt3awsr: DFSDM_FLT3AWSR,
    #[doc = "0x2ac - analog watchdog clear flag register"]
    pub dfsdm_flt3awcfr: DFSDM_FLT3AWCFR,
    #[doc = "0x2b0 - Extremes detector maximum register"]
    pub dfsdm_flt3exmax: DFSDM_FLT3EXMAX,
    #[doc = "0x2b4 - Extremes detector minimum register"]
    pub dfsdm_flt3exmin: DFSDM_FLT3EXMIN,
    #[doc = "0x2b8 - conversion timer register"]
    pub dfsdm_flt3cnvtimr: DFSDM_FLT3CNVTIMR,
}
#[doc = "CH0CFGR1 (rw) register accessor: an alias for `Reg<CH0CFGR1_SPEC>`"]
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1_SPEC>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr1;
#[doc = "CH0CFGR2 (rw) register accessor: an alias for `Reg<CH0CFGR2_SPEC>`"]
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2_SPEC>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr2;
#[doc = "CH0AWSCDR (rw) register accessor: an alias for `Reg<CH0AWSCDR_SPEC>`"]
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDR_SPEC>;
#[doc = "analog watchdog and short-circuit detector register"]
pub mod ch0awscdr;
#[doc = "CH0WDATR (rw) register accessor: an alias for `Reg<CH0WDATR_SPEC>`"]
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATR_SPEC>;
#[doc = "channel watchdog filter data register"]
pub mod ch0wdatr;
#[doc = "CH0DATINR (rw) register accessor: an alias for `Reg<CH0DATINR_SPEC>`"]
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINR_SPEC>;
#[doc = "channel data input register"]
pub mod ch0datinr;
#[doc = "CH0DLYR (rw) register accessor: an alias for `Reg<CH0DLYR_SPEC>`"]
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch0dlyr;
#[doc = "CH1CFGR1 (rw) register accessor: an alias for `Reg<CH1CFGR1_SPEC>`"]
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>;
#[doc = "CH1CFGR1"]
pub mod ch1cfgr1;
#[doc = "CH1CFGR2 (rw) register accessor: an alias for `Reg<CH1CFGR2_SPEC>`"]
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>;
#[doc = "CH1CFGR2"]
pub mod ch1cfgr2;
#[doc = "CH1AWSCDR (rw) register accessor: an alias for `Reg<CH1AWSCDR_SPEC>`"]
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>;
#[doc = "CH1AWSCDR"]
pub mod ch1awscdr;
#[doc = "CH1WDATR (rw) register accessor: an alias for `Reg<CH1WDATR_SPEC>`"]
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATR_SPEC>;
#[doc = "CH1WDATR"]
pub mod ch1wdatr;
#[doc = "CH1DATINR (rw) register accessor: an alias for `Reg<CH1DATINR_SPEC>`"]
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINR_SPEC>;
#[doc = "CH1DATINR"]
pub mod ch1datinr;
#[doc = "CH1DLYR (rw) register accessor: an alias for `Reg<CH1DLYR_SPEC>`"]
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch1dlyr;
#[doc = "CH2CFGR1 (rw) register accessor: an alias for `Reg<CH2CFGR1_SPEC>`"]
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>;
#[doc = "CH2CFGR1"]
pub mod ch2cfgr1;
#[doc = "CH2CFGR2 (rw) register accessor: an alias for `Reg<CH2CFGR2_SPEC>`"]
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>;
#[doc = "CH2CFGR2"]
pub mod ch2cfgr2;
#[doc = "CH2AWSCDR (rw) register accessor: an alias for `Reg<CH2AWSCDR_SPEC>`"]
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>;
#[doc = "CH2AWSCDR"]
pub mod ch2awscdr;
#[doc = "CH2WDATR (rw) register accessor: an alias for `Reg<CH2WDATR_SPEC>`"]
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATR_SPEC>;
#[doc = "CH2WDATR"]
pub mod ch2wdatr;
#[doc = "CH2DATINR (rw) register accessor: an alias for `Reg<CH2DATINR_SPEC>`"]
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINR_SPEC>;
#[doc = "CH2DATINR"]
pub mod ch2datinr;
#[doc = "CH2DLYR (rw) register accessor: an alias for `Reg<CH2DLYR_SPEC>`"]
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch2dlyr;
#[doc = "CH3CFGR1 (rw) register accessor: an alias for `Reg<CH3CFGR1_SPEC>`"]
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>;
#[doc = "CH3CFGR1"]
pub mod ch3cfgr1;
#[doc = "CH3CFGR2 (rw) register accessor: an alias for `Reg<CH3CFGR2_SPEC>`"]
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>;
#[doc = "CH3CFGR2"]
pub mod ch3cfgr2;
#[doc = "CH3AWSCDR (rw) register accessor: an alias for `Reg<CH3AWSCDR_SPEC>`"]
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>;
#[doc = "CH3AWSCDR"]
pub mod ch3awscdr;
#[doc = "CH3WDATR (rw) register accessor: an alias for `Reg<CH3WDATR_SPEC>`"]
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATR_SPEC>;
#[doc = "CH3WDATR"]
pub mod ch3wdatr;
#[doc = "CH3DATINR (rw) register accessor: an alias for `Reg<CH3DATINR_SPEC>`"]
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINR_SPEC>;
#[doc = "CH3DATINR"]
pub mod ch3datinr;
#[doc = "CH3DLYR (rw) register accessor: an alias for `Reg<CH3DLYR_SPEC>`"]
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch3dlyr;
#[doc = "CH4CFGR1 (rw) register accessor: an alias for `Reg<CH4CFGR1_SPEC>`"]
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>;
#[doc = "CH4CFGR1"]
pub mod ch4cfgr1;
#[doc = "CH4CFGR2 (rw) register accessor: an alias for `Reg<CH4CFGR2_SPEC>`"]
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>;
#[doc = "CH4CFGR2"]
pub mod ch4cfgr2;
#[doc = "CH4AWSCDR (rw) register accessor: an alias for `Reg<CH4AWSCDR_SPEC>`"]
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>;
#[doc = "CH4AWSCDR"]
pub mod ch4awscdr;
#[doc = "CH4WDATR (rw) register accessor: an alias for `Reg<CH4WDATR_SPEC>`"]
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATR_SPEC>;
#[doc = "CH4WDATR"]
pub mod ch4wdatr;
#[doc = "CH4DATINR (rw) register accessor: an alias for `Reg<CH4DATINR_SPEC>`"]
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINR_SPEC>;
#[doc = "CH4DATINR"]
pub mod ch4datinr;
#[doc = "CH4DLYR (rw) register accessor: an alias for `Reg<CH4DLYR_SPEC>`"]
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch4dlyr;
#[doc = "CH5CFGR1 (rw) register accessor: an alias for `Reg<CH5CFGR1_SPEC>`"]
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>;
#[doc = "CH5CFGR1"]
pub mod ch5cfgr1;
#[doc = "CH5CFGR2 (rw) register accessor: an alias for `Reg<CH5CFGR2_SPEC>`"]
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>;
#[doc = "CH5CFGR2"]
pub mod ch5cfgr2;
#[doc = "CH5AWSCDR (rw) register accessor: an alias for `Reg<CH5AWSCDR_SPEC>`"]
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>;
#[doc = "CH5AWSCDR"]
pub mod ch5awscdr;
#[doc = "CH5WDATR (rw) register accessor: an alias for `Reg<CH5WDATR_SPEC>`"]
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATR_SPEC>;
#[doc = "CH5WDATR"]
pub mod ch5wdatr;
#[doc = "CH5DATINR (rw) register accessor: an alias for `Reg<CH5DATINR_SPEC>`"]
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINR_SPEC>;
#[doc = "CH5DATINR"]
pub mod ch5datinr;
#[doc = "CH5DLYR (rw) register accessor: an alias for `Reg<CH5DLYR_SPEC>`"]
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch5dlyr;
#[doc = "CH6CFGR1 (rw) register accessor: an alias for `Reg<CH6CFGR1_SPEC>`"]
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>;
#[doc = "CH6CFGR1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2 (rw) register accessor: an alias for `Reg<CH6CFGR2_SPEC>`"]
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>;
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "CH6AWSCDR (rw) register accessor: an alias for `Reg<CH6AWSCDR_SPEC>`"]
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>;
#[doc = "CH6AWSCDR"]
pub mod ch6awscdr;
#[doc = "CH6WDATR (rw) register accessor: an alias for `Reg<CH6WDATR_SPEC>`"]
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATR_SPEC>;
#[doc = "CH6WDATR"]
pub mod ch6wdatr;
#[doc = "CH6DATINR (rw) register accessor: an alias for `Reg<CH6DATINR_SPEC>`"]
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINR_SPEC>;
#[doc = "CH6DATINR"]
pub mod ch6datinr;
#[doc = "CH6DLYR (rw) register accessor: an alias for `Reg<CH6DLYR_SPEC>`"]
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch6dlyr;
#[doc = "CH7CFGR1 (rw) register accessor: an alias for `Reg<CH7CFGR1_SPEC>`"]
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>;
#[doc = "CH7CFGR1"]
pub mod ch7cfgr1;
#[doc = "CH7CFGR2 (rw) register accessor: an alias for `Reg<CH7CFGR2_SPEC>`"]
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>;
#[doc = "CH7CFGR2"]
pub mod ch7cfgr2;
#[doc = "CH7AWSCDR (rw) register accessor: an alias for `Reg<CH7AWSCDR_SPEC>`"]
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>;
#[doc = "CH7AWSCDR"]
pub mod ch7awscdr;
#[doc = "CH7DLYR (rw) register accessor: an alias for `Reg<CH7DLYR_SPEC>`"]
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch7dlyr;
#[doc = "DFSDM_FLT0CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT0CR1_SPEC>`"]
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT0CR2_SPEC>`"]
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR (r) register accessor: an alias for `Reg<DFSDM_FLT0ISR_SPEC>`"]
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT0ICR_SPEC>`"]
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT0JCHGR_SPEC>`"]
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT0FCR_SPEC>`"]
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT0JDATAR_SPEC>`"]
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT0RDATAR_SPEC>`"]
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWHTR_SPEC>`"]
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWLTR_SPEC>`"]
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT0AWSR_SPEC>`"]
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWCFR_SPEC>`"]
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT0EXMAX_SPEC>`"]
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN (r) register accessor: an alias for `Reg<DFSDM_FLT0EXMIN_SPEC>`"]
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT0CNVTIMR_SPEC>`"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT1CR1_SPEC>`"]
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT1CR2_SPEC>`"]
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR (r) register accessor: an alias for `Reg<DFSDM_FLT1ISR_SPEC>`"]
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT1ICR_SPEC>`"]
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1CHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT1CHGR_SPEC>`"]
pub type DFSDM_FLT1CHGR = crate::Reg<dfsdm_flt1chgr::DFSDM_FLT1CHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt1chgr;
#[doc = "DFSDM_FLT1FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT1FCR_SPEC>`"]
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT1JDATAR_SPEC>`"]
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT1RDATAR_SPEC>`"]
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWHTR_SPEC>`"]
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWLTR_SPEC>`"]
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT1AWSR_SPEC>`"]
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWCFR_SPEC>`"]
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT1EXMAX_SPEC>`"]
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN (r) register accessor: an alias for `Reg<DFSDM_FLT1EXMIN_SPEC>`"]
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT1CNVTIMR_SPEC>`"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT2CR1_SPEC>`"]
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT2CR2_SPEC>`"]
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR (r) register accessor: an alias for `Reg<DFSDM_FLT2ISR_SPEC>`"]
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT2ICR_SPEC>`"]
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT2JCHGR_SPEC>`"]
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT2FCR_SPEC>`"]
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT2JDATAR_SPEC>`"]
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT2RDATAR_SPEC>`"]
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWHTR_SPEC>`"]
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWLTR_SPEC>`"]
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT2AWSR_SPEC>`"]
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWCFR_SPEC>`"]
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT2EXMAX_SPEC>`"]
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN (r) register accessor: an alias for `Reg<DFSDM_FLT2EXMIN_SPEC>`"]
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT2CNVTIMR_SPEC>`"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT3CR1_SPEC>`"]
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT3CR2_SPEC>`"]
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR (r) register accessor: an alias for `Reg<DFSDM_FLT3ISR_SPEC>`"]
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT3ICR_SPEC>`"]
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT3JCHGR_SPEC>`"]
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT3FCR_SPEC>`"]
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT3JDATAR_SPEC>`"]
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT3RDATAR_SPEC>`"]
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWHTR_SPEC>`"]
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWLTR_SPEC>`"]
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT3AWSR_SPEC>`"]
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWCFR_SPEC>`"]
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT3EXMAX_SPEC>`"]
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN (r) register accessor: an alias for `Reg<DFSDM_FLT3EXMIN_SPEC>`"]
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT3CNVTIMR_SPEC>`"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
#[doc = "CH7WDATR (rw) register accessor: an alias for `Reg<CH7WDATR_SPEC>`"]
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATR_SPEC>;
#[doc = "CH7WDATR"]
pub mod ch7wdatr;
#[doc = "CH7DATINR (rw) register accessor: an alias for `Reg<CH7DATINR_SPEC>`"]
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINR_SPEC>;
#[doc = "CH7DATINR"]
pub mod ch7datinr;
