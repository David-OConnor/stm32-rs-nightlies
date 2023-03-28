#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0x04 - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0x08 - Cache Size ID register"]
    pub ccsidr: CCSIDR,
}
#[doc = "CLIDR (r) register accessor: an alias for `Reg<CLIDR_SPEC>`"]
pub type CLIDR = crate::Reg<clidr::CLIDR_SPEC>;
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "CTR (r) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "CCSIDR (r) register accessor: an alias for `Reg<CCSIDR_SPEC>`"]
pub type CCSIDR = crate::Reg<ccsidr::CCSIDR_SPEC>;
#[doc = "Cache Size ID register"]
pub mod ccsidr;
