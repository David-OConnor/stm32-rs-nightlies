#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - key registers"]
    pub klr: KLR,
    #[doc = "0x04 - key registers"]
    pub krr: KRR,
}
#[doc = "KLR (w) register accessor: an alias for `Reg<KLR_SPEC>`"]
pub type KLR = crate::Reg<klr::KLR_SPEC>;
#[doc = "key registers"]
pub mod klr;
#[doc = "KRR (w) register accessor: an alias for `Reg<KRR_SPEC>`"]
pub type KRR = crate::Reg<krr::KRR_SPEC>;
#[doc = "key registers"]
pub mod krr;
