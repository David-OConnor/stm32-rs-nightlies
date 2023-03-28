#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x08 - Status register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Flag Clear register"]
    pub ifcr: IFCR,
    #[doc = "0x10 - Data input register"]
    pub dr: DR,
    #[doc = "0x14 - Channel Status register"]
    pub csr: CSR,
    #[doc = "0x18 - Debug Information register"]
    pub dir: DIR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data input register"]
pub mod dr;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "DIR (r) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Debug Information register"]
pub mod dir;
