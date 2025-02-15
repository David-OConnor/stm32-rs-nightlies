#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    pub dac_cr: DAC_CR,
    #[doc = "0x04 - DAC software trigger register"]
    pub dac_swtrgr: DAC_SWTRGR,
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    pub dac_dhr12r1: DAC_DHR12R1,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    pub dac_dhr12l1: DAC_DHR12L1,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    pub dac_dhr8r1: DAC_DHR8R1,
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    pub dac_dhr12r2: DAC_DHR12R2,
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    pub dac_dhr12l2: DAC_DHR12L2,
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    pub dac_dhr8r2: DAC_DHR8R2,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dac_dhr12rd: DAC_DHR12RD,
    #[doc = "0x24 - Dual DAC 12-bit left aligned data holding register"]
    pub dac_dhr12ld: DAC_DHR12LD,
    #[doc = "0x28 - Dual DAC 8-bit right aligned data holding register"]
    pub dac_dhr8rd: DAC_DHR8RD,
    #[doc = "0x2c - DAC channel1 data output register"]
    pub dac_dor1: DAC_DOR1,
    #[doc = "0x30 - DAC channel2 data output register"]
    pub dac_dor2: DAC_DOR2,
    #[doc = "0x34 - DAC status register"]
    pub dac_sr: DAC_SR,
    #[doc = "0x38 - DAC calibration control register"]
    pub dac_ccr: DAC_CCR,
    #[doc = "0x3c - DAC mode control register"]
    pub dac_mcr: DAC_MCR,
    #[doc = "0x40 - DAC channel1 sample and hold sample time register"]
    pub dac_shsr1: DAC_SHSR1,
    #[doc = "0x44 - DAC channel2 sample and hold sample time register"]
    pub dac_shsr2: DAC_SHSR2,
    #[doc = "0x48 - DAC sample and hold time register"]
    pub dac_shhr: DAC_SHHR,
    #[doc = "0x4c - DAC sample and hold refresh time register"]
    pub dac_shrr: DAC_SHRR,
}
#[doc = "DAC_CR (rw) register accessor: an alias for `Reg<DAC_CR_SPEC>`"]
pub type DAC_CR = crate::Reg<dac_cr::DAC_CR_SPEC>;
#[doc = "DAC control register"]
pub mod dac_cr;
#[doc = "DAC_SWTRGR (w) register accessor: an alias for `Reg<DAC_SWTRGR_SPEC>`"]
pub type DAC_SWTRGR = crate::Reg<dac_swtrgr::DAC_SWTRGR_SPEC>;
#[doc = "DAC software trigger register"]
pub mod dac_swtrgr;
#[doc = "DAC_DHR12R1 (rw) register accessor: an alias for `Reg<DAC_DHR12R1_SPEC>`"]
pub type DAC_DHR12R1 = crate::Reg<dac_dhr12r1::DAC_DHR12R1_SPEC>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dac_dhr12r1;
#[doc = "DAC_DHR12L1 (rw) register accessor: an alias for `Reg<DAC_DHR12L1_SPEC>`"]
pub type DAC_DHR12L1 = crate::Reg<dac_dhr12l1::DAC_DHR12L1_SPEC>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dac_dhr12l1;
#[doc = "DAC_DHR8R1 (rw) register accessor: an alias for `Reg<DAC_DHR8R1_SPEC>`"]
pub type DAC_DHR8R1 = crate::Reg<dac_dhr8r1::DAC_DHR8R1_SPEC>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dac_dhr8r1;
#[doc = "DAC_DHR12R2 (rw) register accessor: an alias for `Reg<DAC_DHR12R2_SPEC>`"]
pub type DAC_DHR12R2 = crate::Reg<dac_dhr12r2::DAC_DHR12R2_SPEC>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dac_dhr12r2;
#[doc = "DAC_DHR12L2 (rw) register accessor: an alias for `Reg<DAC_DHR12L2_SPEC>`"]
pub type DAC_DHR12L2 = crate::Reg<dac_dhr12l2::DAC_DHR12L2_SPEC>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dac_dhr12l2;
#[doc = "DAC_DHR8R2 (rw) register accessor: an alias for `Reg<DAC_DHR8R2_SPEC>`"]
pub type DAC_DHR8R2 = crate::Reg<dac_dhr8r2::DAC_DHR8R2_SPEC>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dac_dhr8r2;
#[doc = "DAC_DHR12RD (rw) register accessor: an alias for `Reg<DAC_DHR12RD_SPEC>`"]
pub type DAC_DHR12RD = crate::Reg<dac_dhr12rd::DAC_DHR12RD_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dac_dhr12rd;
#[doc = "DAC_DHR12LD (rw) register accessor: an alias for `Reg<DAC_DHR12LD_SPEC>`"]
pub type DAC_DHR12LD = crate::Reg<dac_dhr12ld::DAC_DHR12LD_SPEC>;
#[doc = "Dual DAC 12-bit left aligned data holding register"]
pub mod dac_dhr12ld;
#[doc = "DAC_DHR8RD (rw) register accessor: an alias for `Reg<DAC_DHR8RD_SPEC>`"]
pub type DAC_DHR8RD = crate::Reg<dac_dhr8rd::DAC_DHR8RD_SPEC>;
#[doc = "Dual DAC 8-bit right aligned data holding register"]
pub mod dac_dhr8rd;
#[doc = "DAC_DOR1 (r) register accessor: an alias for `Reg<DAC_DOR1_SPEC>`"]
pub type DAC_DOR1 = crate::Reg<dac_dor1::DAC_DOR1_SPEC>;
#[doc = "DAC channel1 data output register"]
pub mod dac_dor1;
#[doc = "DAC_DOR2 (r) register accessor: an alias for `Reg<DAC_DOR2_SPEC>`"]
pub type DAC_DOR2 = crate::Reg<dac_dor2::DAC_DOR2_SPEC>;
#[doc = "DAC channel2 data output register"]
pub mod dac_dor2;
#[doc = "DAC_SR (rw) register accessor: an alias for `Reg<DAC_SR_SPEC>`"]
pub type DAC_SR = crate::Reg<dac_sr::DAC_SR_SPEC>;
#[doc = "DAC status register"]
pub mod dac_sr;
#[doc = "DAC_CCR (rw) register accessor: an alias for `Reg<DAC_CCR_SPEC>`"]
pub type DAC_CCR = crate::Reg<dac_ccr::DAC_CCR_SPEC>;
#[doc = "DAC calibration control register"]
pub mod dac_ccr;
#[doc = "DAC_MCR (rw) register accessor: an alias for `Reg<DAC_MCR_SPEC>`"]
pub type DAC_MCR = crate::Reg<dac_mcr::DAC_MCR_SPEC>;
#[doc = "DAC mode control register"]
pub mod dac_mcr;
#[doc = "DAC_SHSR1 (rw) register accessor: an alias for `Reg<DAC_SHSR1_SPEC>`"]
pub type DAC_SHSR1 = crate::Reg<dac_shsr1::DAC_SHSR1_SPEC>;
#[doc = "DAC channel1 sample and hold sample time register"]
pub mod dac_shsr1;
#[doc = "DAC_SHSR2 (rw) register accessor: an alias for `Reg<DAC_SHSR2_SPEC>`"]
pub type DAC_SHSR2 = crate::Reg<dac_shsr2::DAC_SHSR2_SPEC>;
#[doc = "DAC channel2 sample and hold sample time register"]
pub mod dac_shsr2;
#[doc = "DAC_SHHR (rw) register accessor: an alias for `Reg<DAC_SHHR_SPEC>`"]
pub type DAC_SHHR = crate::Reg<dac_shhr::DAC_SHHR_SPEC>;
#[doc = "DAC sample and hold time register"]
pub mod dac_shhr;
#[doc = "DAC_SHRR (rw) register accessor: an alias for `Reg<DAC_SHRR_SPEC>`"]
pub type DAC_SHRR = crate::Reg<dac_shrr::DAC_SHRR_SPEC>;
#[doc = "DAC sample and hold refresh time register"]
pub mod dac_shrr;
