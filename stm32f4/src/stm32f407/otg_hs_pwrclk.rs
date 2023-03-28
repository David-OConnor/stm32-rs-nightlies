#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and clock gating control register"]
pub mod pcgcctl;
