#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IWDG key register"]
    pub kr: KR,
    #[doc = "0x04 - IWDG prescaler register"]
    pub pr: PR,
    #[doc = "0x08 - IWDG reload register"]
    pub rlr: RLR,
    #[doc = "0x0c - IWDG status register"]
    pub sr: SR,
    #[doc = "0x10 - IWDG window register"]
    pub winr: WINR,
    #[doc = "0x14 - IWDG early wakeup interrupt register"]
    pub ewcr: EWCR,
}
#[doc = "KR (w) register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "IWDG key register"]
pub mod kr;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "IWDG prescaler register"]
pub mod pr;
#[doc = "RLR (rw) register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "IWDG reload register"]
pub mod rlr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "IWDG status register"]
pub mod sr;
#[doc = "WINR (rw) register accessor: an alias for `Reg<WINR_SPEC>`"]
pub type WINR = crate::Reg<winr::WINR_SPEC>;
#[doc = "IWDG window register"]
pub mod winr;
#[doc = "EWCR (rw) register accessor: an alias for `Reg<EWCR_SPEC>`"]
pub type EWCR = crate::Reg<ewcr::EWCR_SPEC>;
#[doc = "IWDG early wakeup interrupt register"]
pub mod ewcr;
