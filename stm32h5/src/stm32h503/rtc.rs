#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    pub tr: TR,
    #[doc = "0x04 - RTC date register"]
    pub dr: DR,
    #[doc = "0x08 - RTC subsecond register"]
    pub ssr: SSR,
    #[doc = "0x0c - RTC initialization control and status register"]
    pub icsr: ICSR,
    #[doc = "0x10 - RTC prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - RTC wakeup timer register"]
    pub wutr: WUTR,
    #[doc = "0x18 - RTC control register"]
    pub cr: CR,
    #[doc = "0x1c - RTC privilege mode control register"]
    pub privcfgr: PRIVCFGR,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - RTC write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - RTC calibration register"]
    pub calr: CALR,
    #[doc = "0x2c - RTC shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - RTC timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - RTC timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - RTC timestamp subsecond register"]
    pub tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - RTC alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x44 - RTC alarm A subsecond register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - RTC alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x4c - RTC alarm B subsecond register"]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x50 - RTC status register"]
    pub sr: SR,
    #[doc = "0x54 - RTC masked interrupt status register"]
    pub misr: MISR,
    _reserved20: [u8; 0x04],
    #[doc = "0x5c - RTC status clear register"]
    pub scr: SCR,
    _reserved21: [u8; 0x10],
    #[doc = "0x70 - RTC alarm A binary mode register"]
    pub alrabinr: ALRABINR,
    #[doc = "0x74 - RTC alarm B binary mode register"]
    pub alrbbinr: ALRBBINR,
}
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "RTC time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "RTC date register"]
pub mod dr;
#[doc = "SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "RTC subsecond register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "RTC initialization control and status register"]
pub mod icsr;
#[doc = "PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "RTC prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "RTC wakeup timer register"]
pub mod wutr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "RTC privilege mode control register"]
pub mod privcfgr;
#[doc = "WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "RTC shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "RTC timestamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "RTC timestamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "RTC timestamp subsecond register"]
pub mod tsssr;
#[doc = "ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "RTC alarm A register"]
pub mod alrmar;
#[doc = "ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "RTC alarm A subsecond register"]
pub mod alrmassr;
#[doc = "ALRMBR (rw) register accessor: an alias for `Reg<ALRMBR_SPEC>`"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "RTC alarm B register"]
pub mod alrmbr;
#[doc = "ALRMBSSR (rw) register accessor: an alias for `Reg<ALRMBSSR_SPEC>`"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "RTC alarm B subsecond register"]
pub mod alrmbssr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RTC status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "RTC masked interrupt status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "RTC status clear register"]
pub mod scr;
#[doc = "ALRABINR (rw) register accessor: an alias for `Reg<ALRABINR_SPEC>`"]
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
#[doc = "RTC alarm A binary mode register"]
pub mod alrabinr;
#[doc = "ALRBBINR (rw) register accessor: an alias for `Reg<ALRBBINR_SPEC>`"]
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
#[doc = "RTC alarm B binary mode register"]
pub mod alrbbinr;
