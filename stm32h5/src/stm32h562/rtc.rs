#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    pub tr: TR,
    #[doc = "0x04 - RTC date register"]
    pub dr: DR,
    #[doc = "0x08 - RTC sub second register"]
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
    #[doc = "0x20 - RTC secure configuration register"]
    pub seccfgr: SECCFGR,
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
    #[doc = "0x38 - RTC timestamp sub second register"]
    pub tsssr: TSSSR,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - Alarm register"]
    pub alrmar: ALRMR,
    #[doc = "0x44 - Alarm sub-second register"]
    pub alrmassr: ALRMSSR,
    #[doc = "0x48 - Alarm register"]
    pub alrmbr: ALRMR,
    #[doc = "0x4c - Alarm sub-second register"]
    pub alrmbssr: ALRMSSR,
    #[doc = "0x50 - RTC status register"]
    pub sr: SR,
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x58 - RTC secure masked interrupt status register"]
    pub smisr: SMISR,
    #[doc = "0x5c - RTC status clear register"]
    pub scr: SCR,
    #[doc = "0x60 - RTC option register"]
    pub or: OR,
    _reserved24: [u8; 0x0c],
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
#[doc = "RTC sub second register"]
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
#[doc = "SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
#[doc = "RTC secure configuration register"]
pub mod seccfgr;
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
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
#[doc = "ALRMR (rw) register accessor: an alias for `Reg<ALRMR_SPEC>`"]
pub type ALRMR = crate::Reg<alrmr::ALRMR_SPEC>;
#[doc = "Alarm register"]
pub mod alrmr;
#[doc = "ALRMSSR (rw) register accessor: an alias for `Reg<ALRMSSR_SPEC>`"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSR_SPEC>;
#[doc = "Alarm sub-second register"]
pub mod alrmssr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RTC status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "RTC non-secure masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`"]
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
#[doc = "RTC secure masked interrupt status register"]
pub mod smisr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "RTC status clear register"]
pub mod scr;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "RTC option register"]
pub mod or;
#[doc = "ALRABINR (rw) register accessor: an alias for `Reg<ALRABINR_SPEC>`"]
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
#[doc = "RTC alarm A binary mode register"]
pub mod alrabinr;
#[doc = "ALRBBINR (rw) register accessor: an alias for `Reg<ALRBBINR_SPEC>`"]
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
#[doc = "RTC alarm B binary mode register"]
pub mod alrbbinr;
