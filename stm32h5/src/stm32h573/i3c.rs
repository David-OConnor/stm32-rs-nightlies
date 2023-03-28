#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_i3c_: [u8; 0x04],
    #[doc = "0x04 - I3C configuration register"]
    pub i3c_cfgr: I3C_CFGR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - I3C receive data byte register"]
    pub i3c_rdr: I3C_RDR,
    #[doc = "0x14 - I3C receive data word register"]
    pub i3c_rdwr: I3C_RDWR,
    #[doc = "0x18 - I3C transmit data byte register"]
    pub i3c_tdr: I3C_TDR,
    #[doc = "0x1c - I3C transmit data word register"]
    pub i3c_tdwr: I3C_TDWR,
    #[doc = "0x20 - I3C IBI payload data register"]
    pub i3c_ibidr: I3C_IBIDR,
    #[doc = "0x24 - I3C target transmit configuration register"]
    pub i3c_tgttdr: I3C_TGTTDR,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - I3C status register"]
    pub i3c_sr: I3C_SR,
    #[doc = "0x34 - I3C status error register"]
    pub i3c_ser: I3C_SER,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - I3C received message register"]
    pub i3c_rmr: I3C_RMR,
    _reserved11: [u8; 0x0c],
    #[doc = "0x50 - I3C event register"]
    pub i3c_evr: I3C_EVR,
    #[doc = "0x54 - I3C interrupt enable register"]
    pub i3c_ier: I3C_IER,
    #[doc = "0x58 - I3C clear event register"]
    pub i3c_cevr: I3C_CEVR,
    _reserved14: [u8; 0x04],
    #[doc = "0x60 - I3C own device characteristics register"]
    pub i3c_devr0: I3C_DEVR0,
    #[doc = "0x64 - I3C device 1 characteristics register"]
    pub i3c_devr1: I3C_DEVR1,
    #[doc = "0x68 - I3C device 2 characteristics register"]
    pub i3c_devr2: I3C_DEVR2,
    #[doc = "0x6c - I3C device 3 characteristics register"]
    pub i3c_devr3: I3C_DEVR3,
    #[doc = "0x70 - I3C device 4 characteristics register"]
    pub i3c_devr4: I3C_DEVR4,
    _reserved19: [u8; 0x1c],
    #[doc = "0x90 - I3C maximum read length register"]
    pub i3c_maxrlr: I3C_MAXRLR,
    #[doc = "0x94 - I3C maximum write length register"]
    pub i3c_maxwlr: I3C_MAXWLR,
    _reserved21: [u8; 0x08],
    #[doc = "0xa0 - I3C timing register 0"]
    pub i3c_timingr0: I3C_TIMINGR0,
    #[doc = "0xa4 - I3C timing register 1"]
    pub i3c_timingr1: I3C_TIMINGR1,
    #[doc = "0xa8 - I3C timing register 2"]
    pub i3c_timingr2: I3C_TIMINGR2,
    _reserved24: [u8; 0x14],
    #[doc = "0xc0 - I3C bus characteristics register"]
    pub i3c_bcr: I3C_BCR,
    #[doc = "0xc4 - I3C device characteristics register"]
    pub i3c_dcr: I3C_DCR,
    #[doc = "0xc8 - I3C get capability register"]
    pub i3c_getcapr: I3C_GETCAPR,
    #[doc = "0xcc - I3C controller-role capability register"]
    pub i3c_crcapr: I3C_CRCAPR,
    #[doc = "0xd0 - I3C get capability register"]
    pub i3c_getmxdsr: I3C_GETMXDSR,
    #[doc = "0xd4 - I3C extended provisioned ID register"]
    pub i3c_epidr: I3C_EPIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - I3C message control register alternate"]
    #[inline(always)]
    pub const fn i3c_cr_alternate(&self) -> &I3C_CR_ALTERNATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - I3C message control register"]
    #[inline(always)]
    pub const fn i3c_cr(&self) -> &I3C_CR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "I3C_CR (w) register accessor: an alias for `Reg<I3C_CR_SPEC>`"]
pub type I3C_CR = crate::Reg<i3c_cr::I3C_CR_SPEC>;
#[doc = "I3C message control register"]
pub mod i3c_cr;
#[doc = "I3C_CR_ALTERNATE (w) register accessor: an alias for `Reg<I3C_CR_ALTERNATE_SPEC>`"]
pub type I3C_CR_ALTERNATE = crate::Reg<i3c_cr_alternate::I3C_CR_ALTERNATE_SPEC>;
#[doc = "I3C message control register alternate"]
pub mod i3c_cr_alternate;
#[doc = "I3C_CFGR (rw) register accessor: an alias for `Reg<I3C_CFGR_SPEC>`"]
pub type I3C_CFGR = crate::Reg<i3c_cfgr::I3C_CFGR_SPEC>;
#[doc = "I3C configuration register"]
pub mod i3c_cfgr;
#[doc = "I3C_RDR (r) register accessor: an alias for `Reg<I3C_RDR_SPEC>`"]
pub type I3C_RDR = crate::Reg<i3c_rdr::I3C_RDR_SPEC>;
#[doc = "I3C receive data byte register"]
pub mod i3c_rdr;
#[doc = "I3C_RDWR (r) register accessor: an alias for `Reg<I3C_RDWR_SPEC>`"]
pub type I3C_RDWR = crate::Reg<i3c_rdwr::I3C_RDWR_SPEC>;
#[doc = "I3C receive data word register"]
pub mod i3c_rdwr;
#[doc = "I3C_TDR (w) register accessor: an alias for `Reg<I3C_TDR_SPEC>`"]
pub type I3C_TDR = crate::Reg<i3c_tdr::I3C_TDR_SPEC>;
#[doc = "I3C transmit data byte register"]
pub mod i3c_tdr;
#[doc = "I3C_TDWR (w) register accessor: an alias for `Reg<I3C_TDWR_SPEC>`"]
pub type I3C_TDWR = crate::Reg<i3c_tdwr::I3C_TDWR_SPEC>;
#[doc = "I3C transmit data word register"]
pub mod i3c_tdwr;
#[doc = "I3C_IBIDR (rw) register accessor: an alias for `Reg<I3C_IBIDR_SPEC>`"]
pub type I3C_IBIDR = crate::Reg<i3c_ibidr::I3C_IBIDR_SPEC>;
#[doc = "I3C IBI payload data register"]
pub mod i3c_ibidr;
#[doc = "I3C_TGTTDR (rw) register accessor: an alias for `Reg<I3C_TGTTDR_SPEC>`"]
pub type I3C_TGTTDR = crate::Reg<i3c_tgttdr::I3C_TGTTDR_SPEC>;
#[doc = "I3C target transmit configuration register"]
pub mod i3c_tgttdr;
#[doc = "I3C_SR (r) register accessor: an alias for `Reg<I3C_SR_SPEC>`"]
pub type I3C_SR = crate::Reg<i3c_sr::I3C_SR_SPEC>;
#[doc = "I3C status register"]
pub mod i3c_sr;
#[doc = "I3C_SER (r) register accessor: an alias for `Reg<I3C_SER_SPEC>`"]
pub type I3C_SER = crate::Reg<i3c_ser::I3C_SER_SPEC>;
#[doc = "I3C status error register"]
pub mod i3c_ser;
#[doc = "I3C_RMR (r) register accessor: an alias for `Reg<I3C_RMR_SPEC>`"]
pub type I3C_RMR = crate::Reg<i3c_rmr::I3C_RMR_SPEC>;
#[doc = "I3C received message register"]
pub mod i3c_rmr;
#[doc = "I3C_EVR (r) register accessor: an alias for `Reg<I3C_EVR_SPEC>`"]
pub type I3C_EVR = crate::Reg<i3c_evr::I3C_EVR_SPEC>;
#[doc = "I3C event register"]
pub mod i3c_evr;
#[doc = "I3C_IER (r) register accessor: an alias for `Reg<I3C_IER_SPEC>`"]
pub type I3C_IER = crate::Reg<i3c_ier::I3C_IER_SPEC>;
#[doc = "I3C interrupt enable register"]
pub mod i3c_ier;
#[doc = "I3C_CEVR (w) register accessor: an alias for `Reg<I3C_CEVR_SPEC>`"]
pub type I3C_CEVR = crate::Reg<i3c_cevr::I3C_CEVR_SPEC>;
#[doc = "I3C clear event register"]
pub mod i3c_cevr;
#[doc = "I3C_DEVR0 (rw) register accessor: an alias for `Reg<I3C_DEVR0_SPEC>`"]
pub type I3C_DEVR0 = crate::Reg<i3c_devr0::I3C_DEVR0_SPEC>;
#[doc = "I3C own device characteristics register"]
pub mod i3c_devr0;
#[doc = "I3C_DEVR1 (rw) register accessor: an alias for `Reg<I3C_DEVR1_SPEC>`"]
pub type I3C_DEVR1 = crate::Reg<i3c_devr1::I3C_DEVR1_SPEC>;
#[doc = "I3C device 1 characteristics register"]
pub mod i3c_devr1;
#[doc = "I3C_DEVR2 (rw) register accessor: an alias for `Reg<I3C_DEVR2_SPEC>`"]
pub type I3C_DEVR2 = crate::Reg<i3c_devr2::I3C_DEVR2_SPEC>;
#[doc = "I3C device 2 characteristics register"]
pub mod i3c_devr2;
#[doc = "I3C_DEVR3 (rw) register accessor: an alias for `Reg<I3C_DEVR3_SPEC>`"]
pub type I3C_DEVR3 = crate::Reg<i3c_devr3::I3C_DEVR3_SPEC>;
#[doc = "I3C device 3 characteristics register"]
pub mod i3c_devr3;
#[doc = "I3C_DEVR4 (rw) register accessor: an alias for `Reg<I3C_DEVR4_SPEC>`"]
pub type I3C_DEVR4 = crate::Reg<i3c_devr4::I3C_DEVR4_SPEC>;
#[doc = "I3C device 4 characteristics register"]
pub mod i3c_devr4;
#[doc = "I3C_MAXRLR (rw) register accessor: an alias for `Reg<I3C_MAXRLR_SPEC>`"]
pub type I3C_MAXRLR = crate::Reg<i3c_maxrlr::I3C_MAXRLR_SPEC>;
#[doc = "I3C maximum read length register"]
pub mod i3c_maxrlr;
#[doc = "I3C_MAXWLR (rw) register accessor: an alias for `Reg<I3C_MAXWLR_SPEC>`"]
pub type I3C_MAXWLR = crate::Reg<i3c_maxwlr::I3C_MAXWLR_SPEC>;
#[doc = "I3C maximum write length register"]
pub mod i3c_maxwlr;
#[doc = "I3C_TIMINGR0 (rw) register accessor: an alias for `Reg<I3C_TIMINGR0_SPEC>`"]
pub type I3C_TIMINGR0 = crate::Reg<i3c_timingr0::I3C_TIMINGR0_SPEC>;
#[doc = "I3C timing register 0"]
pub mod i3c_timingr0;
#[doc = "I3C_TIMINGR1 (rw) register accessor: an alias for `Reg<I3C_TIMINGR1_SPEC>`"]
pub type I3C_TIMINGR1 = crate::Reg<i3c_timingr1::I3C_TIMINGR1_SPEC>;
#[doc = "I3C timing register 1"]
pub mod i3c_timingr1;
#[doc = "I3C_TIMINGR2 (rw) register accessor: an alias for `Reg<I3C_TIMINGR2_SPEC>`"]
pub type I3C_TIMINGR2 = crate::Reg<i3c_timingr2::I3C_TIMINGR2_SPEC>;
#[doc = "I3C timing register 2"]
pub mod i3c_timingr2;
#[doc = "I3C_BCR (rw) register accessor: an alias for `Reg<I3C_BCR_SPEC>`"]
pub type I3C_BCR = crate::Reg<i3c_bcr::I3C_BCR_SPEC>;
#[doc = "I3C bus characteristics register"]
pub mod i3c_bcr;
#[doc = "I3C_DCR (rw) register accessor: an alias for `Reg<I3C_DCR_SPEC>`"]
pub type I3C_DCR = crate::Reg<i3c_dcr::I3C_DCR_SPEC>;
#[doc = "I3C device characteristics register"]
pub mod i3c_dcr;
#[doc = "I3C_GETCAPR (rw) register accessor: an alias for `Reg<I3C_GETCAPR_SPEC>`"]
pub type I3C_GETCAPR = crate::Reg<i3c_getcapr::I3C_GETCAPR_SPEC>;
#[doc = "I3C get capability register"]
pub mod i3c_getcapr;
#[doc = "I3C_CRCAPR (rw) register accessor: an alias for `Reg<I3C_CRCAPR_SPEC>`"]
pub type I3C_CRCAPR = crate::Reg<i3c_crcapr::I3C_CRCAPR_SPEC>;
#[doc = "I3C controller-role capability register"]
pub mod i3c_crcapr;
#[doc = "I3C_GETMXDSR (rw) register accessor: an alias for `Reg<I3C_GETMXDSR_SPEC>`"]
pub type I3C_GETMXDSR = crate::Reg<i3c_getmxdsr::I3C_GETMXDSR_SPEC>;
#[doc = "I3C get capability register"]
pub mod i3c_getmxdsr;
#[doc = "I3C_EPIDR (rw) register accessor: an alias for `Reg<I3C_EPIDR_SPEC>`"]
pub type I3C_EPIDR = crate::Reg<i3c_epidr::I3C_EPIDR_SPEC>;
#[doc = "I3C extended provisioned ID register"]
pub mod i3c_epidr;
