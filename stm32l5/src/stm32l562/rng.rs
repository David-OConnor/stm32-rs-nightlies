#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub cr: CR,
    #[doc = "0x04 - RNG status register"]
    pub sr: SR,
    #[doc = "0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
    pub dr: DR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
    pub htcr: HTCR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RNG control register"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RNG status register"]
pub mod sr;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
pub mod dr;
#[doc = "HTCR (rw) register accessor: an alias for `Reg<HTCR_SPEC>`"]
pub type HTCR = crate::Reg<htcr::HTCR_SPEC>;
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
pub mod htcr;
