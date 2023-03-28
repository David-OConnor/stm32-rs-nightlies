#[doc = r"Register block"]
#[repr(C)]
pub struct FLT {
    #[doc = "0x00 - "]
    pub cr1: CR1,
    #[doc = "0x04 - "]
    pub cr2: CR2,
    #[doc = "0x08 - "]
    pub isr: ISR,
    #[doc = "0x0c - "]
    pub icr: ICR,
    #[doc = "0x10 - "]
    pub jchgr: JCHGR,
    #[doc = "0x14 - "]
    pub fcr: FCR,
    #[doc = "0x18 - "]
    pub jdatar: JDATAR,
    #[doc = "0x1c - "]
    pub rdatar: RDATAR,
    #[doc = "0x20 - "]
    pub awhtr: AWHTR,
    #[doc = "0x24 - "]
    pub awltr: AWLTR,
    #[doc = "0x28 - "]
    pub awsr: AWSR,
    #[doc = "0x2c - "]
    pub awcfr: AWCFR,
    #[doc = "0x30 - "]
    pub exmax: EXMAX,
    #[doc = "0x34 - "]
    pub exmin: EXMIN,
    #[doc = "0x38 - "]
    pub fltcnvtimr: FLTCNVTIMR,
    _reserved_end: [u8; 0x44],
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = ""]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = ""]
pub mod cr2;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = ""]
pub mod isr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = ""]
pub mod icr;
#[doc = "JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`"]
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
#[doc = ""]
pub mod jchgr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = ""]
pub mod fcr;
#[doc = "JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`"]
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
#[doc = ""]
pub mod jdatar;
#[doc = "RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`"]
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
#[doc = ""]
pub mod rdatar;
#[doc = "AWHTR (rw) register accessor: an alias for `Reg<AWHTR_SPEC>`"]
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
#[doc = ""]
pub mod awhtr;
#[doc = "AWLTR (rw) register accessor: an alias for `Reg<AWLTR_SPEC>`"]
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
#[doc = ""]
pub mod awltr;
#[doc = "AWSR (r) register accessor: an alias for `Reg<AWSR_SPEC>`"]
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
#[doc = ""]
pub mod awsr;
#[doc = "AWCFR (rw) register accessor: an alias for `Reg<AWCFR_SPEC>`"]
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
#[doc = ""]
pub mod awcfr;
#[doc = "EXMAX (r) register accessor: an alias for `Reg<EXMAX_SPEC>`"]
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
#[doc = ""]
pub mod exmax;
#[doc = "EXMIN (rw) register accessor: an alias for `Reg<EXMIN_SPEC>`"]
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
#[doc = ""]
pub mod exmin;
#[doc = "FLTCNVTIMR (r) register accessor: an alias for `Reg<FLTCNVTIMR_SPEC>`"]
pub type FLTCNVTIMR = crate::Reg<fltcnvtimr::FLTCNVTIMR_SPEC>;
#[doc = ""]
pub mod fltcnvtimr;
