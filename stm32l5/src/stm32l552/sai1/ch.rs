#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - AConfiguration register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - AConfiguration register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - AFRCR"]
    pub frcr: FRCR,
    #[doc = "0x0c - ASlot register"]
    pub slotr: SLOTR,
    #[doc = "0x10 - AInterrupt mask register2"]
    pub im: IM,
    #[doc = "0x14 - AStatus register"]
    pub sr: SR,
    #[doc = "0x18 - AClear flag register"]
    pub clrfr: CLRFR,
    #[doc = "0x1c - AData register"]
    pub dr: DR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "AConfiguration register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "AConfiguration register 2"]
pub mod cr2;
#[doc = "FRCR (rw) register accessor: an alias for `Reg<FRCR_SPEC>`"]
pub type FRCR = crate::Reg<frcr::FRCR_SPEC>;
#[doc = "AFRCR"]
pub mod frcr;
#[doc = "SLOTR (rw) register accessor: an alias for `Reg<SLOTR_SPEC>`"]
pub type SLOTR = crate::Reg<slotr::SLOTR_SPEC>;
#[doc = "ASlot register"]
pub mod slotr;
#[doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "AInterrupt mask register2"]
pub mod im;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "AStatus register"]
pub mod sr;
#[doc = "CLRFR (w) register accessor: an alias for `Reg<CLRFR_SPEC>`"]
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
#[doc = "AClear flag register"]
pub mod clrfr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "AData register"]
pub mod dr;
