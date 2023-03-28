#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel 0 configuration register"]
    pub c0cr: C0CR,
    #[doc = "0x04 - DMAMUX request line multiplexer channel 1 configuration register"]
    pub c1cr: C1CR,
    #[doc = "0x08 - DMAMUX request line multiplexer channel 2 configuration register"]
    pub c2cr: C2CR,
    _reserved3: [u8; 0x74],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: CFR,
    _reserved5: [u8; 0x78],
    #[doc = "0x100 - DMAMUX request generator channel 0 configuration register"]
    pub rg0cr: RG0CR,
    #[doc = "0x104 - DMAMUX request generator channel 1 configuration register"]
    pub rg1cr: RG1CR,
    #[doc = "0x108 - DMAMUX request generator channel 2 configuration register"]
    pub rg2cr: RG2CR,
    #[doc = "0x10c - DMAMUX request generator channel 3 configuration register"]
    pub rg3cr: RG3CR,
    _reserved9: [u8; 0x30],
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    pub rgcfr: RGCFR,
}
#[doc = "C0CR (rw) register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel 0 configuration register"]
pub mod c0cr;
#[doc = "C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel 1 configuration register"]
pub mod c1cr;
#[doc = "C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel 2 configuration register"]
pub mod c2cr;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
#[doc = "RG0CR (rw) register accessor: an alias for `Reg<RG0CR_SPEC>`"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMAMUX request generator channel 0 configuration register"]
pub mod rg0cr;
#[doc = "RG1CR (rw) register accessor: an alias for `Reg<RG1CR_SPEC>`"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMAMUX request generator channel 1 configuration register"]
pub mod rg1cr;
#[doc = "RG2CR (rw) register accessor: an alias for `Reg<RG2CR_SPEC>`"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMAMUX request generator channel 2 configuration register"]
pub mod rg2cr;
#[doc = "RG3CR (rw) register accessor: an alias for `Reg<RG3CR_SPEC>`"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMAMUX request generator channel 3 configuration register"]
pub mod rg3cr;
#[doc = "RGSR (r) register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod rgcfr;
