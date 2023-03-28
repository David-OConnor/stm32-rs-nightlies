#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register"]
    pub cr: CR,
    #[doc = "0x04 - OctoSPI IO Manager Port 1 Configuration Register"]
    pub p1cr: P1CR,
    #[doc = "0x08 - OctoSPI IO Manager Port 2 Configuration Register"]
    pub p2cr: P2CR,
}
#[doc = "P1CR (rw) register accessor: an alias for `Reg<P1CR_SPEC>`"]
pub type P1CR = crate::Reg<p1cr::P1CR_SPEC>;
#[doc = "OctoSPI IO Manager Port 1 Configuration Register"]
pub mod p1cr;
#[doc = "P2CR (rw) register accessor: an alias for `Reg<P2CR_SPEC>`"]
pub type P2CR = crate::Reg<p2cr::P2CR_SPEC>;
#[doc = "OctoSPI IO Manager Port 2 Configuration Register"]
pub mod p2cr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "configuration register"]
pub mod cr;
