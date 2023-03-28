#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch0cfgr1: DFSDM_CH0CFGR1,
    #[doc = "0x04 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch0cfgr2: DFSDM_CH0CFGR2,
    #[doc = "0x08 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch0awscdr: DFSDM_CH0AWSCDR,
    #[doc = "0x0c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch0wdatr: DFSDM_CH0WDATR,
    #[doc = "0x10 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch0datinr: DFSDM_CH0DATINR,
    #[doc = "0x14 - DFSDM channel 0 delay register"]
    pub dfsdm_ch0dlyr: DFSDM_CH0DLYR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch1cfgr1: DFSDM_CH1CFGR1,
    #[doc = "0x24 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch1cfgr2: DFSDM_CH1CFGR2,
    #[doc = "0x28 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch1awscdr: DFSDM_CH1AWSCDR,
    #[doc = "0x2c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch1wdatr: DFSDM_CH1WDATR,
    #[doc = "0x30 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch1datinr: DFSDM_CH1DATINR,
    #[doc = "0x34 - DFSDM channel 1 delay register"]
    pub dfsdm_ch1dlyr: DFSDM_CH1DLYR,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch2cfgr1: DFSDM_CH2CFGR1,
    #[doc = "0x44 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch2cfgr2: DFSDM_CH2CFGR2,
    #[doc = "0x48 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch2awscdr: DFSDM_CH2AWSCDR,
    #[doc = "0x4c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch2wdatr: DFSDM_CH2WDATR,
    #[doc = "0x50 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch2datinr: DFSDM_CH2DATINR,
    #[doc = "0x54 - DFSDM channel 2 delay register"]
    pub dfsdm_ch2dlyr: DFSDM_CH2DLYR,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch3cfgr1: DFSDM_CH3CFGR1,
    #[doc = "0x64 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch3cfgr2: DFSDM_CH3CFGR2,
    #[doc = "0x68 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch3awscdr: DFSDM_CH3AWSCDR,
    #[doc = "0x6c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch3wdatr: DFSDM_CH3WDATR,
    #[doc = "0x70 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch3datinr: DFSDM_CH3DATINR,
    #[doc = "0x74 - DFSDM channel 3 delay register"]
    pub dfsdm_ch3dlyr: DFSDM_CH3DLYR,
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch4cfgr1: DFSDM_CH4CFGR1,
    #[doc = "0x84 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch4cfgr2: DFSDM_CH4CFGR2,
    #[doc = "0x88 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch4awscdr: DFSDM_CH4AWSCDR,
    #[doc = "0x8c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch4wdatr: DFSDM_CH4WDATR,
    #[doc = "0x90 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch4datinr: DFSDM_CH4DATINR,
    #[doc = "0x94 - DFSDM channel 4 delay register"]
    pub dfsdm_ch4dlyr: DFSDM_CH4DLYR,
    _reserved30: [u8; 0x08],
    #[doc = "0xa0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch5cfgr1: DFSDM_CH5CFGR1,
    #[doc = "0xa4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch5cfgr2: DFSDM_CH5CFGR2,
    #[doc = "0xa8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch5awscdr: DFSDM_CH5AWSCDR,
    #[doc = "0xac - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch5wdatr: DFSDM_CH5WDATR,
    #[doc = "0xb0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch5datinr: DFSDM_CH5DATINR,
    #[doc = "0xb4 - DFSDM channel 5 delay register"]
    pub dfsdm_ch5dlyr: DFSDM_CH5DLYR,
    _reserved36: [u8; 0x08],
    #[doc = "0xc0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch6cfgr1: DFSDM_CH6CFGR1,
    #[doc = "0xc4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch6cfgr2: DFSDM_CH6CFGR2,
    #[doc = "0xc8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch6awscdr: DFSDM_CH6AWSCDR,
    #[doc = "0xcc - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch6wdatr: DFSDM_CH6WDATR,
    #[doc = "0xd0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch6datinr: DFSDM_CH6DATINR,
    #[doc = "0xd4 - DFSDM channel 6 delay register"]
    pub dfsdm_ch6dlyr: DFSDM_CH6DLYR,
    _reserved42: [u8; 0x08],
    #[doc = "0xe0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch7cfgr1: DFSDM_CH7CFGR1,
    #[doc = "0xe4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch7cfgr2: DFSDM_CH7CFGR2,
    #[doc = "0xe8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch7awscdr: DFSDM_CH7AWSCDR,
    #[doc = "0xec - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch7wdatr: DFSDM_CH7WDATR,
    #[doc = "0xf0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch7datinr: DFSDM_CH7DATINR,
    #[doc = "0xf4 - DFSDM channel 7 delay register"]
    pub dfsdm_ch7dlyr: DFSDM_CH7DLYR,
    _reserved48: [u8; 0x08],
    #[doc = "0x100 - DFSDM filter 0 control register 1"]
    pub dfsdm_flt0cr1: DFSDM_FLT0CR1,
    #[doc = "0x104 - DFSDM filter 0 control register 2"]
    pub dfsdm_flt0cr2: DFSDM_FLT0CR2,
    #[doc = "0x108 - DFSDM filter 0 interrupt and status register"]
    pub dfsdm_flt0isr: DFSDM_FLT0ISR,
    #[doc = "0x10c - DFSDM filter 0 interrupt flag clear register"]
    pub dfsdm_flt0icr: DFSDM_FLT0ICR,
    #[doc = "0x110 - DFSDM filter 0 injected channel group selection register"]
    pub dfsdm_flt0jchgr: DFSDM_FLT0JCHGR,
    #[doc = "0x114 - DFSDM filter 0 control register"]
    pub dfsdm_flt0fcr: DFSDM_FLT0FCR,
    #[doc = "0x118 - DFSDM filter 0 data register for injected group"]
    pub dfsdm_flt0jdatar: DFSDM_FLT0JDATAR,
    #[doc = "0x11c - DFSDM filter 0 data register for the regular channel"]
    pub dfsdm_flt0rdatar: DFSDM_FLT0RDATAR,
    #[doc = "0x120 - DFSDM filter 0 analog watchdog high threshold register"]
    pub dfsdm_flt0awhtr: DFSDM_FLT0AWHTR,
    #[doc = "0x124 - DFSDM filter 0 analog watchdog low threshold register"]
    pub dfsdm_flt0awltr: DFSDM_FLT0AWLTR,
    #[doc = "0x128 - DFSDM filter 0 analog watchdog status register"]
    pub dfsdm_flt0awsr: DFSDM_FLT0AWSR,
    #[doc = "0x12c - DFSDM filter 0 analog watchdog clear flag register"]
    pub dfsdm_flt0awcfr: DFSDM_FLT0AWCFR,
    #[doc = "0x130 - DFSDM filter 0 extremes detector maximum register"]
    pub dfsdm_flt0exmax: DFSDM_FLT0EXMAX,
    #[doc = "0x134 - DFSDM filter 0 extremes detector minimum register"]
    pub dfsdm_flt0exmin: DFSDM_FLT0EXMIN,
    #[doc = "0x138 - DFSDM filter 0 conversion timer register"]
    pub dfsdm_flt0cnvtimr: DFSDM_FLT0CNVTIMR,
    _reserved63: [u8; 0x44],
    #[doc = "0x180 - DFSDM filter 1 control register 1"]
    pub dfsdm_flt1cr1: DFSDM_FLT1CR1,
    #[doc = "0x184 - DFSDM filter 1 control register 2"]
    pub dfsdm_flt1cr2: DFSDM_FLT1CR2,
    #[doc = "0x188 - DFSDM filter 1 interrupt and status register"]
    pub dfsdm_flt1isr: DFSDM_FLT1ISR,
    #[doc = "0x18c - DFSDM filter 1 interrupt flag clear register"]
    pub dfsdm_flt1icr: DFSDM_FLT1ICR,
    #[doc = "0x190 - DFSDM filter 1 injected channel group selection register"]
    pub dfsdm_flt1jchgr: DFSDM_FLT1JCHGR,
    #[doc = "0x194 - DFSDM filter 1 control register"]
    pub dfsdm_flt1fcr: DFSDM_FLT1FCR,
    #[doc = "0x198 - DFSDM filter 1 data register for injected group"]
    pub dfsdm_flt1jdatar: DFSDM_FLT1JDATAR,
    #[doc = "0x19c - DFSDM filter 1 data register for the regular channel"]
    pub dfsdm_flt1rdatar: DFSDM_FLT1RDATAR,
    #[doc = "0x1a0 - DFSDM filter 1 analog watchdog high threshold register"]
    pub dfsdm_flt1awhtr: DFSDM_FLT1AWHTR,
    #[doc = "0x1a4 - DFSDM filter 1 analog watchdog low threshold register"]
    pub dfsdm_flt1awltr: DFSDM_FLT1AWLTR,
    #[doc = "0x1a8 - DFSDM filter 1 analog watchdog status register"]
    pub dfsdm_flt1awsr: DFSDM_FLT1AWSR,
    #[doc = "0x1ac - DFSDM filter 1 analog watchdog clear flag register"]
    pub dfsdm_flt1awcfr: DFSDM_FLT1AWCFR,
    #[doc = "0x1b0 - DFSDM filter 1 extremes detector maximum register"]
    pub dfsdm_flt1exmax: DFSDM_FLT1EXMAX,
    #[doc = "0x1b4 - DFSDM filter 1 extremes detector minimum register"]
    pub dfsdm_flt1exmin: DFSDM_FLT1EXMIN,
    #[doc = "0x1b8 - DFSDM filter 1 conversion timer register"]
    pub dfsdm_flt1cnvtimr: DFSDM_FLT1CNVTIMR,
    _reserved78: [u8; 0x44],
    #[doc = "0x200 - DFSDM filter 2 control register 1"]
    pub dfsdm_flt2cr1: DFSDM_FLT2CR1,
    #[doc = "0x204 - DFSDM filter 2 control register 2"]
    pub dfsdm_flt2cr2: DFSDM_FLT2CR2,
    #[doc = "0x208 - DFSDM filter 2 interrupt and status register"]
    pub dfsdm_flt2isr: DFSDM_FLT2ISR,
    #[doc = "0x20c - DFSDM filter 2 interrupt flag clear register"]
    pub dfsdm_flt2icr: DFSDM_FLT2ICR,
    #[doc = "0x210 - DFSDM filter 2 injected channel group selection register"]
    pub dfsdm_flt2jchgr: DFSDM_FLT2JCHGR,
    #[doc = "0x214 - DFSDM filter 2 control register"]
    pub dfsdm_flt2fcr: DFSDM_FLT2FCR,
    #[doc = "0x218 - DFSDM filter 2 data register for injected group"]
    pub dfsdm_flt2jdatar: DFSDM_FLT2JDATAR,
    #[doc = "0x21c - DFSDM filter 2 data register for the regular channel"]
    pub dfsdm_flt2rdatar: DFSDM_FLT2RDATAR,
    #[doc = "0x220 - DFSDM filter 2 analog watchdog high threshold register"]
    pub dfsdm_flt2awhtr: DFSDM_FLT2AWHTR,
    #[doc = "0x224 - DFSDM filter 2 analog watchdog low threshold register"]
    pub dfsdm_flt2awltr: DFSDM_FLT2AWLTR,
    #[doc = "0x228 - DFSDM filter 2 analog watchdog status register"]
    pub dfsdm_flt2awsr: DFSDM_FLT2AWSR,
    #[doc = "0x22c - DFSDM filter 2 analog watchdog clear flag register"]
    pub dfsdm_flt2awcfr: DFSDM_FLT2AWCFR,
    #[doc = "0x230 - DFSDM filter 2 extremes detector maximum register"]
    pub dfsdm_flt2exmax: DFSDM_FLT2EXMAX,
    #[doc = "0x234 - DFSDM filter 2 extremes detector minimum register"]
    pub dfsdm_flt2exmin: DFSDM_FLT2EXMIN,
    #[doc = "0x238 - DFSDM filter 2 conversion timer register"]
    pub dfsdm_flt2cnvtimr: DFSDM_FLT2CNVTIMR,
    _reserved93: [u8; 0x44],
    #[doc = "0x280 - DFSDM filter 3 control register 1"]
    pub dfsdm_flt3cr1: DFSDM_FLT3CR1,
    #[doc = "0x284 - DFSDM filter 3 control register 2"]
    pub dfsdm_flt3cr2: DFSDM_FLT3CR2,
    #[doc = "0x288 - DFSDM filter 3 interrupt and status register"]
    pub dfsdm_flt3isr: DFSDM_FLT3ISR,
    #[doc = "0x28c - DFSDM filter 3 interrupt flag clear register"]
    pub dfsdm_flt3icr: DFSDM_FLT3ICR,
    #[doc = "0x290 - DFSDM filter 3 injected channel group selection register"]
    pub dfsdm_flt3jchgr: DFSDM_FLT3JCHGR,
    #[doc = "0x294 - DFSDM filter 3 control register"]
    pub dfsdm_flt3fcr: DFSDM_FLT3FCR,
    #[doc = "0x298 - DFSDM filter 3 data register for injected group"]
    pub dfsdm_flt3jdatar: DFSDM_FLT3JDATAR,
    #[doc = "0x29c - DFSDM filter 3 data register for the regular channel"]
    pub dfsdm_flt3rdatar: DFSDM_FLT3RDATAR,
    #[doc = "0x2a0 - DFSDM filter 3 analog watchdog high threshold register"]
    pub dfsdm_flt3awhtr: DFSDM_FLT3AWHTR,
    #[doc = "0x2a4 - DFSDM filter 3 analog watchdog low threshold register"]
    pub dfsdm_flt3awltr: DFSDM_FLT3AWLTR,
    #[doc = "0x2a8 - DFSDM filter 3 analog watchdog status register"]
    pub dfsdm_flt3awsr: DFSDM_FLT3AWSR,
    #[doc = "0x2ac - DFSDM filter 3 analog watchdog clear flag register"]
    pub dfsdm_flt3awcfr: DFSDM_FLT3AWCFR,
    #[doc = "0x2b0 - DFSDM filter 3 extremes detector maximum register"]
    pub dfsdm_flt3exmax: DFSDM_FLT3EXMAX,
    #[doc = "0x2b4 - DFSDM filter 3 extremes detector minimum register"]
    pub dfsdm_flt3exmin: DFSDM_FLT3EXMIN,
    #[doc = "0x2b8 - DFSDM filter 3 conversion timer register"]
    pub dfsdm_flt3cnvtimr: DFSDM_FLT3CNVTIMR,
    _reserved108: [u8; 0x44],
    #[doc = "0x300 - DFSDM filter 4 control register 1"]
    pub dfsdm_flt4cr1: DFSDM_FLT4CR1,
    #[doc = "0x304 - DFSDM filter 4 control register 2"]
    pub dfsdm_flt4cr2: DFSDM_FLT4CR2,
    #[doc = "0x308 - DFSDM filter 4 interrupt and status register"]
    pub dfsdm_flt4isr: DFSDM_FLT4ISR,
    #[doc = "0x30c - DFSDM filter 4 interrupt flag clear register"]
    pub dfsdm_flt4icr: DFSDM_FLT4ICR,
    #[doc = "0x310 - DFSDM filter 4 injected channel group selection register"]
    pub dfsdm_flt4jchgr: DFSDM_FLT4JCHGR,
    #[doc = "0x314 - DFSDM filter 4 control register"]
    pub dfsdm_flt4fcr: DFSDM_FLT4FCR,
    #[doc = "0x318 - DFSDM filter 4 data register for injected group"]
    pub dfsdm_flt4jdatar: DFSDM_FLT4JDATAR,
    #[doc = "0x31c - DFSDM filter 4 data register for the regular channel"]
    pub dfsdm_flt4rdatar: DFSDM_FLT4RDATAR,
    #[doc = "0x320 - DFSDM filter 4 analog watchdog high threshold register"]
    pub dfsdm_flt4awhtr: DFSDM_FLT4AWHTR,
    #[doc = "0x324 - DFSDM filter 4 analog watchdog low threshold register"]
    pub dfsdm_flt4awltr: DFSDM_FLT4AWLTR,
    #[doc = "0x328 - DFSDM filter 4 analog watchdog status register"]
    pub dfsdm_flt4awsr: DFSDM_FLT4AWSR,
    #[doc = "0x32c - DFSDM filter 4 analog watchdog clear flag register"]
    pub dfsdm_flt4awcfr: DFSDM_FLT4AWCFR,
    #[doc = "0x330 - DFSDM filter 4 extremes detector maximum register"]
    pub dfsdm_flt4exmax: DFSDM_FLT4EXMAX,
    #[doc = "0x334 - DFSDM filter 4 extremes detector minimum register"]
    pub dfsdm_flt4exmin: DFSDM_FLT4EXMIN,
    #[doc = "0x338 - DFSDM filter 4 conversion timer register"]
    pub dfsdm_flt4cnvtimr: DFSDM_FLT4CNVTIMR,
    _reserved123: [u8; 0x44],
    #[doc = "0x380 - DFSDM filter 5 control register 1"]
    pub dfsdm_flt5cr1: DFSDM_FLT5CR1,
    #[doc = "0x384 - DFSDM filter 5 control register 2"]
    pub dfsdm_flt5cr2: DFSDM_FLT5CR2,
    #[doc = "0x388 - DFSDM filter 5 interrupt and status register"]
    pub dfsdm_flt5isr: DFSDM_FLT5ISR,
    #[doc = "0x38c - DFSDM filter 5 interrupt flag clear register"]
    pub dfsdm_flt5icr: DFSDM_FLT5ICR,
    #[doc = "0x390 - DFSDM filter 5 injected channel group selection register"]
    pub dfsdm_flt5jchgr: DFSDM_FLT5JCHGR,
    #[doc = "0x394 - DFSDM filter 5 control register"]
    pub dfsdm_flt5fcr: DFSDM_FLT5FCR,
    #[doc = "0x398 - DFSDM filter 5 data register for injected group"]
    pub dfsdm_flt5jdatar: DFSDM_FLT5JDATAR,
    #[doc = "0x39c - DFSDM filter 5 data register for the regular channel"]
    pub dfsdm_flt5rdatar: DFSDM_FLT5RDATAR,
    #[doc = "0x3a0 - DFSDM filter 5 analog watchdog high threshold register"]
    pub dfsdm_flt5awhtr: DFSDM_FLT5AWHTR,
    #[doc = "0x3a4 - DFSDM filter 5 analog watchdog low threshold register"]
    pub dfsdm_flt5awltr: DFSDM_FLT5AWLTR,
    #[doc = "0x3a8 - DFSDM filter 5 analog watchdog status register"]
    pub dfsdm_flt5awsr: DFSDM_FLT5AWSR,
    #[doc = "0x3ac - DFSDM filter 5 analog watchdog clear flag register"]
    pub dfsdm_flt5awcfr: DFSDM_FLT5AWCFR,
    #[doc = "0x3b0 - DFSDM filter 5 extremes detector maximum register"]
    pub dfsdm_flt5exmax: DFSDM_FLT5EXMAX,
    #[doc = "0x3b4 - DFSDM filter 5 extremes detector minimum register"]
    pub dfsdm_flt5exmin: DFSDM_FLT5EXMIN,
    #[doc = "0x3b8 - DFSDM filter 5 conversion timer register"]
    pub dfsdm_flt5cnvtimr: DFSDM_FLT5CNVTIMR,
    _reserved138: [u8; 0x0434],
    #[doc = "0x7f0 - This register specifies the hardware configuration of DFSDM peripheral."]
    pub dfsdm_hwcfgr: DFSDM_HWCFGR,
    #[doc = "0x7f4 - This register specifies the version of DFSDM peripheral."]
    pub dfsdm_verr: DFSDM_VERR,
    #[doc = "0x7f8 - This register specifies the identification of DFSDM peripheral."]
    pub dfsdm_ipidr: DFSDM_IPIDR,
    #[doc = "0x7fc - This register specifies the size allocated to DFSDM registers."]
    pub dfsdm_sidr: DFSDM_SIDR,
}
#[doc = "DFSDM_CH0CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH0CFGR1_SPEC>`"]
pub type DFSDM_CH0CFGR1 = crate::Reg<dfsdm_ch0cfgr1::DFSDM_CH0CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch0cfgr1;
#[doc = "DFSDM_CH0CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH0CFGR2_SPEC>`"]
pub type DFSDM_CH0CFGR2 = crate::Reg<dfsdm_ch0cfgr2::DFSDM_CH0CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch0cfgr2;
#[doc = "DFSDM_CH0AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH0AWSCDR_SPEC>`"]
pub type DFSDM_CH0AWSCDR = crate::Reg<dfsdm_ch0awscdr::DFSDM_CH0AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch0awscdr;
#[doc = "DFSDM_CH0WDATR (r) register accessor: an alias for `Reg<DFSDM_CH0WDATR_SPEC>`"]
pub type DFSDM_CH0WDATR = crate::Reg<dfsdm_ch0wdatr::DFSDM_CH0WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch0wdatr;
#[doc = "DFSDM_CH0DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH0DATINR_SPEC>`"]
pub type DFSDM_CH0DATINR = crate::Reg<dfsdm_ch0datinr::DFSDM_CH0DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch0datinr;
#[doc = "DFSDM_CH0DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH0DLYR_SPEC>`"]
pub type DFSDM_CH0DLYR = crate::Reg<dfsdm_ch0dlyr::DFSDM_CH0DLYR_SPEC>;
#[doc = "DFSDM channel 0 delay register"]
pub mod dfsdm_ch0dlyr;
#[doc = "DFSDM_CH1CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH1CFGR1_SPEC>`"]
pub type DFSDM_CH1CFGR1 = crate::Reg<dfsdm_ch1cfgr1::DFSDM_CH1CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch1cfgr1;
#[doc = "DFSDM_CH1CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH1CFGR2_SPEC>`"]
pub type DFSDM_CH1CFGR2 = crate::Reg<dfsdm_ch1cfgr2::DFSDM_CH1CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch1cfgr2;
#[doc = "DFSDM_CH1AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH1AWSCDR_SPEC>`"]
pub type DFSDM_CH1AWSCDR = crate::Reg<dfsdm_ch1awscdr::DFSDM_CH1AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch1awscdr;
#[doc = "DFSDM_CH1WDATR (r) register accessor: an alias for `Reg<DFSDM_CH1WDATR_SPEC>`"]
pub type DFSDM_CH1WDATR = crate::Reg<dfsdm_ch1wdatr::DFSDM_CH1WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch1wdatr;
#[doc = "DFSDM_CH1DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH1DATINR_SPEC>`"]
pub type DFSDM_CH1DATINR = crate::Reg<dfsdm_ch1datinr::DFSDM_CH1DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch1datinr;
#[doc = "DFSDM_CH1DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH1DLYR_SPEC>`"]
pub type DFSDM_CH1DLYR = crate::Reg<dfsdm_ch1dlyr::DFSDM_CH1DLYR_SPEC>;
#[doc = "DFSDM channel 1 delay register"]
pub mod dfsdm_ch1dlyr;
#[doc = "DFSDM_CH2CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH2CFGR1_SPEC>`"]
pub type DFSDM_CH2CFGR1 = crate::Reg<dfsdm_ch2cfgr1::DFSDM_CH2CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch2cfgr1;
#[doc = "DFSDM_CH2CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH2CFGR2_SPEC>`"]
pub type DFSDM_CH2CFGR2 = crate::Reg<dfsdm_ch2cfgr2::DFSDM_CH2CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch2cfgr2;
#[doc = "DFSDM_CH2AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH2AWSCDR_SPEC>`"]
pub type DFSDM_CH2AWSCDR = crate::Reg<dfsdm_ch2awscdr::DFSDM_CH2AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch2awscdr;
#[doc = "DFSDM_CH2WDATR (r) register accessor: an alias for `Reg<DFSDM_CH2WDATR_SPEC>`"]
pub type DFSDM_CH2WDATR = crate::Reg<dfsdm_ch2wdatr::DFSDM_CH2WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch2wdatr;
#[doc = "DFSDM_CH2DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH2DATINR_SPEC>`"]
pub type DFSDM_CH2DATINR = crate::Reg<dfsdm_ch2datinr::DFSDM_CH2DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch2datinr;
#[doc = "DFSDM_CH2DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH2DLYR_SPEC>`"]
pub type DFSDM_CH2DLYR = crate::Reg<dfsdm_ch2dlyr::DFSDM_CH2DLYR_SPEC>;
#[doc = "DFSDM channel 2 delay register"]
pub mod dfsdm_ch2dlyr;
#[doc = "DFSDM_CH3CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH3CFGR1_SPEC>`"]
pub type DFSDM_CH3CFGR1 = crate::Reg<dfsdm_ch3cfgr1::DFSDM_CH3CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch3cfgr1;
#[doc = "DFSDM_CH3CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH3CFGR2_SPEC>`"]
pub type DFSDM_CH3CFGR2 = crate::Reg<dfsdm_ch3cfgr2::DFSDM_CH3CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch3cfgr2;
#[doc = "DFSDM_CH3AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH3AWSCDR_SPEC>`"]
pub type DFSDM_CH3AWSCDR = crate::Reg<dfsdm_ch3awscdr::DFSDM_CH3AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch3awscdr;
#[doc = "DFSDM_CH3WDATR (r) register accessor: an alias for `Reg<DFSDM_CH3WDATR_SPEC>`"]
pub type DFSDM_CH3WDATR = crate::Reg<dfsdm_ch3wdatr::DFSDM_CH3WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch3wdatr;
#[doc = "DFSDM_CH3DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH3DATINR_SPEC>`"]
pub type DFSDM_CH3DATINR = crate::Reg<dfsdm_ch3datinr::DFSDM_CH3DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch3datinr;
#[doc = "DFSDM_CH3DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH3DLYR_SPEC>`"]
pub type DFSDM_CH3DLYR = crate::Reg<dfsdm_ch3dlyr::DFSDM_CH3DLYR_SPEC>;
#[doc = "DFSDM channel 3 delay register"]
pub mod dfsdm_ch3dlyr;
#[doc = "DFSDM_CH4CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH4CFGR1_SPEC>`"]
pub type DFSDM_CH4CFGR1 = crate::Reg<dfsdm_ch4cfgr1::DFSDM_CH4CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch4cfgr1;
#[doc = "DFSDM_CH4CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH4CFGR2_SPEC>`"]
pub type DFSDM_CH4CFGR2 = crate::Reg<dfsdm_ch4cfgr2::DFSDM_CH4CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch4cfgr2;
#[doc = "DFSDM_CH4AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH4AWSCDR_SPEC>`"]
pub type DFSDM_CH4AWSCDR = crate::Reg<dfsdm_ch4awscdr::DFSDM_CH4AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch4awscdr;
#[doc = "DFSDM_CH4WDATR (r) register accessor: an alias for `Reg<DFSDM_CH4WDATR_SPEC>`"]
pub type DFSDM_CH4WDATR = crate::Reg<dfsdm_ch4wdatr::DFSDM_CH4WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch4wdatr;
#[doc = "DFSDM_CH4DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH4DATINR_SPEC>`"]
pub type DFSDM_CH4DATINR = crate::Reg<dfsdm_ch4datinr::DFSDM_CH4DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch4datinr;
#[doc = "DFSDM_CH4DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH4DLYR_SPEC>`"]
pub type DFSDM_CH4DLYR = crate::Reg<dfsdm_ch4dlyr::DFSDM_CH4DLYR_SPEC>;
#[doc = "DFSDM channel 4 delay register"]
pub mod dfsdm_ch4dlyr;
#[doc = "DFSDM_CH5CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH5CFGR1_SPEC>`"]
pub type DFSDM_CH5CFGR1 = crate::Reg<dfsdm_ch5cfgr1::DFSDM_CH5CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch5cfgr1;
#[doc = "DFSDM_CH5CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH5CFGR2_SPEC>`"]
pub type DFSDM_CH5CFGR2 = crate::Reg<dfsdm_ch5cfgr2::DFSDM_CH5CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch5cfgr2;
#[doc = "DFSDM_CH5AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH5AWSCDR_SPEC>`"]
pub type DFSDM_CH5AWSCDR = crate::Reg<dfsdm_ch5awscdr::DFSDM_CH5AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch5awscdr;
#[doc = "DFSDM_CH5WDATR (r) register accessor: an alias for `Reg<DFSDM_CH5WDATR_SPEC>`"]
pub type DFSDM_CH5WDATR = crate::Reg<dfsdm_ch5wdatr::DFSDM_CH5WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch5wdatr;
#[doc = "DFSDM_CH5DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH5DATINR_SPEC>`"]
pub type DFSDM_CH5DATINR = crate::Reg<dfsdm_ch5datinr::DFSDM_CH5DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch5datinr;
#[doc = "DFSDM_CH5DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH5DLYR_SPEC>`"]
pub type DFSDM_CH5DLYR = crate::Reg<dfsdm_ch5dlyr::DFSDM_CH5DLYR_SPEC>;
#[doc = "DFSDM channel 5 delay register"]
pub mod dfsdm_ch5dlyr;
#[doc = "DFSDM_CH6CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH6CFGR1_SPEC>`"]
pub type DFSDM_CH6CFGR1 = crate::Reg<dfsdm_ch6cfgr1::DFSDM_CH6CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch6cfgr1;
#[doc = "DFSDM_CH6CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH6CFGR2_SPEC>`"]
pub type DFSDM_CH6CFGR2 = crate::Reg<dfsdm_ch6cfgr2::DFSDM_CH6CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch6cfgr2;
#[doc = "DFSDM_CH6AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH6AWSCDR_SPEC>`"]
pub type DFSDM_CH6AWSCDR = crate::Reg<dfsdm_ch6awscdr::DFSDM_CH6AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch6awscdr;
#[doc = "DFSDM_CH6WDATR (r) register accessor: an alias for `Reg<DFSDM_CH6WDATR_SPEC>`"]
pub type DFSDM_CH6WDATR = crate::Reg<dfsdm_ch6wdatr::DFSDM_CH6WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch6wdatr;
#[doc = "DFSDM_CH6DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH6DATINR_SPEC>`"]
pub type DFSDM_CH6DATINR = crate::Reg<dfsdm_ch6datinr::DFSDM_CH6DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch6datinr;
#[doc = "DFSDM_CH6DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH6DLYR_SPEC>`"]
pub type DFSDM_CH6DLYR = crate::Reg<dfsdm_ch6dlyr::DFSDM_CH6DLYR_SPEC>;
#[doc = "DFSDM channel 6 delay register"]
pub mod dfsdm_ch6dlyr;
#[doc = "DFSDM_CH7CFGR1 (rw) register accessor: an alias for `Reg<DFSDM_CH7CFGR1_SPEC>`"]
pub type DFSDM_CH7CFGR1 = crate::Reg<dfsdm_ch7cfgr1::DFSDM_CH7CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch7cfgr1;
#[doc = "DFSDM_CH7CFGR2 (rw) register accessor: an alias for `Reg<DFSDM_CH7CFGR2_SPEC>`"]
pub type DFSDM_CH7CFGR2 = crate::Reg<dfsdm_ch7cfgr2::DFSDM_CH7CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch7cfgr2;
#[doc = "DFSDM_CH7AWSCDR (rw) register accessor: an alias for `Reg<DFSDM_CH7AWSCDR_SPEC>`"]
pub type DFSDM_CH7AWSCDR = crate::Reg<dfsdm_ch7awscdr::DFSDM_CH7AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch7awscdr;
#[doc = "DFSDM_CH7WDATR (r) register accessor: an alias for `Reg<DFSDM_CH7WDATR_SPEC>`"]
pub type DFSDM_CH7WDATR = crate::Reg<dfsdm_ch7wdatr::DFSDM_CH7WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch7wdatr;
#[doc = "DFSDM_CH7DATINR (rw) register accessor: an alias for `Reg<DFSDM_CH7DATINR_SPEC>`"]
pub type DFSDM_CH7DATINR = crate::Reg<dfsdm_ch7datinr::DFSDM_CH7DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch7datinr;
#[doc = "DFSDM_CH7DLYR (rw) register accessor: an alias for `Reg<DFSDM_CH7DLYR_SPEC>`"]
pub type DFSDM_CH7DLYR = crate::Reg<dfsdm_ch7dlyr::DFSDM_CH7DLYR_SPEC>;
#[doc = "DFSDM channel 7 delay register"]
pub mod dfsdm_ch7dlyr;
#[doc = "DFSDM_FLT0CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT0CR1_SPEC>`"]
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>;
#[doc = "DFSDM filter 0 control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT0CR2_SPEC>`"]
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>;
#[doc = "DFSDM filter 0 control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR (r) register accessor: an alias for `Reg<DFSDM_FLT0ISR_SPEC>`"]
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>;
#[doc = "DFSDM filter 0 interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT0ICR_SPEC>`"]
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>;
#[doc = "DFSDM filter 0 interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT0JCHGR_SPEC>`"]
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>;
#[doc = "DFSDM filter 0 injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT0FCR_SPEC>`"]
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>;
#[doc = "DFSDM filter 0 control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT0JDATAR_SPEC>`"]
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>;
#[doc = "DFSDM filter 0 data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT0RDATAR_SPEC>`"]
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>;
#[doc = "DFSDM filter 0 data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWHTR_SPEC>`"]
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWLTR_SPEC>`"]
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT0AWSR_SPEC>`"]
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT0AWCFR_SPEC>`"]
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT0EXMAX_SPEC>`"]
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>;
#[doc = "DFSDM filter 0 extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT0EXMIN_SPEC>`"]
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>;
#[doc = "DFSDM filter 0 extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT0CNVTIMR_SPEC>`"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>;
#[doc = "DFSDM filter 0 conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT1CR1_SPEC>`"]
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>;
#[doc = "DFSDM filter 1 control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT1CR2_SPEC>`"]
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>;
#[doc = "DFSDM filter 1 control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR (r) register accessor: an alias for `Reg<DFSDM_FLT1ISR_SPEC>`"]
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>;
#[doc = "DFSDM filter 1 interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT1ICR_SPEC>`"]
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>;
#[doc = "DFSDM filter 1 interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT1JCHGR_SPEC>`"]
pub type DFSDM_FLT1JCHGR = crate::Reg<dfsdm_flt1jchgr::DFSDM_FLT1JCHGR_SPEC>;
#[doc = "DFSDM filter 1 injected channel group selection register"]
pub mod dfsdm_flt1jchgr;
#[doc = "DFSDM_FLT1FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT1FCR_SPEC>`"]
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>;
#[doc = "DFSDM filter 1 control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT1JDATAR_SPEC>`"]
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>;
#[doc = "DFSDM filter 1 data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT1RDATAR_SPEC>`"]
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>;
#[doc = "DFSDM filter 1 data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWHTR_SPEC>`"]
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWLTR_SPEC>`"]
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT1AWSR_SPEC>`"]
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT1AWCFR_SPEC>`"]
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT1EXMAX_SPEC>`"]
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>;
#[doc = "DFSDM filter 1 extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT1EXMIN_SPEC>`"]
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>;
#[doc = "DFSDM filter 1 extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT1CNVTIMR_SPEC>`"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>;
#[doc = "DFSDM filter 1 conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT2CR1_SPEC>`"]
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>;
#[doc = "DFSDM filter 2 control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT2CR2_SPEC>`"]
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>;
#[doc = "DFSDM filter 2 control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR (r) register accessor: an alias for `Reg<DFSDM_FLT2ISR_SPEC>`"]
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>;
#[doc = "DFSDM filter 2 interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT2ICR_SPEC>`"]
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>;
#[doc = "DFSDM filter 2 interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT2JCHGR_SPEC>`"]
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>;
#[doc = "DFSDM filter 2 injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT2FCR_SPEC>`"]
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>;
#[doc = "DFSDM filter 2 control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT2JDATAR_SPEC>`"]
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>;
#[doc = "DFSDM filter 2 data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT2RDATAR_SPEC>`"]
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>;
#[doc = "DFSDM filter 2 data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWHTR_SPEC>`"]
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWLTR_SPEC>`"]
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT2AWSR_SPEC>`"]
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT2AWCFR_SPEC>`"]
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT2EXMAX_SPEC>`"]
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>;
#[doc = "DFSDM filter 2 extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT2EXMIN_SPEC>`"]
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>;
#[doc = "DFSDM filter 2 extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT2CNVTIMR_SPEC>`"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>;
#[doc = "DFSDM filter 2 conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT3CR1_SPEC>`"]
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>;
#[doc = "DFSDM filter 3 control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT3CR2_SPEC>`"]
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>;
#[doc = "DFSDM filter 3 control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR (r) register accessor: an alias for `Reg<DFSDM_FLT3ISR_SPEC>`"]
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>;
#[doc = "DFSDM filter 3 interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT3ICR_SPEC>`"]
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>;
#[doc = "DFSDM filter 3 interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT3JCHGR_SPEC>`"]
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>;
#[doc = "DFSDM filter 3 injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT3FCR_SPEC>`"]
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>;
#[doc = "DFSDM filter 3 control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT3JDATAR_SPEC>`"]
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>;
#[doc = "DFSDM filter 3 data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT3RDATAR_SPEC>`"]
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>;
#[doc = "DFSDM filter 3 data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWHTR_SPEC>`"]
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWLTR_SPEC>`"]
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT3AWSR_SPEC>`"]
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT3AWCFR_SPEC>`"]
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT3EXMAX_SPEC>`"]
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>;
#[doc = "DFSDM filter 3 extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT3EXMIN_SPEC>`"]
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>;
#[doc = "DFSDM filter 3 extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT3CNVTIMR_SPEC>`"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>;
#[doc = "DFSDM filter 3 conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
#[doc = "DFSDM_FLT4CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT4CR1_SPEC>`"]
pub type DFSDM_FLT4CR1 = crate::Reg<dfsdm_flt4cr1::DFSDM_FLT4CR1_SPEC>;
#[doc = "DFSDM filter 4 control register 1"]
pub mod dfsdm_flt4cr1;
#[doc = "DFSDM_FLT4CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT4CR2_SPEC>`"]
pub type DFSDM_FLT4CR2 = crate::Reg<dfsdm_flt4cr2::DFSDM_FLT4CR2_SPEC>;
#[doc = "DFSDM filter 4 control register 2"]
pub mod dfsdm_flt4cr2;
#[doc = "DFSDM_FLT4ISR (r) register accessor: an alias for `Reg<DFSDM_FLT4ISR_SPEC>`"]
pub type DFSDM_FLT4ISR = crate::Reg<dfsdm_flt4isr::DFSDM_FLT4ISR_SPEC>;
#[doc = "DFSDM filter 4 interrupt and status register"]
pub mod dfsdm_flt4isr;
#[doc = "DFSDM_FLT4ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT4ICR_SPEC>`"]
pub type DFSDM_FLT4ICR = crate::Reg<dfsdm_flt4icr::DFSDM_FLT4ICR_SPEC>;
#[doc = "DFSDM filter 4 interrupt flag clear register"]
pub mod dfsdm_flt4icr;
#[doc = "DFSDM_FLT4JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT4JCHGR_SPEC>`"]
pub type DFSDM_FLT4JCHGR = crate::Reg<dfsdm_flt4jchgr::DFSDM_FLT4JCHGR_SPEC>;
#[doc = "DFSDM filter 4 injected channel group selection register"]
pub mod dfsdm_flt4jchgr;
#[doc = "DFSDM_FLT4FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT4FCR_SPEC>`"]
pub type DFSDM_FLT4FCR = crate::Reg<dfsdm_flt4fcr::DFSDM_FLT4FCR_SPEC>;
#[doc = "DFSDM filter 4 control register"]
pub mod dfsdm_flt4fcr;
#[doc = "DFSDM_FLT4JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT4JDATAR_SPEC>`"]
pub type DFSDM_FLT4JDATAR = crate::Reg<dfsdm_flt4jdatar::DFSDM_FLT4JDATAR_SPEC>;
#[doc = "DFSDM filter 4 data register for injected group"]
pub mod dfsdm_flt4jdatar;
#[doc = "DFSDM_FLT4RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT4RDATAR_SPEC>`"]
pub type DFSDM_FLT4RDATAR = crate::Reg<dfsdm_flt4rdatar::DFSDM_FLT4RDATAR_SPEC>;
#[doc = "DFSDM filter 4 data register for the regular channel"]
pub mod dfsdm_flt4rdatar;
#[doc = "DFSDM_FLT4AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT4AWHTR_SPEC>`"]
pub type DFSDM_FLT4AWHTR = crate::Reg<dfsdm_flt4awhtr::DFSDM_FLT4AWHTR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog high threshold register"]
pub mod dfsdm_flt4awhtr;
#[doc = "DFSDM_FLT4AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT4AWLTR_SPEC>`"]
pub type DFSDM_FLT4AWLTR = crate::Reg<dfsdm_flt4awltr::DFSDM_FLT4AWLTR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog low threshold register"]
pub mod dfsdm_flt4awltr;
#[doc = "DFSDM_FLT4AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT4AWSR_SPEC>`"]
pub type DFSDM_FLT4AWSR = crate::Reg<dfsdm_flt4awsr::DFSDM_FLT4AWSR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog status register"]
pub mod dfsdm_flt4awsr;
#[doc = "DFSDM_FLT4AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT4AWCFR_SPEC>`"]
pub type DFSDM_FLT4AWCFR = crate::Reg<dfsdm_flt4awcfr::DFSDM_FLT4AWCFR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog clear flag register"]
pub mod dfsdm_flt4awcfr;
#[doc = "DFSDM_FLT4EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT4EXMAX_SPEC>`"]
pub type DFSDM_FLT4EXMAX = crate::Reg<dfsdm_flt4exmax::DFSDM_FLT4EXMAX_SPEC>;
#[doc = "DFSDM filter 4 extremes detector maximum register"]
pub mod dfsdm_flt4exmax;
#[doc = "DFSDM_FLT4EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT4EXMIN_SPEC>`"]
pub type DFSDM_FLT4EXMIN = crate::Reg<dfsdm_flt4exmin::DFSDM_FLT4EXMIN_SPEC>;
#[doc = "DFSDM filter 4 extremes detector minimum register"]
pub mod dfsdm_flt4exmin;
#[doc = "DFSDM_FLT4CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT4CNVTIMR_SPEC>`"]
pub type DFSDM_FLT4CNVTIMR = crate::Reg<dfsdm_flt4cnvtimr::DFSDM_FLT4CNVTIMR_SPEC>;
#[doc = "DFSDM filter 4 conversion timer register"]
pub mod dfsdm_flt4cnvtimr;
#[doc = "DFSDM_FLT5CR1 (rw) register accessor: an alias for `Reg<DFSDM_FLT5CR1_SPEC>`"]
pub type DFSDM_FLT5CR1 = crate::Reg<dfsdm_flt5cr1::DFSDM_FLT5CR1_SPEC>;
#[doc = "DFSDM filter 5 control register 1"]
pub mod dfsdm_flt5cr1;
#[doc = "DFSDM_FLT5CR2 (rw) register accessor: an alias for `Reg<DFSDM_FLT5CR2_SPEC>`"]
pub type DFSDM_FLT5CR2 = crate::Reg<dfsdm_flt5cr2::DFSDM_FLT5CR2_SPEC>;
#[doc = "DFSDM filter 5 control register 2"]
pub mod dfsdm_flt5cr2;
#[doc = "DFSDM_FLT5ISR (r) register accessor: an alias for `Reg<DFSDM_FLT5ISR_SPEC>`"]
pub type DFSDM_FLT5ISR = crate::Reg<dfsdm_flt5isr::DFSDM_FLT5ISR_SPEC>;
#[doc = "DFSDM filter 5 interrupt and status register"]
pub mod dfsdm_flt5isr;
#[doc = "DFSDM_FLT5ICR (rw) register accessor: an alias for `Reg<DFSDM_FLT5ICR_SPEC>`"]
pub type DFSDM_FLT5ICR = crate::Reg<dfsdm_flt5icr::DFSDM_FLT5ICR_SPEC>;
#[doc = "DFSDM filter 5 interrupt flag clear register"]
pub mod dfsdm_flt5icr;
#[doc = "DFSDM_FLT5JCHGR (rw) register accessor: an alias for `Reg<DFSDM_FLT5JCHGR_SPEC>`"]
pub type DFSDM_FLT5JCHGR = crate::Reg<dfsdm_flt5jchgr::DFSDM_FLT5JCHGR_SPEC>;
#[doc = "DFSDM filter 5 injected channel group selection register"]
pub mod dfsdm_flt5jchgr;
#[doc = "DFSDM_FLT5FCR (rw) register accessor: an alias for `Reg<DFSDM_FLT5FCR_SPEC>`"]
pub type DFSDM_FLT5FCR = crate::Reg<dfsdm_flt5fcr::DFSDM_FLT5FCR_SPEC>;
#[doc = "DFSDM filter 5 control register"]
pub mod dfsdm_flt5fcr;
#[doc = "DFSDM_FLT5JDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT5JDATAR_SPEC>`"]
pub type DFSDM_FLT5JDATAR = crate::Reg<dfsdm_flt5jdatar::DFSDM_FLT5JDATAR_SPEC>;
#[doc = "DFSDM filter 5 data register for injected group"]
pub mod dfsdm_flt5jdatar;
#[doc = "DFSDM_FLT5RDATAR (r) register accessor: an alias for `Reg<DFSDM_FLT5RDATAR_SPEC>`"]
pub type DFSDM_FLT5RDATAR = crate::Reg<dfsdm_flt5rdatar::DFSDM_FLT5RDATAR_SPEC>;
#[doc = "DFSDM filter 5 data register for the regular channel"]
pub mod dfsdm_flt5rdatar;
#[doc = "DFSDM_FLT5AWHTR (rw) register accessor: an alias for `Reg<DFSDM_FLT5AWHTR_SPEC>`"]
pub type DFSDM_FLT5AWHTR = crate::Reg<dfsdm_flt5awhtr::DFSDM_FLT5AWHTR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog high threshold register"]
pub mod dfsdm_flt5awhtr;
#[doc = "DFSDM_FLT5AWLTR (rw) register accessor: an alias for `Reg<DFSDM_FLT5AWLTR_SPEC>`"]
pub type DFSDM_FLT5AWLTR = crate::Reg<dfsdm_flt5awltr::DFSDM_FLT5AWLTR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog low threshold register"]
pub mod dfsdm_flt5awltr;
#[doc = "DFSDM_FLT5AWSR (r) register accessor: an alias for `Reg<DFSDM_FLT5AWSR_SPEC>`"]
pub type DFSDM_FLT5AWSR = crate::Reg<dfsdm_flt5awsr::DFSDM_FLT5AWSR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog status register"]
pub mod dfsdm_flt5awsr;
#[doc = "DFSDM_FLT5AWCFR (rw) register accessor: an alias for `Reg<DFSDM_FLT5AWCFR_SPEC>`"]
pub type DFSDM_FLT5AWCFR = crate::Reg<dfsdm_flt5awcfr::DFSDM_FLT5AWCFR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog clear flag register"]
pub mod dfsdm_flt5awcfr;
#[doc = "DFSDM_FLT5EXMAX (r) register accessor: an alias for `Reg<DFSDM_FLT5EXMAX_SPEC>`"]
pub type DFSDM_FLT5EXMAX = crate::Reg<dfsdm_flt5exmax::DFSDM_FLT5EXMAX_SPEC>;
#[doc = "DFSDM filter 5 extremes detector maximum register"]
pub mod dfsdm_flt5exmax;
#[doc = "DFSDM_FLT5EXMIN (rw) register accessor: an alias for `Reg<DFSDM_FLT5EXMIN_SPEC>`"]
pub type DFSDM_FLT5EXMIN = crate::Reg<dfsdm_flt5exmin::DFSDM_FLT5EXMIN_SPEC>;
#[doc = "DFSDM filter 5 extremes detector minimum register"]
pub mod dfsdm_flt5exmin;
#[doc = "DFSDM_FLT5CNVTIMR (r) register accessor: an alias for `Reg<DFSDM_FLT5CNVTIMR_SPEC>`"]
pub type DFSDM_FLT5CNVTIMR = crate::Reg<dfsdm_flt5cnvtimr::DFSDM_FLT5CNVTIMR_SPEC>;
#[doc = "DFSDM filter 5 conversion timer register"]
pub mod dfsdm_flt5cnvtimr;
#[doc = "DFSDM_HWCFGR (r) register accessor: an alias for `Reg<DFSDM_HWCFGR_SPEC>`"]
pub type DFSDM_HWCFGR = crate::Reg<dfsdm_hwcfgr::DFSDM_HWCFGR_SPEC>;
#[doc = "This register specifies the hardware configuration of DFSDM peripheral."]
pub mod dfsdm_hwcfgr;
#[doc = "DFSDM_VERR (r) register accessor: an alias for `Reg<DFSDM_VERR_SPEC>`"]
pub type DFSDM_VERR = crate::Reg<dfsdm_verr::DFSDM_VERR_SPEC>;
#[doc = "This register specifies the version of DFSDM peripheral."]
pub mod dfsdm_verr;
#[doc = "DFSDM_IPIDR (r) register accessor: an alias for `Reg<DFSDM_IPIDR_SPEC>`"]
pub type DFSDM_IPIDR = crate::Reg<dfsdm_ipidr::DFSDM_IPIDR_SPEC>;
#[doc = "This register specifies the identification of DFSDM peripheral."]
pub mod dfsdm_ipidr;
#[doc = "DFSDM_SIDR (r) register accessor: an alias for `Reg<DFSDM_SIDR_SPEC>`"]
pub type DFSDM_SIDR = crate::Reg<dfsdm_sidr::DFSDM_SIDR_SPEC>;
#[doc = "This register specifies the size allocated to DFSDM registers."]
pub mod dfsdm_sidr;
