#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timacr: TIMACR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timaisr: TIMAISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timaicr: TIMAICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timadier5: TIMADIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntar: CNTAR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perar: PERAR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repar: REPAR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1ar: CMP1AR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1car: CMP1CAR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2ar: CMP2AR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3ar: CMP3AR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4ar: CMP4AR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1ar: CPT1AR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2ar: CPT2AR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtar: DTAR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub seta1r: SETA1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rsta1r: RSTA1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub seta2r: SETA2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rsta2r: RSTA2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefar1: EEFAR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefar2: EEFAR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstar: RSTAR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpar: CHPAR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1acr: CPT1ACR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2acr: CPT2ACR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outar: OUTAR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltar: FLTAR,
}
#[doc = "TIMACR (rw) register accessor: an alias for `Reg<TIMACR_SPEC>`"]
pub type TIMACR = crate::Reg<timacr::TIMACR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "TIMAISR (r) register accessor: an alias for `Reg<TIMAISR_SPEC>`"]
pub type TIMAISR = crate::Reg<timaisr::TIMAISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timaisr;
#[doc = "TIMAICR (w) register accessor: an alias for `Reg<TIMAICR_SPEC>`"]
pub type TIMAICR = crate::Reg<timaicr::TIMAICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMADIER5 (rw) register accessor: an alias for `Reg<TIMADIER5_SPEC>`"]
pub type TIMADIER5 = crate::Reg<timadier5::TIMADIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timadier5;
#[doc = "CNTAR (rw) register accessor: an alias for `Reg<CNTAR_SPEC>`"]
pub type CNTAR = crate::Reg<cntar::CNTAR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "PERAR (rw) register accessor: an alias for `Reg<PERAR_SPEC>`"]
pub type PERAR = crate::Reg<perar::PERAR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "REPAR (rw) register accessor: an alias for `Reg<REPAR_SPEC>`"]
pub type REPAR = crate::Reg<repar::REPAR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "CMP1AR (rw) register accessor: an alias for `Reg<CMP1AR_SPEC>`"]
pub type CMP1AR = crate::Reg<cmp1ar::CMP1AR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "CMP1CAR (rw) register accessor: an alias for `Reg<CMP1CAR_SPEC>`"]
pub type CMP1CAR = crate::Reg<cmp1car::CMP1CAR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "CMP2AR (rw) register accessor: an alias for `Reg<CMP2AR_SPEC>`"]
pub type CMP2AR = crate::Reg<cmp2ar::CMP2AR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "CMP3AR (rw) register accessor: an alias for `Reg<CMP3AR_SPEC>`"]
pub type CMP3AR = crate::Reg<cmp3ar::CMP3AR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "CMP4AR (rw) register accessor: an alias for `Reg<CMP4AR_SPEC>`"]
pub type CMP4AR = crate::Reg<cmp4ar::CMP4AR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "CPT1AR (r) register accessor: an alias for `Reg<CPT1AR_SPEC>`"]
pub type CPT1AR = crate::Reg<cpt1ar::CPT1AR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "CPT2AR (r) register accessor: an alias for `Reg<CPT2AR_SPEC>`"]
pub type CPT2AR = crate::Reg<cpt2ar::CPT2AR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "DTAR (rw) register accessor: an alias for `Reg<DTAR_SPEC>`"]
pub type DTAR = crate::Reg<dtar::DTAR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "SETA1R (rw) register accessor: an alias for `Reg<SETA1R_SPEC>`"]
pub type SETA1R = crate::Reg<seta1r::SETA1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "RSTA1R (rw) register accessor: an alias for `Reg<RSTA1R_SPEC>`"]
pub type RSTA1R = crate::Reg<rsta1r::RSTA1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "SETA2R (rw) register accessor: an alias for `Reg<SETA2R_SPEC>`"]
pub type SETA2R = crate::Reg<seta2r::SETA2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "RSTA2R (rw) register accessor: an alias for `Reg<RSTA2R_SPEC>`"]
pub type RSTA2R = crate::Reg<rsta2r::RSTA2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "EEFAR1 (rw) register accessor: an alias for `Reg<EEFAR1_SPEC>`"]
pub type EEFAR1 = crate::Reg<eefar1::EEFAR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "EEFAR2 (rw) register accessor: an alias for `Reg<EEFAR2_SPEC>`"]
pub type EEFAR2 = crate::Reg<eefar2::EEFAR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "RSTAR (rw) register accessor: an alias for `Reg<RSTAR_SPEC>`"]
pub type RSTAR = crate::Reg<rstar::RSTAR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "CHPAR (rw) register accessor: an alias for `Reg<CHPAR_SPEC>`"]
pub type CHPAR = crate::Reg<chpar::CHPAR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "CPT1ACR (rw) register accessor: an alias for `Reg<CPT1ACR_SPEC>`"]
pub type CPT1ACR = crate::Reg<cpt1acr::CPT1ACR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2ACR (rw) register accessor: an alias for `Reg<CPT2ACR_SPEC>`"]
pub type CPT2ACR = crate::Reg<cpt2acr::CPT2ACR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "OUTAR (rw) register accessor: an alias for `Reg<OUTAR_SPEC>`"]
pub type OUTAR = crate::Reg<outar::OUTAR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "FLTAR (rw) register accessor: an alias for `Reg<FLTAR_SPEC>`"]
pub type FLTAR = crate::Reg<fltar::FLTAR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltar;
