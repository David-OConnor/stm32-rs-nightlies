#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: HCFG,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: HPRT,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100..0x300 - Host channel"]
    pub hc: [HC; 16],
}
#[doc = "HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "Host channel"]
pub use self::hc::HC;
#[doc = r"Cluster"]
#[doc = "Host channel"]
pub mod hc;
