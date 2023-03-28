#[doc = r"Register block"]
#[repr(C)]
pub struct DIEP0 {
    #[doc = "0x00 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - device endpoint-x interrupt register"]
    pub int: INT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - device endpoint-0 transfer size register"]
    pub tsiz: TSIZ,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub txfsts: TXFSTS,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod ctl;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "device endpoint-x interrupt register"]
pub mod int;
#[doc = "TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`"]
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
#[doc = "device endpoint-0 transfer size register"]
pub mod tsiz;
#[doc = "TXFSTS (r) register accessor: an alias for `Reg<TXFSTS_SPEC>`"]
pub type TXFSTS = crate::Reg<txfsts::TXFSTS_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod txfsts;
