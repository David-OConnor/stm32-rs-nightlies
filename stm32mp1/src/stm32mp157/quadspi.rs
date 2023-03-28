#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    pub quadspi_cr: QUADSPI_CR,
    #[doc = "0x04 - QUADSPI device configuration register"]
    pub quadspi_dcr: QUADSPI_DCR,
    #[doc = "0x08 - QUADSPI status register"]
    pub quadspi_sr: QUADSPI_SR,
    #[doc = "0x0c - QUADSPI flag clear register"]
    pub quadspi_fcr: QUADSPI_FCR,
    #[doc = "0x10 - QUADSPI data length register"]
    pub quadspi_dlr: QUADSPI_DLR,
    #[doc = "0x14 - QUADSPI communication configuration register"]
    pub quadspi_ccr: QUADSPI_CCR,
    #[doc = "0x18 - QUADSPI address register"]
    pub quadspi_ar: QUADSPI_AR,
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    pub quadspi_abr: QUADSPI_ABR,
    #[doc = "0x20 - QUADSPI data register"]
    pub quadspi_dr: QUADSPI_DR,
    #[doc = "0x24 - QUADSPI polling status mask register"]
    pub quadspi_psmkr: QUADSPI_PSMKR,
    #[doc = "0x28 - QUADSPI polling status match register"]
    pub quadspi_psmar: QUADSPI_PSMAR,
    #[doc = "0x2c - QUADSPI polling interval register"]
    pub quadspi_pir: QUADSPI_PIR,
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    pub quadspi_lptr: QUADSPI_LPTR,
    _reserved13: [u8; 0x03bc],
    #[doc = "0x3f0 - QUADSPI HW configuration register"]
    pub quadspi_hwcfgr: QUADSPI_HWCFGR,
    #[doc = "0x3f4 - QUADSPI version register"]
    pub quadspi_verr: QUADSPI_VERR,
    #[doc = "0x3f8 - QUADSPI identification register"]
    pub quadspi_ipidr: QUADSPI_IPIDR,
    #[doc = "0x3fc - QUADSPI size identification register"]
    pub quadspi_sidr: QUADSPI_SIDR,
}
#[doc = "QUADSPI_CR (rw) register accessor: an alias for `Reg<QUADSPI_CR_SPEC>`"]
pub type QUADSPI_CR = crate::Reg<quadspi_cr::QUADSPI_CR_SPEC>;
#[doc = "QUADSPI control register"]
pub mod quadspi_cr;
#[doc = "QUADSPI_DCR (rw) register accessor: an alias for `Reg<QUADSPI_DCR_SPEC>`"]
pub type QUADSPI_DCR = crate::Reg<quadspi_dcr::QUADSPI_DCR_SPEC>;
#[doc = "QUADSPI device configuration register"]
pub mod quadspi_dcr;
#[doc = "QUADSPI_SR (r) register accessor: an alias for `Reg<QUADSPI_SR_SPEC>`"]
pub type QUADSPI_SR = crate::Reg<quadspi_sr::QUADSPI_SR_SPEC>;
#[doc = "QUADSPI status register"]
pub mod quadspi_sr;
#[doc = "QUADSPI_FCR (w) register accessor: an alias for `Reg<QUADSPI_FCR_SPEC>`"]
pub type QUADSPI_FCR = crate::Reg<quadspi_fcr::QUADSPI_FCR_SPEC>;
#[doc = "QUADSPI flag clear register"]
pub mod quadspi_fcr;
#[doc = "QUADSPI_DLR (rw) register accessor: an alias for `Reg<QUADSPI_DLR_SPEC>`"]
pub type QUADSPI_DLR = crate::Reg<quadspi_dlr::QUADSPI_DLR_SPEC>;
#[doc = "QUADSPI data length register"]
pub mod quadspi_dlr;
#[doc = "QUADSPI_CCR (rw) register accessor: an alias for `Reg<QUADSPI_CCR_SPEC>`"]
pub type QUADSPI_CCR = crate::Reg<quadspi_ccr::QUADSPI_CCR_SPEC>;
#[doc = "QUADSPI communication configuration register"]
pub mod quadspi_ccr;
#[doc = "QUADSPI_AR (rw) register accessor: an alias for `Reg<QUADSPI_AR_SPEC>`"]
pub type QUADSPI_AR = crate::Reg<quadspi_ar::QUADSPI_AR_SPEC>;
#[doc = "QUADSPI address register"]
pub mod quadspi_ar;
#[doc = "QUADSPI_ABR (rw) register accessor: an alias for `Reg<QUADSPI_ABR_SPEC>`"]
pub type QUADSPI_ABR = crate::Reg<quadspi_abr::QUADSPI_ABR_SPEC>;
#[doc = "QUADSPI alternate bytes registers"]
pub mod quadspi_abr;
#[doc = "QUADSPI_DR (rw) register accessor: an alias for `Reg<QUADSPI_DR_SPEC>`"]
pub type QUADSPI_DR = crate::Reg<quadspi_dr::QUADSPI_DR_SPEC>;
#[doc = "QUADSPI data register"]
pub mod quadspi_dr;
#[doc = "QUADSPI_PSMKR (rw) register accessor: an alias for `Reg<QUADSPI_PSMKR_SPEC>`"]
pub type QUADSPI_PSMKR = crate::Reg<quadspi_psmkr::QUADSPI_PSMKR_SPEC>;
#[doc = "QUADSPI polling status mask register"]
pub mod quadspi_psmkr;
#[doc = "QUADSPI_PSMAR (rw) register accessor: an alias for `Reg<QUADSPI_PSMAR_SPEC>`"]
pub type QUADSPI_PSMAR = crate::Reg<quadspi_psmar::QUADSPI_PSMAR_SPEC>;
#[doc = "QUADSPI polling status match register"]
pub mod quadspi_psmar;
#[doc = "QUADSPI_PIR (rw) register accessor: an alias for `Reg<QUADSPI_PIR_SPEC>`"]
pub type QUADSPI_PIR = crate::Reg<quadspi_pir::QUADSPI_PIR_SPEC>;
#[doc = "QUADSPI polling interval register"]
pub mod quadspi_pir;
#[doc = "QUADSPI_LPTR (rw) register accessor: an alias for `Reg<QUADSPI_LPTR_SPEC>`"]
pub type QUADSPI_LPTR = crate::Reg<quadspi_lptr::QUADSPI_LPTR_SPEC>;
#[doc = "QUADSPI low-power timeout register"]
pub mod quadspi_lptr;
#[doc = "QUADSPI_HWCFGR (r) register accessor: an alias for `Reg<QUADSPI_HWCFGR_SPEC>`"]
pub type QUADSPI_HWCFGR = crate::Reg<quadspi_hwcfgr::QUADSPI_HWCFGR_SPEC>;
#[doc = "QUADSPI HW configuration register"]
pub mod quadspi_hwcfgr;
#[doc = "QUADSPI_VERR (r) register accessor: an alias for `Reg<QUADSPI_VERR_SPEC>`"]
pub type QUADSPI_VERR = crate::Reg<quadspi_verr::QUADSPI_VERR_SPEC>;
#[doc = "QUADSPI version register"]
pub mod quadspi_verr;
#[doc = "QUADSPI_IPIDR (r) register accessor: an alias for `Reg<QUADSPI_IPIDR_SPEC>`"]
pub type QUADSPI_IPIDR = crate::Reg<quadspi_ipidr::QUADSPI_IPIDR_SPEC>;
#[doc = "QUADSPI identification register"]
pub mod quadspi_ipidr;
#[doc = "QUADSPI_SIDR (r) register accessor: an alias for `Reg<QUADSPI_SIDR_SPEC>`"]
pub type QUADSPI_SIDR = crate::Reg<quadspi_sidr::QUADSPI_SIDR_SPEC>;
#[doc = "QUADSPI size identification register"]
pub mod quadspi_sidr;
