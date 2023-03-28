#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DFSDM channel 0 configuration register"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - DFSDM channel 0 configuration register"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - DFSDM channel 0 analog watchdog and short-circuit detector register"]
    pub awscdr: AWSCDR,
    #[doc = "0x0c - DFSDM channel 0 watchdog filter data register"]
    pub wdatr: WDATR,
    #[doc = "0x10 - DFSDM channel 0 data input register"]
    pub datinr: DATINR,
    #[doc = "0x14 - "]
    pub dlyr: DLYR,
    _reserved_end: [u8; 0x08],
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "DFSDM channel 0 configuration register"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "DFSDM channel 0 configuration register"]
pub mod cfgr2;
#[doc = "AWSCDR (rw) register accessor: an alias for `Reg<AWSCDR_SPEC>`"]
pub type AWSCDR = crate::Reg<awscdr::AWSCDR_SPEC>;
#[doc = "DFSDM channel 0 analog watchdog and short-circuit detector register"]
pub mod awscdr;
#[doc = "WDATR (r) register accessor: an alias for `Reg<WDATR_SPEC>`"]
pub type WDATR = crate::Reg<wdatr::WDATR_SPEC>;
#[doc = "DFSDM channel 0 watchdog filter data register"]
pub mod wdatr;
#[doc = "DATINR (rw) register accessor: an alias for `Reg<DATINR_SPEC>`"]
pub type DATINR = crate::Reg<datinr::DATINR_SPEC>;
#[doc = "DFSDM channel 0 data input register"]
pub mod datinr;
#[doc = "DLYR (rw) register accessor: an alias for `Reg<DLYR_SPEC>`"]
pub type DLYR = crate::Reg<dlyr::DLYR_SPEC>;
#[doc = ""]
pub mod dlyr;
