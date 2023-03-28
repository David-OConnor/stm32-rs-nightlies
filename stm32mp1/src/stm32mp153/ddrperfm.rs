#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write-only register. A read request returns all zeros."]
    pub ddrperfm_ctl: DDRPERFM_CTL,
    #[doc = "0x04 - DDRPERFM configurationl register"]
    pub ddrperfm_cfg: DDRPERFM_CFG,
    #[doc = "0x08 - DDRPERFM status register"]
    pub ddrperfm_status: DDRPERFM_STATUS,
    #[doc = "0x0c - Write-only register. A read request returns all zeros"]
    pub ddrperfm_ccr: DDRPERFM_CCR,
    #[doc = "0x10 - DDRPERFM interrupt enable register"]
    pub ddrperfm_ier: DDRPERFM_IER,
    #[doc = "0x14 - DDRPERFM interrupt status register"]
    pub ddrperfm_isr: DDRPERFM_ISR,
    #[doc = "0x18 - Write-only register. A read request returns all zeros"]
    pub ddrperfm_icr: DDRPERFM_ICR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - DDRPERFM time counter register"]
    pub ddrperfm_tcnt: DDRPERFM_TCNT,
    _reserved8: [u8; 0x3c],
    #[doc = "0x60 - DDRPERFM event counter 0 register"]
    pub ddrperfm_cnt0: DDRPERFM_CNT0,
    _reserved9: [u8; 0x04],
    #[doc = "0x68 - DDRPERFM event counter 1 register"]
    pub ddrperfm_cnt1: DDRPERFM_CNT1,
    _reserved10: [u8; 0x04],
    #[doc = "0x70 - DDRPERFM event counter 2 register"]
    pub ddrperfm_cnt2: DDRPERFM_CNT2,
    _reserved11: [u8; 0x04],
    #[doc = "0x78 - DDRPERFM event counter 3 register"]
    pub ddrperfm_cnt3: DDRPERFM_CNT3,
    _reserved12: [u8; 0x0374],
    #[doc = "0x3f0 - DDRPERFM hardware configuration register"]
    pub ddrperfm_hwcfg: DDRPERFM_HWCFG,
    #[doc = "0x3f4 - DDRPERFM version register"]
    pub ddrperfm_ver: DDRPERFM_VER,
    #[doc = "0x3f8 - DDRPERFM ID register"]
    pub ddrperfm_id: DDRPERFM_ID,
    #[doc = "0x3fc - DDRPERFM magic ID register"]
    pub ddrperfm_sid: DDRPERFM_SID,
}
#[doc = "DDRPERFM_CTL (w) register accessor: an alias for `Reg<DDRPERFM_CTL_SPEC>`"]
pub type DDRPERFM_CTL = crate::Reg<ddrperfm_ctl::DDRPERFM_CTL_SPEC>;
#[doc = "Write-only register. A read request returns all zeros."]
pub mod ddrperfm_ctl;
#[doc = "DDRPERFM_CFG (rw) register accessor: an alias for `Reg<DDRPERFM_CFG_SPEC>`"]
pub type DDRPERFM_CFG = crate::Reg<ddrperfm_cfg::DDRPERFM_CFG_SPEC>;
#[doc = "DDRPERFM configurationl register"]
pub mod ddrperfm_cfg;
#[doc = "DDRPERFM_STATUS (r) register accessor: an alias for `Reg<DDRPERFM_STATUS_SPEC>`"]
pub type DDRPERFM_STATUS = crate::Reg<ddrperfm_status::DDRPERFM_STATUS_SPEC>;
#[doc = "DDRPERFM status register"]
pub mod ddrperfm_status;
#[doc = "DDRPERFM_CCR (w) register accessor: an alias for `Reg<DDRPERFM_CCR_SPEC>`"]
pub type DDRPERFM_CCR = crate::Reg<ddrperfm_ccr::DDRPERFM_CCR_SPEC>;
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_ccr;
#[doc = "DDRPERFM_IER (rw) register accessor: an alias for `Reg<DDRPERFM_IER_SPEC>`"]
pub type DDRPERFM_IER = crate::Reg<ddrperfm_ier::DDRPERFM_IER_SPEC>;
#[doc = "DDRPERFM interrupt enable register"]
pub mod ddrperfm_ier;
#[doc = "DDRPERFM_ISR (r) register accessor: an alias for `Reg<DDRPERFM_ISR_SPEC>`"]
pub type DDRPERFM_ISR = crate::Reg<ddrperfm_isr::DDRPERFM_ISR_SPEC>;
#[doc = "DDRPERFM interrupt status register"]
pub mod ddrperfm_isr;
#[doc = "DDRPERFM_ICR (w) register accessor: an alias for `Reg<DDRPERFM_ICR_SPEC>`"]
pub type DDRPERFM_ICR = crate::Reg<ddrperfm_icr::DDRPERFM_ICR_SPEC>;
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_icr;
#[doc = "DDRPERFM_TCNT (r) register accessor: an alias for `Reg<DDRPERFM_TCNT_SPEC>`"]
pub type DDRPERFM_TCNT = crate::Reg<ddrperfm_tcnt::DDRPERFM_TCNT_SPEC>;
#[doc = "DDRPERFM time counter register"]
pub mod ddrperfm_tcnt;
#[doc = "DDRPERFM_CNT0 (r) register accessor: an alias for `Reg<DDRPERFM_CNT0_SPEC>`"]
pub type DDRPERFM_CNT0 = crate::Reg<ddrperfm_cnt0::DDRPERFM_CNT0_SPEC>;
#[doc = "DDRPERFM event counter 0 register"]
pub mod ddrperfm_cnt0;
#[doc = "DDRPERFM_CNT1 (r) register accessor: an alias for `Reg<DDRPERFM_CNT1_SPEC>`"]
pub type DDRPERFM_CNT1 = crate::Reg<ddrperfm_cnt1::DDRPERFM_CNT1_SPEC>;
#[doc = "DDRPERFM event counter 1 register"]
pub mod ddrperfm_cnt1;
#[doc = "DDRPERFM_CNT2 (r) register accessor: an alias for `Reg<DDRPERFM_CNT2_SPEC>`"]
pub type DDRPERFM_CNT2 = crate::Reg<ddrperfm_cnt2::DDRPERFM_CNT2_SPEC>;
#[doc = "DDRPERFM event counter 2 register"]
pub mod ddrperfm_cnt2;
#[doc = "DDRPERFM_CNT3 (r) register accessor: an alias for `Reg<DDRPERFM_CNT3_SPEC>`"]
pub type DDRPERFM_CNT3 = crate::Reg<ddrperfm_cnt3::DDRPERFM_CNT3_SPEC>;
#[doc = "DDRPERFM event counter 3 register"]
pub mod ddrperfm_cnt3;
#[doc = "DDRPERFM_HWCFG (r) register accessor: an alias for `Reg<DDRPERFM_HWCFG_SPEC>`"]
pub type DDRPERFM_HWCFG = crate::Reg<ddrperfm_hwcfg::DDRPERFM_HWCFG_SPEC>;
#[doc = "DDRPERFM hardware configuration register"]
pub mod ddrperfm_hwcfg;
#[doc = "DDRPERFM_VER (r) register accessor: an alias for `Reg<DDRPERFM_VER_SPEC>`"]
pub type DDRPERFM_VER = crate::Reg<ddrperfm_ver::DDRPERFM_VER_SPEC>;
#[doc = "DDRPERFM version register"]
pub mod ddrperfm_ver;
#[doc = "DDRPERFM_ID (r) register accessor: an alias for `Reg<DDRPERFM_ID_SPEC>`"]
pub type DDRPERFM_ID = crate::Reg<ddrperfm_id::DDRPERFM_ID_SPEC>;
#[doc = "DDRPERFM ID register"]
pub mod ddrperfm_id;
#[doc = "DDRPERFM_SID (r) register accessor: an alias for `Reg<DDRPERFM_SID_SPEC>`"]
pub type DDRPERFM_SID = crate::Reg<ddrperfm_sid::DDRPERFM_SID_SPEC>;
#[doc = "DDRPERFM magic ID register"]
pub mod ddrperfm_sid;
