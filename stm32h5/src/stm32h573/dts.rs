#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Temperature sensor configuration register 1"]
    pub dts_cfgr1: DTS_CFGR1,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Temperature sensor T0 value register 1"]
    pub dts_t0valr1: DTS_T0VALR1,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Temperature sensor ramp value register"]
    pub dts_rampvalr: DTS_RAMPVALR,
    #[doc = "0x14 - Temperature sensor interrupt threshold register 1"]
    pub dts_itr1: DTS_ITR1,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - Temperature sensor data register"]
    pub dts_dr: DTS_DR,
    #[doc = "0x20 - Temperature sensor status register"]
    pub dts_sr: DTS_SR,
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    pub dts_itenr: DTS_ITENR,
    #[doc = "0x28 - Temperature sensor clear interrupt flag register"]
    pub dts_icifr: DTS_ICIFR,
    #[doc = "0x2c - Temperature sensor option register"]
    pub dts_or: DTS_OR,
}
#[doc = "DTS_CFGR1 (rw) register accessor: an alias for `Reg<DTS_CFGR1_SPEC>`"]
pub type DTS_CFGR1 = crate::Reg<dts_cfgr1::DTS_CFGR1_SPEC>;
#[doc = "Temperature sensor configuration register 1"]
pub mod dts_cfgr1;
#[doc = "DTS_T0VALR1 (r) register accessor: an alias for `Reg<DTS_T0VALR1_SPEC>`"]
pub type DTS_T0VALR1 = crate::Reg<dts_t0valr1::DTS_T0VALR1_SPEC>;
#[doc = "Temperature sensor T0 value register 1"]
pub mod dts_t0valr1;
#[doc = "DTS_RAMPVALR (r) register accessor: an alias for `Reg<DTS_RAMPVALR_SPEC>`"]
pub type DTS_RAMPVALR = crate::Reg<dts_rampvalr::DTS_RAMPVALR_SPEC>;
#[doc = "Temperature sensor ramp value register"]
pub mod dts_rampvalr;
#[doc = "DTS_ITR1 (rw) register accessor: an alias for `Reg<DTS_ITR1_SPEC>`"]
pub type DTS_ITR1 = crate::Reg<dts_itr1::DTS_ITR1_SPEC>;
#[doc = "Temperature sensor interrupt threshold register 1"]
pub mod dts_itr1;
#[doc = "DTS_DR (rw) register accessor: an alias for `Reg<DTS_DR_SPEC>`"]
pub type DTS_DR = crate::Reg<dts_dr::DTS_DR_SPEC>;
#[doc = "Temperature sensor data register"]
pub mod dts_dr;
#[doc = "DTS_SR (r) register accessor: an alias for `Reg<DTS_SR_SPEC>`"]
pub type DTS_SR = crate::Reg<dts_sr::DTS_SR_SPEC>;
#[doc = "Temperature sensor status register"]
pub mod dts_sr;
#[doc = "DTS_ITENR (rw) register accessor: an alias for `Reg<DTS_ITENR_SPEC>`"]
pub type DTS_ITENR = crate::Reg<dts_itenr::DTS_ITENR_SPEC>;
#[doc = "Temperature sensor interrupt enable register"]
pub mod dts_itenr;
#[doc = "DTS_ICIFR (rw) register accessor: an alias for `Reg<DTS_ICIFR_SPEC>`"]
pub type DTS_ICIFR = crate::Reg<dts_icifr::DTS_ICIFR_SPEC>;
#[doc = "Temperature sensor clear interrupt flag register"]
pub mod dts_icifr;
#[doc = "DTS_OR (rw) register accessor: an alias for `Reg<DTS_OR_SPEC>`"]
pub type DTS_OR = crate::Reg<dts_or::DTS_OR_SPEC>;
#[doc = "Temperature sensor option register"]
pub mod dts_or;
