#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    #[doc = "0x08 - APB Low Freeze Register 1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x0c - APB Low Freeze Register 2"]
    pub apb1fzr2: APB1FZR2,
    #[doc = "0x10 - APB High Freeze Register"]
    pub apb2fzr: APB2FZR,
}
#[doc = "IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1FZR1 (rw) register accessor: an alias for `Reg<APB1FZR1_SPEC>`"]
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1_SPEC>;
#[doc = "APB Low Freeze Register 1"]
pub mod apb1fzr1;
#[doc = "APB1FZR2 (rw) register accessor: an alias for `Reg<APB1FZR2_SPEC>`"]
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2_SPEC>;
#[doc = "APB Low Freeze Register 2"]
pub mod apb1fzr2;
#[doc = "APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "APB High Freeze Register"]
pub mod apb2fzr;
