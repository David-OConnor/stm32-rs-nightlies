#[doc = r"Register block"]
#[repr(C)]
pub struct LAYER {
    #[doc = "0x00 - Layerx Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Layerx Window Horizontal Position Configuration Register"]
    pub whpcr: WHPCR,
    #[doc = "0x08 - Layerx Window Vertical Position Configuration Register"]
    pub wvpcr: WVPCR,
    #[doc = "0x0c - Layerx Color Keying Configuration Register"]
    pub ckcr: CKCR,
    #[doc = "0x10 - Layerx Pixel Format Configuration Register"]
    pub pfcr: PFCR,
    #[doc = "0x14 - Layerx Constant Alpha Configuration Register"]
    pub cacr: CACR,
    #[doc = "0x18 - Layerx Default Color Configuration Register"]
    pub dccr: DCCR,
    #[doc = "0x1c - Layerx Blending Factors Configuration Register"]
    pub bfcr: BFCR,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Layerx Color Frame Buffer Address Register"]
    pub cfbar: CFBAR,
    #[doc = "0x2c - Layerx Color Frame Buffer Length Register"]
    pub cfblr: CFBLR,
    #[doc = "0x30 - Layerx ColorFrame Buffer Line Number Register"]
    pub cfblnr: CFBLNR,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - Layerx CLUT Write Register"]
    pub clutwr: CLUTWR,
    _reserved_end: [u8; 0x3c],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Layerx Control Register"]
pub mod cr;
#[doc = "WHPCR (rw) register accessor: an alias for `Reg<WHPCR_SPEC>`"]
pub type WHPCR = crate::Reg<whpcr::WHPCR_SPEC>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod whpcr;
#[doc = "WVPCR (rw) register accessor: an alias for `Reg<WVPCR_SPEC>`"]
pub type WVPCR = crate::Reg<wvpcr::WVPCR_SPEC>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod wvpcr;
#[doc = "CKCR (rw) register accessor: an alias for `Reg<CKCR_SPEC>`"]
pub type CKCR = crate::Reg<ckcr::CKCR_SPEC>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod ckcr;
#[doc = "PFCR (rw) register accessor: an alias for `Reg<PFCR_SPEC>`"]
pub type PFCR = crate::Reg<pfcr::PFCR_SPEC>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod pfcr;
#[doc = "CACR (rw) register accessor: an alias for `Reg<CACR_SPEC>`"]
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod cacr;
#[doc = "DCCR (rw) register accessor: an alias for `Reg<DCCR_SPEC>`"]
pub type DCCR = crate::Reg<dccr::DCCR_SPEC>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod dccr;
#[doc = "BFCR (rw) register accessor: an alias for `Reg<BFCR_SPEC>`"]
pub type BFCR = crate::Reg<bfcr::BFCR_SPEC>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod bfcr;
#[doc = "CFBAR (rw) register accessor: an alias for `Reg<CFBAR_SPEC>`"]
pub type CFBAR = crate::Reg<cfbar::CFBAR_SPEC>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod cfbar;
#[doc = "CFBLR (rw) register accessor: an alias for `Reg<CFBLR_SPEC>`"]
pub type CFBLR = crate::Reg<cfblr::CFBLR_SPEC>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod cfblr;
#[doc = "CFBLNR (rw) register accessor: an alias for `Reg<CFBLNR_SPEC>`"]
pub type CFBLNR = crate::Reg<cfblnr::CFBLNR_SPEC>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod cfblnr;
#[doc = "CLUTWR (w) register accessor: an alias for `Reg<CLUTWR_SPEC>`"]
pub type CLUTWR = crate::Reg<clutwr::CLUTWR_SPEC>;
#[doc = "Layerx CLUT Write Register"]
pub mod clutwr;
