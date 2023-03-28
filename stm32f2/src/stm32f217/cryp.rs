#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub din: DIN,
    #[doc = "0x0c - data output register"]
    pub dout: DOUT,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: DMACR,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR"]
    pub key: [KEY; 4],
    #[doc = "0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR"]
    pub init: [INIT; 2],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "DOUT (r) register accessor: an alias for `Reg<DOUT_SPEC>`"]
pub type DOUT = crate::Reg<dout::DOUT_SPEC>;
#[doc = "data output register"]
pub mod dout;
#[doc = "DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "IMSCR (rw) register accessor: an alias for `Reg<IMSCR_SPEC>`"]
pub type IMSCR = crate::Reg<imscr::IMSCR_SPEC>;
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "RISR (r) register accessor: an alias for `Reg<RISR_SPEC>`"]
pub type RISR = crate::Reg<risr::RISR_SPEC>;
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub use self::key::KEY;
#[doc = r"Cluster"]
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub mod key;
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub use self::init::INIT;
#[doc = r"Cluster"]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub mod init;
