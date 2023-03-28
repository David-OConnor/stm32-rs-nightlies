#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel 1 configuration register"]
    pub cr: CR,
    #[doc = "0x04 - DMA channel 1 number of data tegister"]
    pub ndtr: NDTR,
    #[doc = "0x08 - DMA channel 1 peripheral address"]
    pub par: PAR,
    #[doc = "0x0c - DMA channel 1 memory address"]
    pub mar: MAR,
    _reserved_end: [u8; 0x04],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA channel 1 configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "DMA channel 1 number of data tegister"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "DMA channel 1 peripheral address"]
pub mod par;
#[doc = "MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "DMA channel 1 memory address"]
pub mod mar;
