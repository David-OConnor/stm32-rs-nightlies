#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub rng_cr: RNG_CR,
    #[doc = "0x04 - RNG status register"]
    pub rng_sr: RNG_SR,
    #[doc = "0x08 - RNG data register"]
    pub rng_dr: RNG_DR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - RNG health test control register"]
    pub rng_htcr: RNG_HTCR,
}
#[doc = "RNG_CR (rw) register accessor: an alias for `Reg<RNG_CR_SPEC>`"]
pub type RNG_CR = crate::Reg<rng_cr::RNG_CR_SPEC>;
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG_SR (rw) register accessor: an alias for `Reg<RNG_SR_SPEC>`"]
pub type RNG_SR = crate::Reg<rng_sr::RNG_SR_SPEC>;
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "RNG_DR (r) register accessor: an alias for `Reg<RNG_DR_SPEC>`"]
pub type RNG_DR = crate::Reg<rng_dr::RNG_DR_SPEC>;
#[doc = "RNG data register"]
pub mod rng_dr;
#[doc = "RNG_HTCR (rw) register accessor: an alias for `Reg<RNG_HTCR_SPEC>`"]
pub type RNG_HTCR = crate::Reg<rng_htcr::RNG_HTCR_SPEC>;
#[doc = "RNG health test control register"]
pub mod rng_htcr;
