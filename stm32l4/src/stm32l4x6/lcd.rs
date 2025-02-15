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
    #[doc = "0x14..0x54 - display memory"]
    pub ram_com: [RAM_COM; 8],
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
#[doc = "RAM_COM (rw) register accessor: an alias for `Reg<RAM_COM_SPEC>`"]
pub type RAM_COM = crate::Reg<ram_com::RAM_COM_SPEC>;
#[doc = "display memory"]
pub mod ram_com;
