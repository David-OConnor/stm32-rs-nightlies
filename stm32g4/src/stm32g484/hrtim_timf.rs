#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timfcr: TIMFCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timfisr: TIMFISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timficr: TIMFICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timfdier: TIMFDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntfr: CNTFR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perfr: PERFR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repfr: REPFR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1fr: CMP1FR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cfr: CMP1CFR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2fr: CMP2FR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3fr: CMP3FR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4fr: CMP4FR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1fr: CPT1FR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2fr: CPT2FR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtfr: DTFR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setf1r: SETF1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rste1r: RSTE1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setf2r: SETF2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstf2r: RSTF2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eeffr1: EEFFR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eeffr2: EEFFR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstfr: RSTFR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpfr: CHPFR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1fcr: CPT1FCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2fcr: CPT2FCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outfr: OUTFR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltfr: FLTFR,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timfcr2: TIMFCR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub feefr3: FEEFR3,
}
#[doc = "TIMFCR (rw) register accessor: an alias for `Reg<TIMFCR_SPEC>`"]
pub type TIMFCR = crate::Reg<timfcr::TIMFCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timfcr;
#[doc = "TIMFISR (r) register accessor: an alias for `Reg<TIMFISR_SPEC>`"]
pub type TIMFISR = crate::Reg<timfisr::TIMFISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timfisr;
#[doc = "TIMFICR (w) register accessor: an alias for `Reg<TIMFICR_SPEC>`"]
pub type TIMFICR = crate::Reg<timficr::TIMFICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timficr;
#[doc = "TIMFDIER (rw) register accessor: an alias for `Reg<TIMFDIER_SPEC>`"]
pub type TIMFDIER = crate::Reg<timfdier::TIMFDIER_SPEC>;
#[doc = "TIMxDIER"]
pub mod timfdier;
#[doc = "CNTFR (rw) register accessor: an alias for `Reg<CNTFR_SPEC>`"]
pub type CNTFR = crate::Reg<cntfr::CNTFR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntfr;
#[doc = "PERFR (rw) register accessor: an alias for `Reg<PERFR_SPEC>`"]
pub type PERFR = crate::Reg<perfr::PERFR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perfr;
#[doc = "REPFR (rw) register accessor: an alias for `Reg<REPFR_SPEC>`"]
pub type REPFR = crate::Reg<repfr::REPFR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repfr;
#[doc = "CMP1FR (rw) register accessor: an alias for `Reg<CMP1FR_SPEC>`"]
pub type CMP1FR = crate::Reg<cmp1fr::CMP1FR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1fr;
#[doc = "CMP1CFR (rw) register accessor: an alias for `Reg<CMP1CFR_SPEC>`"]
pub type CMP1CFR = crate::Reg<cmp1cfr::CMP1CFR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cfr;
#[doc = "CMP2FR (rw) register accessor: an alias for `Reg<CMP2FR_SPEC>`"]
pub type CMP2FR = crate::Reg<cmp2fr::CMP2FR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2fr;
#[doc = "CMP3FR (rw) register accessor: an alias for `Reg<CMP3FR_SPEC>`"]
pub type CMP3FR = crate::Reg<cmp3fr::CMP3FR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3fr;
#[doc = "CMP4FR (rw) register accessor: an alias for `Reg<CMP4FR_SPEC>`"]
pub type CMP4FR = crate::Reg<cmp4fr::CMP4FR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4fr;
#[doc = "CPT1FR (r) register accessor: an alias for `Reg<CPT1FR_SPEC>`"]
pub type CPT1FR = crate::Reg<cpt1fr::CPT1FR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1fr;
#[doc = "CPT2FR (r) register accessor: an alias for `Reg<CPT2FR_SPEC>`"]
pub type CPT2FR = crate::Reg<cpt2fr::CPT2FR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2fr;
#[doc = "DTFR (rw) register accessor: an alias for `Reg<DTFR_SPEC>`"]
pub type DTFR = crate::Reg<dtfr::DTFR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtfr;
#[doc = "SETF1R (rw) register accessor: an alias for `Reg<SETF1R_SPEC>`"]
pub type SETF1R = crate::Reg<setf1r::SETF1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setf1r;
#[doc = "RSTE1R (rw) register accessor: an alias for `Reg<RSTE1R_SPEC>`"]
pub type RSTE1R = crate::Reg<rste1r::RSTE1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "SETF2R (rw) register accessor: an alias for `Reg<SETF2R_SPEC>`"]
pub type SETF2R = crate::Reg<setf2r::SETF2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setf2r;
#[doc = "RSTF2R (rw) register accessor: an alias for `Reg<RSTF2R_SPEC>`"]
pub type RSTF2R = crate::Reg<rstf2r::RSTF2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstf2r;
#[doc = "EEFFR1 (rw) register accessor: an alias for `Reg<EEFFR1_SPEC>`"]
pub type EEFFR1 = crate::Reg<eeffr1::EEFFR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eeffr1;
#[doc = "EEFFR2 (rw) register accessor: an alias for `Reg<EEFFR2_SPEC>`"]
pub type EEFFR2 = crate::Reg<eeffr2::EEFFR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eeffr2;
#[doc = "RSTFR (rw) register accessor: an alias for `Reg<RSTFR_SPEC>`"]
pub type RSTFR = crate::Reg<rstfr::RSTFR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstfr;
#[doc = "CHPFR (rw) register accessor: an alias for `Reg<CHPFR_SPEC>`"]
pub type CHPFR = crate::Reg<chpfr::CHPFR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpfr;
#[doc = "CPT1FCR (rw) register accessor: an alias for `Reg<CPT1FCR_SPEC>`"]
pub type CPT1FCR = crate::Reg<cpt1fcr::CPT1FCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1fcr;
#[doc = "CPT2FCR (rw) register accessor: an alias for `Reg<CPT2FCR_SPEC>`"]
pub type CPT2FCR = crate::Reg<cpt2fcr::CPT2FCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2fcr;
#[doc = "OUTFR (rw) register accessor: an alias for `Reg<OUTFR_SPEC>`"]
pub type OUTFR = crate::Reg<outfr::OUTFR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outfr;
#[doc = "FLTFR (rw) register accessor: an alias for `Reg<FLTFR_SPEC>`"]
pub type FLTFR = crate::Reg<fltfr::FLTFR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltfr;
#[doc = "TIMFCR2 (rw) register accessor: an alias for `Reg<TIMFCR2_SPEC>`"]
pub type TIMFCR2 = crate::Reg<timfcr2::TIMFCR2_SPEC>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timfcr2;
#[doc = "FEEFR3 (rw) register accessor: an alias for `Reg<FEEFR3_SPEC>`"]
pub type FEEFR3 = crate::Reg<feefr3::FEEFR3_SPEC>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod feefr3;
