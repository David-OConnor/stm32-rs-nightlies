#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control Register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x0c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Output Enable Register"]
    pub oenr: OENR,
    #[doc = "0x18 - DISR"]
    pub disr: DISR,
    #[doc = "0x1c - Output Disable Status Register"]
    pub odsr: ODSR,
    #[doc = "0x20 - Burst Mode Control Register"]
    pub bmcr: BMCR,
    #[doc = "0x24 - BMTRG"]
    pub bmtrg: BMTRG,
    #[doc = "0x28 - BMCMPR6"]
    pub bmcmpr6: BMCMPR6,
    #[doc = "0x2c - Burst Mode Period Register"]
    pub bmper: BMPER,
    #[doc = "0x30 - Timer External Event Control Register 1"]
    pub eecr1: EECR1,
    #[doc = "0x34 - Timer External Event Control Register 2"]
    pub eecr2: EECR2,
    #[doc = "0x38 - Timer External Event Control Register 3"]
    pub eecr3: EECR3,
    #[doc = "0x3c - ADC Trigger 1 Register"]
    pub adc1r: ADC1R,
    #[doc = "0x40 - ADC Trigger 2 Register"]
    pub adc2r: ADC2R,
    #[doc = "0x44 - ADC Trigger 3 Register"]
    pub adc3r: ADC3R,
    #[doc = "0x48 - ADC Trigger 4 Register"]
    pub adc4r: ADC4R,
    #[doc = "0x4c - DLL Control Register"]
    pub dllcr: DLLCR,
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    pub fltinr1: FLTINR1,
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    pub fltinr2: FLTINR2,
    #[doc = "0x58 - BDMUPDR"]
    pub bdmupdr: BDMUPDR,
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    pub bdtx_upr: BDTX_UPR,
    _reserved24: [u8; 0x10],
    #[doc = "0x70 - Burst DMA Data register"]
    pub bdmadr: BDMADR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "OENR (w) register accessor: an alias for `Reg<OENR_SPEC>`"]
pub type OENR = crate::Reg<oenr::OENR_SPEC>;
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "DISR (rw) register accessor: an alias for `Reg<DISR_SPEC>`"]
pub type DISR = crate::Reg<disr::DISR_SPEC>;
#[doc = "DISR"]
pub mod disr;
#[doc = "ODSR (r) register accessor: an alias for `Reg<ODSR_SPEC>`"]
pub type ODSR = crate::Reg<odsr::ODSR_SPEC>;
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "BMCR (rw) register accessor: an alias for `Reg<BMCR_SPEC>`"]
pub type BMCR = crate::Reg<bmcr::BMCR_SPEC>;
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRG (rw) register accessor: an alias for `Reg<BMTRG_SPEC>`"]
pub type BMTRG = crate::Reg<bmtrg::BMTRG_SPEC>;
#[doc = "BMTRG"]
pub mod bmtrg;
#[doc = "BMCMPR6 (rw) register accessor: an alias for `Reg<BMCMPR6_SPEC>`"]
pub type BMCMPR6 = crate::Reg<bmcmpr6::BMCMPR6_SPEC>;
#[doc = "BMCMPR6"]
pub mod bmcmpr6;
#[doc = "BMPER (rw) register accessor: an alias for `Reg<BMPER_SPEC>`"]
pub type BMPER = crate::Reg<bmper::BMPER_SPEC>;
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "EECR1 (rw) register accessor: an alias for `Reg<EECR1_SPEC>`"]
pub type EECR1 = crate::Reg<eecr1::EECR1_SPEC>;
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "EECR2 (rw) register accessor: an alias for `Reg<EECR2_SPEC>`"]
pub type EECR2 = crate::Reg<eecr2::EECR2_SPEC>;
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "EECR3 (rw) register accessor: an alias for `Reg<EECR3_SPEC>`"]
pub type EECR3 = crate::Reg<eecr3::EECR3_SPEC>;
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC1R (rw) register accessor: an alias for `Reg<ADC1R_SPEC>`"]
pub type ADC1R = crate::Reg<adc1r::ADC1R_SPEC>;
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC2R (rw) register accessor: an alias for `Reg<ADC2R_SPEC>`"]
pub type ADC2R = crate::Reg<adc2r::ADC2R_SPEC>;
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC3R (rw) register accessor: an alias for `Reg<ADC3R_SPEC>`"]
pub type ADC3R = crate::Reg<adc3r::ADC3R_SPEC>;
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC4R (rw) register accessor: an alias for `Reg<ADC4R_SPEC>`"]
pub type ADC4R = crate::Reg<adc4r::ADC4R_SPEC>;
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLLCR (rw) register accessor: an alias for `Reg<DLLCR_SPEC>`"]
pub type DLLCR = crate::Reg<dllcr::DLLCR_SPEC>;
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "FLTINR1 (rw) register accessor: an alias for `Reg<FLTINR1_SPEC>`"]
pub type FLTINR1 = crate::Reg<fltinr1::FLTINR1_SPEC>;
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "FLTINR2 (rw) register accessor: an alias for `Reg<FLTINR2_SPEC>`"]
pub type FLTINR2 = crate::Reg<fltinr2::FLTINR2_SPEC>;
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR (rw) register accessor: an alias for `Reg<BDMUPDR_SPEC>`"]
pub type BDMUPDR = crate::Reg<bdmupdr::BDMUPDR_SPEC>;
#[doc = "BDMUPDR"]
pub mod bdmupdr;
#[doc = "BDTxUPR (rw) register accessor: an alias for `Reg<BDTX_UPR_SPEC>`"]
pub type BDTX_UPR = crate::Reg<bdtx_upr::BDTX_UPR_SPEC>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtx_upr;
#[doc = "BDMADR (rw) register accessor: an alias for `Reg<BDMADR_SPEC>`"]
pub type BDMADR = crate::Reg<bdmadr::BDMADR_SPEC>;
#[doc = "Burst DMA Data register"]
pub mod bdmadr;
