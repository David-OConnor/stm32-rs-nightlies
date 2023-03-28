#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETZPC ROM secure size definition"]
    pub etzpc_tzma0_size: ETZPC_TZMA0_SIZE,
    #[doc = "0x04 - ETZPC RAM secure size definition"]
    pub etzpc_tzma1_size: ETZPC_TZMA1_SIZE,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Register reset values"]
    pub etzpc_decprot0: ETZPC_DECPROT0,
    #[doc = "0x14 - Register reset values"]
    pub etzpc_decprot1: ETZPC_DECPROT1,
    #[doc = "0x18 - Register reset values"]
    pub etzpc_decprot2: ETZPC_DECPROT2,
    #[doc = "0x1c - Register reset values"]
    pub etzpc_decprot3: ETZPC_DECPROT3,
    #[doc = "0x20 - Register reset values"]
    pub etzpc_decprot4: ETZPC_DECPROT4,
    #[doc = "0x24 - Register reset values"]
    pub etzpc_decprot5: ETZPC_DECPROT5,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - ETZPC decprot lock 0 register"]
    pub etzpc_decprot_lock0: ETZPC_DECPROT_LOCK0,
    #[doc = "0x34 - ETZPC decprot lock 1 register"]
    pub etzpc_decprot_lock1: ETZPC_DECPROT_LOCK1,
    #[doc = "0x38 - ETZPC decprot lock 2 register"]
    pub etzpc_decprot_lock2: ETZPC_DECPROT_LOCK2,
    _reserved11: [u8; 0x03b4],
    #[doc = "0x3f0 - ETZPC IP HW configuration register"]
    pub etzpc_hwcfgr: ETZPC_HWCFGR,
    #[doc = "0x3f4 - ETZPC IP version register"]
    pub etzpc_verr: ETZPC_VERR,
    #[doc = "0x3f8 - ETZPC IP version register"]
    pub etzpc_idr: ETZPC_IDR,
    #[doc = "0x3fc - ETZPC IP version register"]
    pub etzpc_sidr: ETZPC_SIDR,
}
#[doc = "ETZPC_TZMA0_SIZE (rw) register accessor: an alias for `Reg<ETZPC_TZMA0_SIZE_SPEC>`"]
pub type ETZPC_TZMA0_SIZE = crate::Reg<etzpc_tzma0_size::ETZPC_TZMA0_SIZE_SPEC>;
#[doc = "ETZPC ROM secure size definition"]
pub mod etzpc_tzma0_size;
#[doc = "ETZPC_TZMA1_SIZE (rw) register accessor: an alias for `Reg<ETZPC_TZMA1_SIZE_SPEC>`"]
pub type ETZPC_TZMA1_SIZE = crate::Reg<etzpc_tzma1_size::ETZPC_TZMA1_SIZE_SPEC>;
#[doc = "ETZPC RAM secure size definition"]
pub mod etzpc_tzma1_size;
#[doc = "ETZPC_DECPROT0 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT0_SPEC>`"]
pub type ETZPC_DECPROT0 = crate::Reg<etzpc_decprot0::ETZPC_DECPROT0_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot0;
#[doc = "ETZPC_DECPROT1 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT1_SPEC>`"]
pub type ETZPC_DECPROT1 = crate::Reg<etzpc_decprot1::ETZPC_DECPROT1_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot1;
#[doc = "ETZPC_DECPROT2 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT2_SPEC>`"]
pub type ETZPC_DECPROT2 = crate::Reg<etzpc_decprot2::ETZPC_DECPROT2_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot2;
#[doc = "ETZPC_DECPROT3 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT3_SPEC>`"]
pub type ETZPC_DECPROT3 = crate::Reg<etzpc_decprot3::ETZPC_DECPROT3_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot3;
#[doc = "ETZPC_DECPROT4 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT4_SPEC>`"]
pub type ETZPC_DECPROT4 = crate::Reg<etzpc_decprot4::ETZPC_DECPROT4_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot4;
#[doc = "ETZPC_DECPROT5 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT5_SPEC>`"]
pub type ETZPC_DECPROT5 = crate::Reg<etzpc_decprot5::ETZPC_DECPROT5_SPEC>;
#[doc = "Register reset values"]
pub mod etzpc_decprot5;
#[doc = "ETZPC_DECPROT_LOCK0 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT_LOCK0_SPEC>`"]
pub type ETZPC_DECPROT_LOCK0 = crate::Reg<etzpc_decprot_lock0::ETZPC_DECPROT_LOCK0_SPEC>;
#[doc = "ETZPC decprot lock 0 register"]
pub mod etzpc_decprot_lock0;
#[doc = "ETZPC_DECPROT_LOCK1 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT_LOCK1_SPEC>`"]
pub type ETZPC_DECPROT_LOCK1 = crate::Reg<etzpc_decprot_lock1::ETZPC_DECPROT_LOCK1_SPEC>;
#[doc = "ETZPC decprot lock 1 register"]
pub mod etzpc_decprot_lock1;
#[doc = "ETZPC_DECPROT_LOCK2 (rw) register accessor: an alias for `Reg<ETZPC_DECPROT_LOCK2_SPEC>`"]
pub type ETZPC_DECPROT_LOCK2 = crate::Reg<etzpc_decprot_lock2::ETZPC_DECPROT_LOCK2_SPEC>;
#[doc = "ETZPC decprot lock 2 register"]
pub mod etzpc_decprot_lock2;
#[doc = "ETZPC_HWCFGR (r) register accessor: an alias for `Reg<ETZPC_HWCFGR_SPEC>`"]
pub type ETZPC_HWCFGR = crate::Reg<etzpc_hwcfgr::ETZPC_HWCFGR_SPEC>;
#[doc = "ETZPC IP HW configuration register"]
pub mod etzpc_hwcfgr;
#[doc = "ETZPC_VERR (r) register accessor: an alias for `Reg<ETZPC_VERR_SPEC>`"]
pub type ETZPC_VERR = crate::Reg<etzpc_verr::ETZPC_VERR_SPEC>;
#[doc = "ETZPC IP version register"]
pub mod etzpc_verr;
#[doc = "ETZPC_IDR (r) register accessor: an alias for `Reg<ETZPC_IDR_SPEC>`"]
pub type ETZPC_IDR = crate::Reg<etzpc_idr::ETZPC_IDR_SPEC>;
#[doc = "ETZPC IP version register"]
pub mod etzpc_idr;
#[doc = "ETZPC_SIDR (r) register accessor: an alias for `Reg<ETZPC_SIDR_SPEC>`"]
pub type ETZPC_SIDR = crate::Reg<etzpc_sidr::ETZPC_SIDR_SPEC>;
#[doc = "ETZPC IP version register"]
pub mod etzpc_sidr;
