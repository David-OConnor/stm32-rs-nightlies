#[doc = r"Register block"]
#[repr(C)]
pub struct DOEP {
    #[doc = "0x00 - OTG device endpoint-1 control register"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - OTG_HS device endpoint-1 interrupt register"]
    pub int: INT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device endpoint-1 transfer size register"]
    pub tsiz: TSIZ,
    #[doc = "0x14 - OTG_HS device endpoint-1 DMA address register"]
    pub dma: DMA,
    _reserved_end: [u8; 0x08],
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod ctl;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod int;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod dma;
#[doc = "TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`"]
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod tsiz;