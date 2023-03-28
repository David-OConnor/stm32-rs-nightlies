#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel x configuration register"]
    pub cr: CR,
    #[doc = "0x04 - DMA channel x number of data register"]
    pub ndtr: NDTR,
    #[doc = "0x08 - This register must not be written when the channel is enabled."]
    pub par: PAR,
    #[doc = "0x0c - This register must not be written when the channel is enabled."]
    pub m0ar: M0AR,
    #[doc = "0x10 - This register must not be written when the channel is enabled"]
    pub m1ar: M1AR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA channel x configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod par;
#[doc = "M0AR (rw) register accessor: an alias for `Reg<M0AR_SPEC>`"]
pub type M0AR = crate::Reg<m0ar::M0AR_SPEC>;
#[doc = "This register must not be written when the channel is enabled."]
pub mod m0ar;
#[doc = "M1AR (rw) register accessor: an alias for `Reg<M1AR_SPEC>`"]
pub type M1AR = crate::Reg<m1ar::M1AR_SPEC>;
#[doc = "This register must not be written when the channel is enabled"]
pub mod m1ar;
