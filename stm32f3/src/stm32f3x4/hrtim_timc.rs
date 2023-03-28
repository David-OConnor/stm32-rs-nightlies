#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timccr: TIMCCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timcisr: TIMCISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timcicr: TIMCICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timcdier: TIMCDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntcr: CNTCR,
    #[doc = "0x14 - Timerx Period Register"]
    pub percr: PERCR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repcr: REPCR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1cr: CMP1CR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1ccr: CMP1CCR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2cr: CMP2CR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3cr: CMP3CR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4cr: CMP4CR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1cr: CPT1CR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2cr: CPT2CR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtcr: DTCR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setc1r: SETC1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstc1r: RSTC1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setc2r: SETC2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstc2r: RSTC2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefcr1: EEFCR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefcr2: EEFCR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstcr: RSTCR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpcr: CHPCR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ccr: CPT1CCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ccr: CPT2CCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outcr: OUTCR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltcr: FLTCR,
}
#[doc = "TIMCCR (rw) register accessor: an alias for `Reg<TIMCCR_SPEC>`"]
pub type TIMCCR = crate::Reg<timccr::TIMCCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timccr;
#[doc = "TIMCISR (r) register accessor: an alias for `Reg<TIMCISR_SPEC>`"]
pub type TIMCISR = crate::Reg<timcisr::TIMCISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timcisr;
#[doc = "TIMCICR (w) register accessor: an alias for `Reg<TIMCICR_SPEC>`"]
pub type TIMCICR = crate::Reg<timcicr::TIMCICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timcicr;
#[doc = "TIMCDIER (rw) register accessor: an alias for `Reg<TIMCDIER_SPEC>`"]
pub type TIMCDIER = crate::Reg<timcdier::TIMCDIER_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timcdier;
#[doc = "CNTCR (rw) register accessor: an alias for `Reg<CNTCR_SPEC>`"]
pub type CNTCR = crate::Reg<cntcr::CNTCR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntcr;
#[doc = "PERCR (rw) register accessor: an alias for `Reg<PERCR_SPEC>`"]
pub type PERCR = crate::Reg<percr::PERCR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod percr;
#[doc = "REPCR (rw) register accessor: an alias for `Reg<REPCR_SPEC>`"]
pub type REPCR = crate::Reg<repcr::REPCR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repcr;
#[doc = "CMP1CR (rw) register accessor: an alias for `Reg<CMP1CR_SPEC>`"]
pub type CMP1CR = crate::Reg<cmp1cr::CMP1CR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1cr;
#[doc = "CMP1CCR (rw) register accessor: an alias for `Reg<CMP1CCR_SPEC>`"]
pub type CMP1CCR = crate::Reg<cmp1ccr::CMP1CCR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1ccr;
#[doc = "CMP2CR (rw) register accessor: an alias for `Reg<CMP2CR_SPEC>`"]
pub type CMP2CR = crate::Reg<cmp2cr::CMP2CR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2cr;
#[doc = "CMP3CR (rw) register accessor: an alias for `Reg<CMP3CR_SPEC>`"]
pub type CMP3CR = crate::Reg<cmp3cr::CMP3CR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3cr;
#[doc = "CMP4CR (rw) register accessor: an alias for `Reg<CMP4CR_SPEC>`"]
pub type CMP4CR = crate::Reg<cmp4cr::CMP4CR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4cr;
#[doc = "CPT1CR (r) register accessor: an alias for `Reg<CPT1CR_SPEC>`"]
pub type CPT1CR = crate::Reg<cpt1cr::CPT1CR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1cr;
#[doc = "CPT2CR (r) register accessor: an alias for `Reg<CPT2CR_SPEC>`"]
pub type CPT2CR = crate::Reg<cpt2cr::CPT2CR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2cr;
#[doc = "DTCR (rw) register accessor: an alias for `Reg<DTCR_SPEC>`"]
pub type DTCR = crate::Reg<dtcr::DTCR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtcr;
#[doc = "SETC1R (rw) register accessor: an alias for `Reg<SETC1R_SPEC>`"]
pub type SETC1R = crate::Reg<setc1r::SETC1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setc1r;
#[doc = "RSTC1R (rw) register accessor: an alias for `Reg<RSTC1R_SPEC>`"]
pub type RSTC1R = crate::Reg<rstc1r::RSTC1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstc1r;
#[doc = "SETC2R (rw) register accessor: an alias for `Reg<SETC2R_SPEC>`"]
pub type SETC2R = crate::Reg<setc2r::SETC2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setc2r;
#[doc = "RSTC2R (rw) register accessor: an alias for `Reg<RSTC2R_SPEC>`"]
pub type RSTC2R = crate::Reg<rstc2r::RSTC2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstc2r;
#[doc = "EEFCR1 (rw) register accessor: an alias for `Reg<EEFCR1_SPEC>`"]
pub type EEFCR1 = crate::Reg<eefcr1::EEFCR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefcr1;
#[doc = "EEFCR2 (rw) register accessor: an alias for `Reg<EEFCR2_SPEC>`"]
pub type EEFCR2 = crate::Reg<eefcr2::EEFCR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefcr2;
#[doc = "RSTCR (rw) register accessor: an alias for `Reg<RSTCR_SPEC>`"]
pub type RSTCR = crate::Reg<rstcr::RSTCR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstcr;
#[doc = "CHPCR (rw) register accessor: an alias for `Reg<CHPCR_SPEC>`"]
pub type CHPCR = crate::Reg<chpcr::CHPCR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpcr;
#[doc = "CPT1CCR (rw) register accessor: an alias for `Reg<CPT1CCR_SPEC>`"]
pub type CPT1CCR = crate::Reg<cpt1ccr::CPT1CCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ccr;
#[doc = "CPT2CCR (rw) register accessor: an alias for `Reg<CPT2CCR_SPEC>`"]
pub type CPT2CCR = crate::Reg<cpt2ccr::CPT2CCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2ccr;
#[doc = "OUTCR (rw) register accessor: an alias for `Reg<OUTCR_SPEC>`"]
pub type OUTCR = crate::Reg<outcr::OUTCR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outcr;
#[doc = "FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltcr;
