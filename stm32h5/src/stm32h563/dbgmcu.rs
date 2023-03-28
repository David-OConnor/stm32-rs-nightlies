#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU identity code register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - DBGMCU configuration register"]
    pub cr: CR,
    #[doc = "0x08 - DBGMCU APB1L peripheral freeze register"]
    pub apb1lfzr: APB1LFZR,
    #[doc = "0x0c - DBGMCU APB1H peripheral freeze register"]
    pub apb1hfzr: APB1HFZR,
    #[doc = "0x10 - DBGMCU APB2 peripheral freeze register"]
    pub apb2fzr: APB2FZR,
    #[doc = "0x14 - DBGMCU APB3 peripheral freeze register"]
    pub apb3fzr: APB3FZR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - DBGMCU AHB1 peripheral freeze register"]
    pub ahb1fzr: AHB1FZR,
    _reserved7: [u8; 0xd8],
    #[doc = "0xfc - DBGMCU status register"]
    pub sr: SR,
    #[doc = "0x100 - DBGMCU debug authentication mailbox host register"]
    pub dbg_auth_host: DBG_AUTH_HOST,
    #[doc = "0x104 - DBGMCU debug authentication mailbox device register"]
    pub dbg_auth_device: DBG_AUTH_DEVICE,
    #[doc = "0x108 - DBGMCU debug authentication mailbox acknowledge register"]
    pub dbg_auth_ack: DBG_AUTH_ACK,
    _reserved11: [u8; 0x0ec4],
    #[doc = "0xfd0 - DBGMCU CoreSight peripheral identity register 4"]
    pub pidr4: PIDR4,
    _reserved12: [u8; 0x0c],
    #[doc = "0xfe0 - DBGMCU CoreSight peripheral identity register 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - DBGMCU CoreSight peripheral identity register 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - DBGMCU CoreSight peripheral identity register 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - DBGMCU CoreSight peripheral identity register 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - DBGMCU CoreSight component identity register 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - DBGMCU CoreSight component identity register 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - DBGMCU CoreSight component identity register 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - DBGMCU CoreSight component identity register 3"]
    pub cidr3: CIDR3,
}
#[doc = "IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DBGMCU identity code register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DBGMCU configuration register"]
pub mod cr;
#[doc = "APB1LFZR (rw) register accessor: an alias for `Reg<APB1LFZR_SPEC>`"]
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZR_SPEC>;
#[doc = "DBGMCU APB1L peripheral freeze register"]
pub mod apb1lfzr;
#[doc = "APB1HFZR (rw) register accessor: an alias for `Reg<APB1HFZR_SPEC>`"]
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZR_SPEC>;
#[doc = "DBGMCU APB1H peripheral freeze register"]
pub mod apb1hfzr;
#[doc = "APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "DBGMCU APB2 peripheral freeze register"]
pub mod apb2fzr;
#[doc = "APB3FZR (rw) register accessor: an alias for `Reg<APB3FZR_SPEC>`"]
pub type APB3FZR = crate::Reg<apb3fzr::APB3FZR_SPEC>;
#[doc = "DBGMCU APB3 peripheral freeze register"]
pub mod apb3fzr;
#[doc = "AHB1FZR (rw) register accessor: an alias for `Reg<AHB1FZR_SPEC>`"]
pub type AHB1FZR = crate::Reg<ahb1fzr::AHB1FZR_SPEC>;
#[doc = "DBGMCU AHB1 peripheral freeze register"]
pub mod ahb1fzr;
#[doc = "SR (w) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DBGMCU status register"]
pub mod sr;
#[doc = "DBG_AUTH_HOST (rw) register accessor: an alias for `Reg<DBG_AUTH_HOST_SPEC>`"]
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOST_SPEC>;
#[doc = "DBGMCU debug authentication mailbox host register"]
pub mod dbg_auth_host;
#[doc = "DBG_AUTH_DEVICE (r) register accessor: an alias for `Reg<DBG_AUTH_DEVICE_SPEC>`"]
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICE_SPEC>;
#[doc = "DBGMCU debug authentication mailbox device register"]
pub mod dbg_auth_device;
#[doc = "DBG_AUTH_ACK (rw) register accessor: an alias for `Reg<DBG_AUTH_ACK_SPEC>`"]
pub type DBG_AUTH_ACK = crate::Reg<dbg_auth_ack::DBG_AUTH_ACK_SPEC>;
#[doc = "DBGMCU debug authentication mailbox acknowledge register"]
pub mod dbg_auth_ack;
#[doc = "PIDR4 (r) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 4"]
pub mod pidr4;
#[doc = "PIDR0 (r) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 3"]
pub mod cidr3;
