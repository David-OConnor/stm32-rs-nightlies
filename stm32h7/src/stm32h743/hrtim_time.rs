#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timecr: TIMECR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timeisr: TIMEISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timeicr: TIMEICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timedier5: TIMEDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cnter: CNTER,
    #[doc = "0x14 - Timerx Period Register"]
    pub perer: PERER,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub reper: REPER,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1er: CMP1ER,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cer: CMP1CER,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2er: CMP2ER,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3er: CMP3ER,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4er: CMP4ER,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1er: CPT1ER,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2er: CPT2ER,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dter: DTER,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub sete1r: SETE1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rste1r: RSTE1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub sete2r: SETE2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rste2r: RSTE2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefer1: EEFER1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefer2: EEFER2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rster: RSTER,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chper: CHPER,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ecr: CPT1ECR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ecr: CPT2ECR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outer: OUTER,
    #[doc = "0x68 - Timerx Fault Register"]
    pub flter: FLTER,
}
#[doc = "TIMECR (rw) register accessor: an alias for `Reg<TIMECR_SPEC>`"]
pub type TIMECR = crate::Reg<timecr::TIMECR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timecr;
#[doc = "TIMEISR (r) register accessor: an alias for `Reg<TIMEISR_SPEC>`"]
pub type TIMEISR = crate::Reg<timeisr::TIMEISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timeisr;
#[doc = "TIMEICR (w) register accessor: an alias for `Reg<TIMEICR_SPEC>`"]
pub type TIMEICR = crate::Reg<timeicr::TIMEICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timeicr;
#[doc = "TIMEDIER5 (rw) register accessor: an alias for `Reg<TIMEDIER5_SPEC>`"]
pub type TIMEDIER5 = crate::Reg<timedier5::TIMEDIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timedier5;
#[doc = "CNTER (rw) register accessor: an alias for `Reg<CNTER_SPEC>`"]
pub type CNTER = crate::Reg<cnter::CNTER_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cnter;
#[doc = "PERER (rw) register accessor: an alias for `Reg<PERER_SPEC>`"]
pub type PERER = crate::Reg<perer::PERER_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perer;
#[doc = "REPER (rw) register accessor: an alias for `Reg<REPER_SPEC>`"]
pub type REPER = crate::Reg<reper::REPER_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod reper;
#[doc = "CMP1ER (rw) register accessor: an alias for `Reg<CMP1ER_SPEC>`"]
pub type CMP1ER = crate::Reg<cmp1er::CMP1ER_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1er;
#[doc = "CMP1CER (rw) register accessor: an alias for `Reg<CMP1CER_SPEC>`"]
pub type CMP1CER = crate::Reg<cmp1cer::CMP1CER_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cer;
#[doc = "CMP2ER (rw) register accessor: an alias for `Reg<CMP2ER_SPEC>`"]
pub type CMP2ER = crate::Reg<cmp2er::CMP2ER_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2er;
#[doc = "CMP3ER (rw) register accessor: an alias for `Reg<CMP3ER_SPEC>`"]
pub type CMP3ER = crate::Reg<cmp3er::CMP3ER_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3er;
#[doc = "CMP4ER (rw) register accessor: an alias for `Reg<CMP4ER_SPEC>`"]
pub type CMP4ER = crate::Reg<cmp4er::CMP4ER_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4er;
#[doc = "CPT1ER (r) register accessor: an alias for `Reg<CPT1ER_SPEC>`"]
pub type CPT1ER = crate::Reg<cpt1er::CPT1ER_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1er;
#[doc = "CPT2ER (r) register accessor: an alias for `Reg<CPT2ER_SPEC>`"]
pub type CPT2ER = crate::Reg<cpt2er::CPT2ER_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2er;
#[doc = "DTER (rw) register accessor: an alias for `Reg<DTER_SPEC>`"]
pub type DTER = crate::Reg<dter::DTER_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dter;
#[doc = "SETE1R (rw) register accessor: an alias for `Reg<SETE1R_SPEC>`"]
pub type SETE1R = crate::Reg<sete1r::SETE1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod sete1r;
#[doc = "RSTE1R (rw) register accessor: an alias for `Reg<RSTE1R_SPEC>`"]
pub type RSTE1R = crate::Reg<rste1r::RSTE1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "SETE2R (rw) register accessor: an alias for `Reg<SETE2R_SPEC>`"]
pub type SETE2R = crate::Reg<sete2r::SETE2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod sete2r;
#[doc = "RSTE2R (rw) register accessor: an alias for `Reg<RSTE2R_SPEC>`"]
pub type RSTE2R = crate::Reg<rste2r::RSTE2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rste2r;
#[doc = "EEFER1 (rw) register accessor: an alias for `Reg<EEFER1_SPEC>`"]
pub type EEFER1 = crate::Reg<eefer1::EEFER1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefer1;
#[doc = "EEFER2 (rw) register accessor: an alias for `Reg<EEFER2_SPEC>`"]
pub type EEFER2 = crate::Reg<eefer2::EEFER2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefer2;
#[doc = "RSTER (rw) register accessor: an alias for `Reg<RSTER_SPEC>`"]
pub type RSTER = crate::Reg<rster::RSTER_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rster;
#[doc = "CHPER (rw) register accessor: an alias for `Reg<CHPER_SPEC>`"]
pub type CHPER = crate::Reg<chper::CHPER_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chper;
#[doc = "CPT1ECR (rw) register accessor: an alias for `Reg<CPT1ECR_SPEC>`"]
pub type CPT1ECR = crate::Reg<cpt1ecr::CPT1ECR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ecr;
#[doc = "CPT2ECR (rw) register accessor: an alias for `Reg<CPT2ECR_SPEC>`"]
pub type CPT2ECR = crate::Reg<cpt2ecr::CPT2ECR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2ecr;
#[doc = "OUTER (rw) register accessor: an alias for `Reg<OUTER_SPEC>`"]
pub type OUTER = crate::Reg<outer::OUTER_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outer;
#[doc = "FLTER (rw) register accessor: an alias for `Reg<FLTER_SPEC>`"]
pub type FLTER = crate::Reg<flter::FLTER_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod flter;
