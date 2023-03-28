#[doc = r"Register block"]
#[repr(C)]
pub struct DOEP {
    #[doc = "0x00 - device endpoint-1 control register"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - device endpoint-1 interrupt register"]
    pub int: INT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - device OUT endpoint-1 transfer size register"]
    pub tsiz: TSIZ,
    _reserved_end: [u8; 0x0c],
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod ctl;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod int;
#[doc = "TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`"]
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod tsiz;
