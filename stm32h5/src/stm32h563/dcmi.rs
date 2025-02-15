#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCMI control register"]
    pub dcmi_cr: DCMI_CR,
    #[doc = "0x04 - DCMI status register"]
    pub dcmi_sr: DCMI_SR,
    #[doc = "0x08 - DCMI raw interrupt status register"]
    pub dcmi_ris: DCMI_RIS,
    #[doc = "0x0c - DCMI interrupt enable register"]
    pub dcmi_ier: DCMI_IER,
    #[doc = "0x10 - DCMI masked interrupt status register"]
    pub dcmi_mis: DCMI_MIS,
    #[doc = "0x14 - DCMI interrupt clear register"]
    pub dcmi_icr: DCMI_ICR,
    #[doc = "0x18 - DCMI embedded synchronization code register"]
    pub dcmi_escr: DCMI_ESCR,
    #[doc = "0x1c - DCMI embedded synchronization unmask register"]
    pub dcmi_esur: DCMI_ESUR,
    #[doc = "0x20 - DCMI crop window start"]
    pub dcmi_cwstrt: DCMI_CWSTRT,
    #[doc = "0x24 - DCMI crop window size"]
    pub dcmi_cwsize: DCMI_CWSIZE,
    #[doc = "0x28 - DCMI data register"]
    pub dcmi_dr: DCMI_DR,
}
#[doc = "DCMI_CR (rw) register accessor: an alias for `Reg<DCMI_CR_SPEC>`"]
pub type DCMI_CR = crate::Reg<dcmi_cr::DCMI_CR_SPEC>;
#[doc = "DCMI control register"]
pub mod dcmi_cr;
#[doc = "DCMI_SR (r) register accessor: an alias for `Reg<DCMI_SR_SPEC>`"]
pub type DCMI_SR = crate::Reg<dcmi_sr::DCMI_SR_SPEC>;
#[doc = "DCMI status register"]
pub mod dcmi_sr;
#[doc = "DCMI_RIS (r) register accessor: an alias for `Reg<DCMI_RIS_SPEC>`"]
pub type DCMI_RIS = crate::Reg<dcmi_ris::DCMI_RIS_SPEC>;
#[doc = "DCMI raw interrupt status register"]
pub mod dcmi_ris;
#[doc = "DCMI_IER (rw) register accessor: an alias for `Reg<DCMI_IER_SPEC>`"]
pub type DCMI_IER = crate::Reg<dcmi_ier::DCMI_IER_SPEC>;
#[doc = "DCMI interrupt enable register"]
pub mod dcmi_ier;
#[doc = "DCMI_MIS (r) register accessor: an alias for `Reg<DCMI_MIS_SPEC>`"]
pub type DCMI_MIS = crate::Reg<dcmi_mis::DCMI_MIS_SPEC>;
#[doc = "DCMI masked interrupt status register"]
pub mod dcmi_mis;
#[doc = "DCMI_ICR (w) register accessor: an alias for `Reg<DCMI_ICR_SPEC>`"]
pub type DCMI_ICR = crate::Reg<dcmi_icr::DCMI_ICR_SPEC>;
#[doc = "DCMI interrupt clear register"]
pub mod dcmi_icr;
#[doc = "DCMI_ESCR (rw) register accessor: an alias for `Reg<DCMI_ESCR_SPEC>`"]
pub type DCMI_ESCR = crate::Reg<dcmi_escr::DCMI_ESCR_SPEC>;
#[doc = "DCMI embedded synchronization code register"]
pub mod dcmi_escr;
#[doc = "DCMI_ESUR (rw) register accessor: an alias for `Reg<DCMI_ESUR_SPEC>`"]
pub type DCMI_ESUR = crate::Reg<dcmi_esur::DCMI_ESUR_SPEC>;
#[doc = "DCMI embedded synchronization unmask register"]
pub mod dcmi_esur;
#[doc = "DCMI_CWSTRT (rw) register accessor: an alias for `Reg<DCMI_CWSTRT_SPEC>`"]
pub type DCMI_CWSTRT = crate::Reg<dcmi_cwstrt::DCMI_CWSTRT_SPEC>;
#[doc = "DCMI crop window start"]
pub mod dcmi_cwstrt;
#[doc = "DCMI_CWSIZE (rw) register accessor: an alias for `Reg<DCMI_CWSIZE_SPEC>`"]
pub type DCMI_CWSIZE = crate::Reg<dcmi_cwsize::DCMI_CWSIZE_SPEC>;
#[doc = "DCMI crop window size"]
pub mod dcmi_cwsize;
#[doc = "DCMI_DR (r) register accessor: an alias for `Reg<DCMI_DR_SPEC>`"]
pub type DCMI_DR = crate::Reg<dcmi_dr::DCMI_DR_SPEC>;
#[doc = "DCMI data register"]
pub mod dcmi_dr;
