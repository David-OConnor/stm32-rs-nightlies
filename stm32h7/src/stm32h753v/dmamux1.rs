#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register"]
    pub ccr: [CCR; 16],
    _reserved1: [u8; 0x40],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: CFR,
    _reserved3: [u8; 0x78],
    #[doc = "0x100..0x120 - DMAMux - DMA request generator channel x control register"]
    pub rgcr: [RGCR; 8],
    _reserved4: [u8; 0x20],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub rgcfr: RGCFR,
}
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod ccr;
#[doc = "RGCR (rw) register accessor: an alias for `Reg<RGCR_SPEC>`"]
pub type RGCR = crate::Reg<rgcr::RGCR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rgcr;
#[doc = "RGSR (r) register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
