#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - software trigger register"]
    pub swtrigr: SWTRIGR,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    pub dhr12r1: DHR12R1,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    pub dhr12l1: DHR12L1,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    pub dhr8r1: DHR8R1,
    _reserved5: [u8; 0x18],
    #[doc = "0x2c - DAC channel1 data output register"]
    pub dor1: DOR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x34 - DAC status register"]
    pub sr: SR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SWTRIGR (w) register accessor: an alias for `Reg<SWTRIGR_SPEC>`"]
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "DHR12R1 (rw) register accessor: an alias for `Reg<DHR12R1_SPEC>`"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: an alias for `Reg<DHR12L1_SPEC>`"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: an alias for `Reg<DHR8R1_SPEC>`"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DOR1 (r) register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DAC status register"]
pub mod sr;
