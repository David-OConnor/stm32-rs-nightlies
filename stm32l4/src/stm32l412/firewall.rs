#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub cssa: CSSA,
    #[doc = "0x04 - Code segment length"]
    pub csl: CSL,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub nvdssa: NVDSSA,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub nvdsl: NVDSL,
    #[doc = "0x10 - Volatile data segment start address"]
    pub vdssa: VDSSA,
    #[doc = "0x14 - Volatile data segment length"]
    pub vdsl: VDSL,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Configuration register"]
    pub cr: CR,
}
#[doc = "CSSA (rw) register accessor: an alias for `Reg<CSSA_SPEC>`"]
pub type CSSA = crate::Reg<cssa::CSSA_SPEC>;
#[doc = "Code segment start address"]
pub mod cssa;
#[doc = "CSL (rw) register accessor: an alias for `Reg<CSL_SPEC>`"]
pub type CSL = crate::Reg<csl::CSL_SPEC>;
#[doc = "Code segment length"]
pub mod csl;
#[doc = "NVDSSA (rw) register accessor: an alias for `Reg<NVDSSA_SPEC>`"]
pub type NVDSSA = crate::Reg<nvdssa::NVDSSA_SPEC>;
#[doc = "Non-volatile data segment start address"]
pub mod nvdssa;
#[doc = "NVDSL (rw) register accessor: an alias for `Reg<NVDSL_SPEC>`"]
pub type NVDSL = crate::Reg<nvdsl::NVDSL_SPEC>;
#[doc = "Non-volatile data segment length"]
pub mod nvdsl;
#[doc = "VDSSA (rw) register accessor: an alias for `Reg<VDSSA_SPEC>`"]
pub type VDSSA = crate::Reg<vdssa::VDSSA_SPEC>;
#[doc = "Volatile data segment start address"]
pub mod vdssa;
#[doc = "VDSL (rw) register accessor: an alias for `Reg<VDSL_SPEC>`"]
pub type VDSL = crate::Reg<vdsl::VDSL_SPEC>;
#[doc = "Volatile data segment length"]
pub mod vdsl;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Configuration register"]
pub mod cr;
