#[doc = r"Register block"]
#[repr(C)]
pub struct ST {
    #[doc = "0x00 - stream x configuration register"]
    pub cr: CR,
    #[doc = "0x04 - stream x number of data register"]
    pub ndtr: NDTR,
    #[doc = "0x08 - stream x peripheral address register"]
    pub par: PAR,
    #[doc = "0x0c - stream x memory 0 address register"]
    pub m0ar: M0AR,
    #[doc = "0x10 - stream x memory 1 address register"]
    pub m1ar: M1AR,
    #[doc = "0x14 - stream x FIFO control register"]
    pub fcr: FCR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod par;
#[doc = "M0AR (rw) register accessor: an alias for `Reg<M0AR_SPEC>`"]
pub type M0AR = crate::Reg<m0ar::M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod m0ar;
#[doc = "M1AR (rw) register accessor: an alias for `Reg<M1AR_SPEC>`"]
pub type M1AR = crate::Reg<m1ar::M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod m1ar;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod fcr;
