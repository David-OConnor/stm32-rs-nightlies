#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - frame control register"]
    pub fcr: FCR,
    #[doc = "0x08 - status register"]
    pub sr: SR,
    #[doc = "0x0c - clear register"]
    pub clr: CLR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - display memory"]
    pub ram_com0: RAM_COM0,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - display memory"]
    pub ram_com1: RAM_COM1,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - display memory"]
    pub ram_com2: RAM_COM2,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - display memory"]
    pub ram_com3: RAM_COM3,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - display memory"]
    pub ram_com4: RAM_COM4,
    _reserved9: [u8; 0x04],
    #[doc = "0x3c - display memory"]
    pub ram_com5: RAM_COM5,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - display memory"]
    pub ram_com6: RAM_COM6,
    _reserved11: [u8; 0x04],
    #[doc = "0x4c - display memory"]
    pub ram_com7: RAM_COM7,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "frame control register"]
pub mod fcr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "clear register"]
pub mod clr;
#[doc = "RAM_COM0 (rw) register accessor: an alias for `Reg<RAM_COM0_SPEC>`"]
pub type RAM_COM0 = crate::Reg<ram_com0::RAM_COM0_SPEC>;
#[doc = "display memory"]
pub mod ram_com0;
#[doc = "RAM_COM1 (rw) register accessor: an alias for `Reg<RAM_COM1_SPEC>`"]
pub type RAM_COM1 = crate::Reg<ram_com1::RAM_COM1_SPEC>;
#[doc = "display memory"]
pub mod ram_com1;
#[doc = "RAM_COM2 (rw) register accessor: an alias for `Reg<RAM_COM2_SPEC>`"]
pub type RAM_COM2 = crate::Reg<ram_com2::RAM_COM2_SPEC>;
#[doc = "display memory"]
pub mod ram_com2;
#[doc = "RAM_COM3 (rw) register accessor: an alias for `Reg<RAM_COM3_SPEC>`"]
pub type RAM_COM3 = crate::Reg<ram_com3::RAM_COM3_SPEC>;
#[doc = "display memory"]
pub mod ram_com3;
#[doc = "RAM_COM4 (rw) register accessor: an alias for `Reg<RAM_COM4_SPEC>`"]
pub type RAM_COM4 = crate::Reg<ram_com4::RAM_COM4_SPEC>;
#[doc = "display memory"]
pub mod ram_com4;
#[doc = "RAM_COM5 (rw) register accessor: an alias for `Reg<RAM_COM5_SPEC>`"]
pub type RAM_COM5 = crate::Reg<ram_com5::RAM_COM5_SPEC>;
#[doc = "display memory"]
pub mod ram_com5;
#[doc = "RAM_COM6 (rw) register accessor: an alias for `Reg<RAM_COM6_SPEC>`"]
pub type RAM_COM6 = crate::Reg<ram_com6::RAM_COM6_SPEC>;
#[doc = "display memory"]
pub mod ram_com6;
#[doc = "RAM_COM7 (rw) register accessor: an alias for `Reg<RAM_COM7_SPEC>`"]
pub type RAM_COM7 = crate::Reg<ram_com7::RAM_COM7_SPEC>;
#[doc = "display memory"]
pub mod ram_com7;
