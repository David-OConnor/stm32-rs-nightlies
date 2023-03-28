#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    pub cr: CR,
    #[doc = "0x04 - ICACHE status register"]
    pub sr: SR,
    #[doc = "0x08 - ICACHE interrupt enable register"]
    pub ier: IER,
    #[doc = "0x0c - ICACHE flag clear register"]
    pub fcr: FCR,
    #[doc = "0x10 - ICACHE hit monitor register"]
    pub hmonr: HMONR,
    #[doc = "0x14 - ICACHE miss monitor register"]
    pub mmonr: MMONR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20..0x30 - ICACHE region configuration register"]
    pub crr: [CRR; 4],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ICACHE control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ICACHE status register"]
pub mod sr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ICACHE interrupt enable register"]
pub mod ier;
#[doc = "FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "ICACHE flag clear register"]
pub mod fcr;
#[doc = "HMONR (r) register accessor: an alias for `Reg<HMONR_SPEC>`"]
pub type HMONR = crate::Reg<hmonr::HMONR_SPEC>;
#[doc = "ICACHE hit monitor register"]
pub mod hmonr;
#[doc = "MMONR (r) register accessor: an alias for `Reg<MMONR_SPEC>`"]
pub type MMONR = crate::Reg<mmonr::MMONR_SPEC>;
#[doc = "ICACHE miss monitor register"]
pub mod mmonr;
#[doc = "CRR (rw) register accessor: an alias for `Reg<CRR_SPEC>`"]
pub type CRR = crate::Reg<crr::CRR_SPEC>;
#[doc = "ICACHE region configuration register"]
pub mod crr;
