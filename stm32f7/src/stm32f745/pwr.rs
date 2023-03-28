#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub cr1: CR1,
    #[doc = "0x04 - power control/status register"]
    pub csr1: CSR1,
    #[doc = "0x08 - power control register"]
    pub cr2: CR2,
    #[doc = "0x0c - power control/status register"]
    pub csr2: CSR2,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "power control register"]
pub mod cr1;
#[doc = "CSR1 (rw) register accessor: an alias for `Reg<CSR1_SPEC>`"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "power control/status register"]
pub mod csr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "power control register"]
pub mod cr2;
#[doc = "CSR2 (rw) register accessor: an alias for `Reg<CSR2_SPEC>`"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "power control/status register"]
pub mod csr2;
