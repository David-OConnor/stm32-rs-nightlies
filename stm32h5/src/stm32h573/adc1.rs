#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - ADC control register"]
    pub cr: CR,
    _reserved_3_cdr: [u8; 0x04],
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - ADC sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x18 - ADC sample time register 2"]
    pub smpr2: SMPR2,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - ADC watchdog threshold register 1"]
    pub tr1: TR1,
    #[doc = "0x24 - ADC watchdog threshold register 2"]
    pub tr2: TR2,
    #[doc = "0x28 - ADC watchdog threshold register 3"]
    pub tr3: TR3,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - ADC regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x34 - ADC regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x38 - ADC regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x3c - ADC regular sequence register 4"]
    pub sqr4: SQR4,
    #[doc = "0x40 - ADC regular data register"]
    pub dr: DR,
    _reserved15: [u8; 0x08],
    #[doc = "0x4c - ADC injected sequence register"]
    pub jsqr: JSQR,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - ADC offset 1 register"]
    pub ofr1: OFR1,
    #[doc = "0x64 - ADC offset 2 register"]
    pub ofr2: OFR2,
    #[doc = "0x68 - ADC offset 3 register"]
    pub ofr3: OFR3,
    #[doc = "0x6c - ADC offset 4 register"]
    pub ofr4: OFR4,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - ADC injected channel 1 data register"]
    pub jdr1: JDR1,
    #[doc = "0x84 - ADC injected channel 2 data register"]
    pub jdr2: JDR2,
    #[doc = "0x88 - ADC injected channel 3 data register"]
    pub jdr3: JDR3,
    #[doc = "0x8c - ADC injected channel 4 data register"]
    pub jdr4: JDR4,
    _reserved24: [u8; 0x10],
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration Register"]
    pub awd2cr: AWD2CR,
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration Register"]
    pub awd3cr: AWD3CR,
    _reserved26: [u8; 0x08],
    #[doc = "0xb0 - ADC Differential mode Selection Register"]
    pub difsel: DIFSEL,
    #[doc = "0xb4 - ADC Calibration Factors"]
    pub calfact: CALFACT,
    _reserved28: [u8; 0x10],
    #[doc = "0xc8 - ADC option register"]
    pub or: OR,
    _reserved29: [u8; 0x0234],
    #[doc = "0x300 - ADC common status register"]
    pub csr: CSR,
    _reserved30: [u8; 0x04],
    #[doc = "0x308 - ADC common control register"]
    pub ccr: CCR,
    _reserved31: [u8; 0xe4],
    #[doc = "0x3f0 - ADC hardware configuration register"]
    pub hwcfgr0: HWCFGR0,
    #[doc = "0x3f4 - ADC version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - ADC identification register"]
    pub ipdr: IPDR,
    #[doc = "0x3fc - ADC size identification register"]
    pub sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x0c - ADC_CDR_mode"]
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - ADC configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
}
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "ADC configuration register"]
pub mod cfgr;
#[doc = "CDR (r) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "ADC_CDR_mode"]
pub mod cdr;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "ADC sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "ADC sample time register 2"]
pub mod smpr2;
#[doc = "TR1 (rw) register accessor: an alias for `Reg<TR1_SPEC>`"]
pub type TR1 = crate::Reg<tr1::TR1_SPEC>;
#[doc = "ADC watchdog threshold register 1"]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: an alias for `Reg<TR2_SPEC>`"]
pub type TR2 = crate::Reg<tr2::TR2_SPEC>;
#[doc = "ADC watchdog threshold register 2"]
pub mod tr2;
#[doc = "TR3 (rw) register accessor: an alias for `Reg<TR3_SPEC>`"]
pub type TR3 = crate::Reg<tr3::TR3_SPEC>;
#[doc = "ADC watchdog threshold register 3"]
pub mod tr3;
#[doc = "SQR1 (rw) register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "ADC regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "ADC regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "ADC regular sequence register 3"]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: an alias for `Reg<SQR4_SPEC>`"]
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
#[doc = "ADC regular sequence register 4"]
pub mod sqr4;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ADC regular data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "ADC injected sequence register"]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: an alias for `Reg<OFR1_SPEC>`"]
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
#[doc = "ADC offset 1 register"]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: an alias for `Reg<OFR2_SPEC>`"]
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
#[doc = "ADC offset 2 register"]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: an alias for `Reg<OFR3_SPEC>`"]
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
#[doc = "ADC offset 3 register"]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: an alias for `Reg<OFR4_SPEC>`"]
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
#[doc = "ADC offset 4 register"]
pub mod ofr4;
#[doc = "JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "ADC injected channel 1 data register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "ADC injected channel 2 data register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "ADC injected channel 3 data register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: an alias for `Reg<JDR4_SPEC>`"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "ADC injected channel 4 data register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: an alias for `Reg<AWD2CR_SPEC>`"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
#[doc = "ADC Analog Watchdog 2 Configuration Register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: an alias for `Reg<AWD3CR_SPEC>`"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
#[doc = "ADC Analog Watchdog 3 Configuration Register"]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: an alias for `Reg<DIFSEL_SPEC>`"]
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
#[doc = "ADC Differential mode Selection Register"]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: an alias for `Reg<CALFACT_SPEC>`"]
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
#[doc = "ADC Calibration Factors"]
pub mod calfact;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "ADC option register"]
pub mod or;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC common status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "HWCFGR0 (r) register accessor: an alias for `Reg<HWCFGR0_SPEC>`"]
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0_SPEC>;
#[doc = "ADC hardware configuration register"]
pub mod hwcfgr0;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "ADC version register"]
pub mod verr;
#[doc = "IPDR (r) register accessor: an alias for `Reg<IPDR_SPEC>`"]
pub type IPDR = crate::Reg<ipdr::IPDR_SPEC>;
#[doc = "ADC identification register"]
pub mod ipdr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "ADC size identification register"]
pub mod sidr;