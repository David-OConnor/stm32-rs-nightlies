#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub tr: TR,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub dr: DR,
    #[doc = "0x08 - RTC sub second register"]
    pub ssr: SSR,
    #[doc = "0x0c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub icsr: ICSR,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub prer: PRER,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub wutr: WUTR,
    #[doc = "0x18 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub cr: CR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub smcr: SMCR,
    #[doc = "0x24 - RTC write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub calr: CALR,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub tstr: TSTR,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub tsdr: TSDR,
    #[doc = "0x38 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - Alarm register"]
    pub alrmar: ALRMR,
    #[doc = "0x44 - Alarm sub-second register"]
    pub alrmassr: ALRMSSR,
    #[doc = "0x48 - Alarm register"]
    pub alrmbr: ALRMR,
    #[doc = "0x4c - Alarm sub-second register"]
    pub alrmbssr: ALRMSSR,
    #[doc = "0x50 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub sr: SR,
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x58 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub smisr: SMISR,
    #[doc = "0x5c - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub scr: SCR,
    #[doc = "0x60 - RTC configuration register"]
    pub cfgr: CFGR,
    _reserved23: [u8; 0x038c],
    #[doc = "0x3f0 - RTC hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - RTC version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - RTC identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - RTC size identification register"]
    pub sidr: SIDR,
}
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod tr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod dr;
#[doc = "SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod icsr;
#[doc = "PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod prer;
#[doc = "WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod wutr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod cr;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "This register can be written only when the APB access is secure."]
pub mod smcr;
#[doc = "WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
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
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod sr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "RTC non-secure masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`"]
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod smisr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod scr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "RTC configuration register"]
pub mod cfgr;
#[doc = "HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "RTC hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "RTC version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "RTC identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "RTC size identification register"]
pub mod sidr;
