#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Remap Memory register"]
    pub memrmp: MEMRMP,
    #[doc = "0x04 - peripheral mode configuration register"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - CCM SRAM control and status register"]
    pub scsr: SCSR,
    #[doc = "0x1c - configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x20 - SRAM Write protection register 1"]
    pub swpr: SWPR,
    #[doc = "0x24 - SRAM2 Key Register"]
    pub skr: SKR,
}
#[doc = "MEMRMP (rw) register accessor: an alias for `Reg<MEMRMP_SPEC>`"]
pub type MEMRMP = crate::Reg<memrmp::MEMRMP_SPEC>;
#[doc = "Remap Memory register"]
pub mod memrmp;
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "peripheral mode configuration register"]
pub mod cfgr1;
#[doc = "EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR (rw) register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "CCM SRAM control and status register"]
pub mod scsr;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "SWPR (rw) register accessor: an alias for `Reg<SWPR_SPEC>`"]
pub type SWPR = crate::Reg<swpr::SWPR_SPEC>;
#[doc = "SRAM Write protection register 1"]
pub mod swpr;
#[doc = "SKR (w) register accessor: an alias for `Reg<SKR_SPEC>`"]
pub type SKR = crate::Reg<skr::SKR_SPEC>;
#[doc = "SRAM2 Key Register"]
pub mod skr;
