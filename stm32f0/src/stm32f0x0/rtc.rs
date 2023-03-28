#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - time-stamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tafcr: TAFCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    _reserved15: [u8; 0x08],
    #[doc = "0x50..0x64 - backup register"]
    pub bkpr: [BKPR; 5],
}
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "date register"]
pub mod dr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "timestamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "timestamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "time-stamp sub second register"]
pub mod tsssr;
#[doc = "CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "TAFCR (rw) register accessor: an alias for `Reg<TAFCR_SPEC>`"]
pub type TAFCR = crate::Reg<tafcr::TAFCR_SPEC>;
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`"]
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
#[doc = "backup register"]
pub mod bkpr;
