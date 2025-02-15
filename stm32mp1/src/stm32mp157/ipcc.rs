#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPCC Processor 1 control register"]
    pub ipcc_c1cr: IPCC_C1CR,
    #[doc = "0x04 - IPCC Processor 1 mask register"]
    pub ipcc_c1mr: IPCC_C1MR,
    #[doc = "0x08 - Reading this register will always return 0x0000 0000."]
    pub ipcc_c1scr: IPCC_C1SCR,
    #[doc = "0x0c - IPCC processor 1 to processor 2 status register"]
    pub ipcc_c1toc2sr: IPCC_C1TOC2SR,
    #[doc = "0x10 - IPCC Processor 2 control register"]
    pub ipcc_c2cr: IPCC_C2CR,
    #[doc = "0x14 - IPCC Processor 2 mask register"]
    pub ipcc_c2mr: IPCC_C2MR,
    #[doc = "0x18 - Reading this register will always return 0x0000 0000."]
    pub ipcc_c2scr: IPCC_C2SCR,
    #[doc = "0x1c - IPCC processor 2 to processor 1 status register"]
    pub ipcc_c2toc1sr: IPCC_C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    pub ipcc_hwcfgr: IPCC_HWCFGR,
    #[doc = "0x3f4 - IPCC IP Version register"]
    pub ipcc_ver: IPCC_VER,
    #[doc = "0x3f8 - IPCC IP Identification register"]
    pub ipcc_id: IPCC_ID,
    #[doc = "0x3fc - IPCC Size ID register"]
    pub ipcc_sid: IPCC_SID,
}
#[doc = "IPCC_C1CR (rw) register accessor: an alias for `Reg<IPCC_C1CR_SPEC>`"]
pub type IPCC_C1CR = crate::Reg<ipcc_c1cr::IPCC_C1CR_SPEC>;
#[doc = "IPCC Processor 1 control register"]
pub mod ipcc_c1cr;
#[doc = "IPCC_C1MR (rw) register accessor: an alias for `Reg<IPCC_C1MR_SPEC>`"]
pub type IPCC_C1MR = crate::Reg<ipcc_c1mr::IPCC_C1MR_SPEC>;
#[doc = "IPCC Processor 1 mask register"]
pub mod ipcc_c1mr;
#[doc = "IPCC_C1SCR (rw) register accessor: an alias for `Reg<IPCC_C1SCR_SPEC>`"]
pub type IPCC_C1SCR = crate::Reg<ipcc_c1scr::IPCC_C1SCR_SPEC>;
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod ipcc_c1scr;
#[doc = "IPCC_C1TOC2SR (r) register accessor: an alias for `Reg<IPCC_C1TOC2SR_SPEC>`"]
pub type IPCC_C1TOC2SR = crate::Reg<ipcc_c1toc2sr::IPCC_C1TOC2SR_SPEC>;
#[doc = "IPCC processor 1 to processor 2 status register"]
pub mod ipcc_c1toc2sr;
#[doc = "IPCC_C2CR (rw) register accessor: an alias for `Reg<IPCC_C2CR_SPEC>`"]
pub type IPCC_C2CR = crate::Reg<ipcc_c2cr::IPCC_C2CR_SPEC>;
#[doc = "IPCC Processor 2 control register"]
pub mod ipcc_c2cr;
#[doc = "IPCC_C2MR (rw) register accessor: an alias for `Reg<IPCC_C2MR_SPEC>`"]
pub type IPCC_C2MR = crate::Reg<ipcc_c2mr::IPCC_C2MR_SPEC>;
#[doc = "IPCC Processor 2 mask register"]
pub mod ipcc_c2mr;
#[doc = "IPCC_C2SCR (rw) register accessor: an alias for `Reg<IPCC_C2SCR_SPEC>`"]
pub type IPCC_C2SCR = crate::Reg<ipcc_c2scr::IPCC_C2SCR_SPEC>;
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod ipcc_c2scr;
#[doc = "IPCC_C2TOC1SR (r) register accessor: an alias for `Reg<IPCC_C2TOC1SR_SPEC>`"]
pub type IPCC_C2TOC1SR = crate::Reg<ipcc_c2toc1sr::IPCC_C2TOC1SR_SPEC>;
#[doc = "IPCC processor 2 to processor 1 status register"]
pub mod ipcc_c2toc1sr;
#[doc = "IPCC_HWCFGR (r) register accessor: an alias for `Reg<IPCC_HWCFGR_SPEC>`"]
pub type IPCC_HWCFGR = crate::Reg<ipcc_hwcfgr::IPCC_HWCFGR_SPEC>;
#[doc = "IPCC Hardware configuration register"]
pub mod ipcc_hwcfgr;
#[doc = "IPCC_VER (r) register accessor: an alias for `Reg<IPCC_VER_SPEC>`"]
pub type IPCC_VER = crate::Reg<ipcc_ver::IPCC_VER_SPEC>;
#[doc = "IPCC IP Version register"]
pub mod ipcc_ver;
#[doc = "IPCC_ID (r) register accessor: an alias for `Reg<IPCC_ID_SPEC>`"]
pub type IPCC_ID = crate::Reg<ipcc_id::IPCC_ID_SPEC>;
#[doc = "IPCC IP Identification register"]
pub mod ipcc_id;
#[doc = "IPCC_SID (r) register accessor: an alias for `Reg<IPCC_SID_SPEC>`"]
pub type IPCC_SID = crate::Reg<ipcc_sid::IPCC_SID_SPEC>;
#[doc = "IPCC Size ID register"]
pub mod ipcc_sid;
