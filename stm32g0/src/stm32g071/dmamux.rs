#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c0cr: C0CR,
    #[doc = "0x04 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c1cr: C1CR,
    #[doc = "0x08 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c2cr: C2CR,
    #[doc = "0x0c - DMAMux - DMA request line multiplexer channel x control register"]
    pub c3cr: C3CR,
    #[doc = "0x10 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c4cr: C4CR,
    #[doc = "0x14 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c5cr: C5CR,
    #[doc = "0x18 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c6cr: C6CR,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: CFR,
    _reserved9: [u8; 0x78],
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    pub rg0cr: RG0CR,
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    pub rg1cr: RG1CR,
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    pub rg2cr: RG2CR,
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    pub rg3cr: RG3CR,
    _reserved13: [u8; 0x30],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub rgcfr: RGCFR,
    _reserved15: [u8; 0x02a4],
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - DMAMUX version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - DMAMUX IP identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - DMAMUX size identification register"]
    pub sidr: SIDR,
}
#[doc = "C0CR (rw) register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c0cr;
#[doc = "C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c1cr;
#[doc = "C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c2cr;
#[doc = "C3CR (rw) register accessor: an alias for `Reg<C3CR_SPEC>`"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c3cr;
#[doc = "C4CR (rw) register accessor: an alias for `Reg<C4CR_SPEC>`"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c4cr;
#[doc = "C5CR (rw) register accessor: an alias for `Reg<C5CR_SPEC>`"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c5cr;
#[doc = "C6CR (rw) register accessor: an alias for `Reg<C6CR_SPEC>`"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c6cr;
#[doc = "RG0CR (rw) register accessor: an alias for `Reg<RG0CR_SPEC>`"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg0cr;
#[doc = "RG1CR (rw) register accessor: an alias for `Reg<RG1CR_SPEC>`"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg1cr;
#[doc = "RG2CR (rw) register accessor: an alias for `Reg<RG2CR_SPEC>`"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg2cr;
#[doc = "RG3CR (rw) register accessor: an alias for `Reg<RG3CR_SPEC>`"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg3cr;
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
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "DMAMUX size identification register"]
pub mod sidr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "DMAMUX IP identification register"]
pub mod ipidr;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "DMAMUX version register"]
pub mod verr;
#[doc = "HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod hwcfgr1;
#[doc = "HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod hwcfgr2;
