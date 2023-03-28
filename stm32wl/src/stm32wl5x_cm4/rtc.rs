#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Time register"]
    pub tr: TR,
    #[doc = "0x04 - Date register"]
    pub dr: DR,
    #[doc = "0x08 - Sub second register"]
    pub ssr: SSR,
    #[doc = "0x0c - Initialization control and status register"]
    pub icsr: ICSR,
    #[doc = "0x10 - Pre-scaler register"]
    pub prer: PRER,
    #[doc = "0x14 - Wakeup timer register"]
    pub wutr: WUTR,
    #[doc = "0x18 - Control register"]
    pub cr: CR,
    _reserved7: [u8; 0x08],
    #[doc = "0x24 - Write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - Calibration register"]
    pub calr: CALR,
    #[doc = "0x2c - Shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - Timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - Timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - Timestamp sub second register"]
    pub tsssr: TSSSR,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - Alarm register"]
    pub alrmar: ALRMR,
    #[doc = "0x44 - Alarm sub-second register"]
    pub alrmassr: ALRMSSR,
    #[doc = "0x48 - Alarm register"]
    pub alrmbr: ALRMR,
    #[doc = "0x4c - Alarm sub-second register"]
    pub alrmbssr: ALRMSSR,
    #[doc = "0x50 - Status register (interrupts)"]
    pub sr: SR,
    #[doc = "0x54 - Masked interrupt status register"]
    pub misr: MISR,
    _reserved19: [u8; 0x04],
    #[doc = "0x5c - Status clear register (interrupts)"]
    pub scr: SCR,
    _reserved20: [u8; 0x10],
    #[doc = "0x70..0x78 - RTC alarm A binary mode register"]
    pub alrbinr: [ALRBINR; 2],
}
impl RegisterBlock {
    #[doc = "0x70 - RTC alarm A binary mode register"]
    #[inline(always)]
    pub fn alrabinr(&self) -> &ALRBINR {
        &self.alrbinr[0]
    }
    #[doc = "0x74 - RTC alarm A binary mode register"]
    #[inline(always)]
    pub fn alrbbinr(&self) -> &ALRBINR {
        &self.alrbinr[1]
    }
}
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Date register"]
pub mod dr;
#[doc = "SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Sub second register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Initialization control and status register"]
pub mod icsr;
#[doc = "PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "Pre-scaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "Wakeup timer register"]
pub mod wutr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "Calibration register"]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "Shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "Timestamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "Timestamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "Timestamp sub second register"]
pub mod tsssr;
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
#[doc = "Status register (interrupts)"]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "Masked interrupt status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status clear register (interrupts)"]
pub mod scr;
#[doc = "ALRBINR (rw) register accessor: an alias for `Reg<ALRBINR_SPEC>`"]
pub type ALRBINR = crate::Reg<alrbinr::ALRBINR_SPEC>;
#[doc = "RTC alarm A binary mode register"]
pub mod alrbinr;
