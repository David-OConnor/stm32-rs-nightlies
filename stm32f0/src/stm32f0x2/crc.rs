#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 0x04],
    #[doc = "0x04 - Independent data register"]
    pub idr: IDR,
    #[doc = "0x08 - Control register"]
    pub cr: CR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Initial CRC value"]
    pub init: INIT,
    #[doc = "0x14 - CRC polynomial"]
    pub pol: POL,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register - half-word sized"]
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Data register - byte sized"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Independent data register"]
pub mod idr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "Initial CRC value"]
pub mod init;
#[doc = "POL (rw) register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "CRC polynomial"]
pub mod pol;
#[doc = "DR8 (rw) register accessor: an alias for `Reg<DR8_SPEC>`"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "Data register - byte sized"]
pub mod dr8;
#[doc = "DR16 (rw) register accessor: an alias for `Reg<DR16_SPEC>`"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "Data register - half-word sized"]
pub mod dr16;
