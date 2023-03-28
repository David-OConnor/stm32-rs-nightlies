#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC data register"]
    pub dr: DR,
    #[doc = "0x04 - CRC independent data register"]
    pub idr: IDR,
    #[doc = "0x08 - CRC control register"]
    pub cr: CR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CRC initial value"]
    pub init: INIT,
    #[doc = "0x14 - CRC polynomial"]
    pub pol: POL,
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "CRC data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "CRC independent data register"]
pub mod idr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRC control register"]
pub mod cr;
#[doc = "INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "CRC initial value"]
pub mod init;
#[doc = "POL (rw) register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "CRC polynomial"]
pub mod pol;
