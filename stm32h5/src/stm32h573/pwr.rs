#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR power mode control register"]
    pub pmcr: PMCR,
    #[doc = "0x04 - PWR status register"]
    pub pmsr: PMSR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - PWR voltage scaling control register"]
    pub voscr: VOSCR,
    #[doc = "0x14 - PWR voltage scaling status register"]
    pub vossr: VOSSR,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - PWR Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x24 - PWR Backup domain control register"]
    pub dbpcr: DBPCR,
    #[doc = "0x28 - PWR Backup domain status register"]
    pub bdsr: BDSR,
    #[doc = "0x2c - PWR USB Type-C power delivery register"]
    pub ucpdr: UCPDR,
    #[doc = "0x30 - PWR supply configuration control register"]
    pub sccr: SCCR,
    #[doc = "0x34 - PWR voltage monitor control register"]
    pub vmcr: VMCR,
    #[doc = "0x38 - PWR USB supply control register"]
    pub usbscr: USBSCR,
    #[doc = "0x3c - PWR voltage monitor status register"]
    pub vmsr: VMSR,
    #[doc = "0x40 - PWR wakeup status clear register"]
    pub wuscr: WUSCR,
    #[doc = "0x44 - PWR wakeup status register"]
    pub wusr: WUSR,
    #[doc = "0x48 - PWR wakeup configuration register"]
    pub wucr: WUCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - PWR I/O retention register"]
    pub ioretr: IORETR,
    _reserved16: [u8; 0xac],
    #[doc = "0x100 - PWR security configuration register"]
    pub seccfgr: SECCFGR,
    #[doc = "0x104 - PWR privilege configuration register"]
    pub privcfgr: PRIVCFGR,
}
#[doc = "PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "PWR power mode control register"]
pub mod pmcr;
#[doc = "PMSR (r) register accessor: an alias for `Reg<PMSR_SPEC>`"]
pub type PMSR = crate::Reg<pmsr::PMSR_SPEC>;
#[doc = "PWR status register"]
pub mod pmsr;
#[doc = "VOSCR (rw) register accessor: an alias for `Reg<VOSCR_SPEC>`"]
pub type VOSCR = crate::Reg<voscr::VOSCR_SPEC>;
#[doc = "PWR voltage scaling control register"]
pub mod voscr;
#[doc = "VOSSR (r) register accessor: an alias for `Reg<VOSSR_SPEC>`"]
pub type VOSSR = crate::Reg<vossr::VOSSR_SPEC>;
#[doc = "PWR voltage scaling status register"]
pub mod vossr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "PWR Backup domain control register"]
pub mod bdcr;
#[doc = "DBPCR (rw) register accessor: an alias for `Reg<DBPCR_SPEC>`"]
pub type DBPCR = crate::Reg<dbpcr::DBPCR_SPEC>;
#[doc = "PWR Backup domain control register"]
pub mod dbpcr;
#[doc = "BDSR (r) register accessor: an alias for `Reg<BDSR_SPEC>`"]
pub type BDSR = crate::Reg<bdsr::BDSR_SPEC>;
#[doc = "PWR Backup domain status register"]
pub mod bdsr;
#[doc = "UCPDR (rw) register accessor: an alias for `Reg<UCPDR_SPEC>`"]
pub type UCPDR = crate::Reg<ucpdr::UCPDR_SPEC>;
#[doc = "PWR USB Type-C power delivery register"]
pub mod ucpdr;
#[doc = "SCCR (rw) register accessor: an alias for `Reg<SCCR_SPEC>`"]
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
#[doc = "PWR supply configuration control register"]
pub mod sccr;
#[doc = "VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "PWR voltage monitor control register"]
pub mod vmcr;
#[doc = "USBSCR (rw) register accessor: an alias for `Reg<USBSCR_SPEC>`"]
pub type USBSCR = crate::Reg<usbscr::USBSCR_SPEC>;
#[doc = "PWR USB supply control register"]
pub mod usbscr;
#[doc = "VMSR (r) register accessor: an alias for `Reg<VMSR_SPEC>`"]
pub type VMSR = crate::Reg<vmsr::VMSR_SPEC>;
#[doc = "PWR voltage monitor status register"]
pub mod vmsr;
#[doc = "WUSCR (w) register accessor: an alias for `Reg<WUSCR_SPEC>`"]
pub type WUSCR = crate::Reg<wuscr::WUSCR_SPEC>;
#[doc = "PWR wakeup status clear register"]
pub mod wuscr;
#[doc = "WUSR (r) register accessor: an alias for `Reg<WUSR_SPEC>`"]
pub type WUSR = crate::Reg<wusr::WUSR_SPEC>;
#[doc = "PWR wakeup status register"]
pub mod wusr;
#[doc = "WUCR (rw) register accessor: an alias for `Reg<WUCR_SPEC>`"]
pub type WUCR = crate::Reg<wucr::WUCR_SPEC>;
#[doc = "PWR wakeup configuration register"]
pub mod wucr;
#[doc = "IORETR (rw) register accessor: an alias for `Reg<IORETR_SPEC>`"]
pub type IORETR = crate::Reg<ioretr::IORETR_SPEC>;
#[doc = "PWR I/O retention register"]
pub mod ioretr;
#[doc = "SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
#[doc = "PWR security configuration register"]
pub mod seccfgr;
#[doc = "PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "PWR privilege configuration register"]
pub mod privcfgr;
