#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMAC X1 buffer configuration register"]
    pub fmac_x1bufcfg: FMAC_X1BUFCFG,
    #[doc = "0x04 - FMAC X2 buffer configuration register"]
    pub fmac_x2bufcfg: FMAC_X2BUFCFG,
    #[doc = "0x08 - FMAC Y buffer configuration register"]
    pub fmac_ybufcfg: FMAC_YBUFCFG,
    #[doc = "0x0c - FMAC parameter register"]
    pub fmac_param: FMAC_PARAM,
    #[doc = "0x10 - FMAC control register"]
    pub fmac_cr: FMAC_CR,
    #[doc = "0x14 - FMAC status register"]
    pub fmac_sr: FMAC_SR,
    #[doc = "0x18 - FMAC write data register"]
    pub fmac_wdata: FMAC_WDATA,
    #[doc = "0x1c - FMAC read data register"]
    pub fmac_rdata: FMAC_RDATA,
}
#[doc = "FMAC_X1BUFCFG (rw) register accessor: an alias for `Reg<FMAC_X1BUFCFG_SPEC>`"]
pub type FMAC_X1BUFCFG = crate::Reg<fmac_x1bufcfg::FMAC_X1BUFCFG_SPEC>;
#[doc = "FMAC X1 buffer configuration register"]
pub mod fmac_x1bufcfg;
#[doc = "FMAC_X2BUFCFG (rw) register accessor: an alias for `Reg<FMAC_X2BUFCFG_SPEC>`"]
pub type FMAC_X2BUFCFG = crate::Reg<fmac_x2bufcfg::FMAC_X2BUFCFG_SPEC>;
#[doc = "FMAC X2 buffer configuration register"]
pub mod fmac_x2bufcfg;
#[doc = "FMAC_YBUFCFG (rw) register accessor: an alias for `Reg<FMAC_YBUFCFG_SPEC>`"]
pub type FMAC_YBUFCFG = crate::Reg<fmac_ybufcfg::FMAC_YBUFCFG_SPEC>;
#[doc = "FMAC Y buffer configuration register"]
pub mod fmac_ybufcfg;
#[doc = "FMAC_PARAM (rw) register accessor: an alias for `Reg<FMAC_PARAM_SPEC>`"]
pub type FMAC_PARAM = crate::Reg<fmac_param::FMAC_PARAM_SPEC>;
#[doc = "FMAC parameter register"]
pub mod fmac_param;
#[doc = "FMAC_CR (rw) register accessor: an alias for `Reg<FMAC_CR_SPEC>`"]
pub type FMAC_CR = crate::Reg<fmac_cr::FMAC_CR_SPEC>;
#[doc = "FMAC control register"]
pub mod fmac_cr;
#[doc = "FMAC_SR (r) register accessor: an alias for `Reg<FMAC_SR_SPEC>`"]
pub type FMAC_SR = crate::Reg<fmac_sr::FMAC_SR_SPEC>;
#[doc = "FMAC status register"]
pub mod fmac_sr;
#[doc = "FMAC_WDATA (w) register accessor: an alias for `Reg<FMAC_WDATA_SPEC>`"]
pub type FMAC_WDATA = crate::Reg<fmac_wdata::FMAC_WDATA_SPEC>;
#[doc = "FMAC write data register"]
pub mod fmac_wdata;
#[doc = "FMAC_RDATA (r) register accessor: an alias for `Reg<FMAC_RDATA_SPEC>`"]
pub type FMAC_RDATA = crate::Reg<fmac_rdata::FMAC_RDATA_SPEC>;
#[doc = "FMAC read data register"]
pub mod fmac_rdata;
