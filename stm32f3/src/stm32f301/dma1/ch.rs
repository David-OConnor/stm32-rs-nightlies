#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel configuration register (DMA_CCR)"]
    pub cr: CR,
    #[doc = "0x04 - DMA channel 1 number of data register"]
    pub ndtr: NDTR,
    #[doc = "0x08 - DMA channel 1 peripheral address register"]
    pub par: PAR,
    #[doc = "0x0c - DMA channel 1 memory address register"]
    pub mar: MAR,
    _reserved_end: [u8; 0x04],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod par;
#[doc = "MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod mar;
