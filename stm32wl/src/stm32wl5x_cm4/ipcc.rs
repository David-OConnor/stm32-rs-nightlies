#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPCC Processor 1 control register"]
    pub c1cr: C1CR,
    #[doc = "0x04 - IPCC Processor 1 mask register"]
    pub c1mr: C1MR,
    #[doc = "0x08 - Reading this register will always return 0x0000 0000."]
    pub c1scr: C1SCR,
    #[doc = "0x0c - IPCC processor 1 to processor 2 status register"]
    pub ic1toc2sr: IC1TOC2SR,
    #[doc = "0x10 - IPCC Processor 2 control register"]
    pub c2cr: C2CR,
    #[doc = "0x14 - IPCC Processor 2 mask register"]
    pub c2mr: C2MR,
    #[doc = "0x18 - Reading this register will always return 0x0000 0000."]
    pub c2scr: C2SCR,
    #[doc = "0x1c - IPCC processor 2 to processor 1 status register"]
    pub c2toc1sr: C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - IPCC IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - IPCC IP Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - IPCC Size ID register"]
    pub sidr: SIDR,
}
#[doc = "C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "IPCC Processor 1 control register"]
pub mod c1cr;
#[doc = "C1MR (rw) register accessor: an alias for `Reg<C1MR_SPEC>`"]
pub type C1MR = crate::Reg<c1mr::C1MR_SPEC>;
#[doc = "IPCC Processor 1 mask register"]
pub mod c1mr;
#[doc = "C1SCR (rw) register accessor: an alias for `Reg<C1SCR_SPEC>`"]
pub type C1SCR = crate::Reg<c1scr::C1SCR_SPEC>;
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod c1scr;
#[doc = "IC1TOC2SR (r) register accessor: an alias for `Reg<IC1TOC2SR_SPEC>`"]
pub type IC1TOC2SR = crate::Reg<ic1toc2sr::IC1TOC2SR_SPEC>;
#[doc = "IPCC processor 1 to processor 2 status register"]
pub mod ic1toc2sr;
#[doc = "C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "IPCC Processor 2 control register"]
pub mod c2cr;
#[doc = "C2MR (rw) register accessor: an alias for `Reg<C2MR_SPEC>`"]
pub type C2MR = crate::Reg<c2mr::C2MR_SPEC>;
#[doc = "IPCC Processor 2 mask register"]
pub mod c2mr;
#[doc = "C2SCR (rw) register accessor: an alias for `Reg<C2SCR_SPEC>`"]
pub type C2SCR = crate::Reg<c2scr::C2SCR_SPEC>;
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod c2scr;
#[doc = "C2TOC1SR (r) register accessor: an alias for `Reg<C2TOC1SR_SPEC>`"]
pub type C2TOC1SR = crate::Reg<c2toc1sr::C2TOC1SR_SPEC>;
#[doc = "IPCC processor 2 to processor 1 status register"]
pub mod c2toc1sr;
#[doc = "HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "IPCC Hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "IPCC IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "IPCC IP Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "IPCC Size ID register"]
pub mod sidr;
