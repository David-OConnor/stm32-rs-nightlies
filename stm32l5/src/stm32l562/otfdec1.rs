#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTFDEC control register"]
    pub cr: CR,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - OTFDEC region x configuration register"]
    pub r1cfgr: R1CFGR,
    #[doc = "0x24 - OTFDEC region x start address register"]
    pub r1startaddr: R1STARTADDR,
    #[doc = "0x28 - OTFDEC region x end address register"]
    pub r1endaddr: R1ENDADDR,
    #[doc = "0x2c - OTFDEC region x nonce register 0"]
    pub r1noncer0: R1NONCER0,
    #[doc = "0x30 - OTFDEC region x nonce register 1"]
    pub r1noncer1: R1NONCER1,
    #[doc = "0x34 - OTFDEC region x key register 0"]
    pub r1keyr0: R1KEYR0,
    #[doc = "0x38 - OTFDEC region x key register 1"]
    pub r1keyr1: R1KEYR1,
    #[doc = "0x3c - OTFDEC region x key register 2"]
    pub r1keyr2: R1KEYR2,
    #[doc = "0x40 - OTFDEC region x key register 3"]
    pub r1keyr3: R1KEYR3,
    _reserved10: [u8; 0x0c],
    #[doc = "0x50 - OTFDEC region x configuration register"]
    pub r2cfgr: R2CFGR,
    #[doc = "0x54 - OTFDEC region x start address register"]
    pub r2startaddr: R2STARTADDR,
    #[doc = "0x58 - OTFDEC region x end address register"]
    pub r2endaddr: R2ENDADDR,
    #[doc = "0x5c - OTFDEC region x nonce register 0"]
    pub r2noncer0: R2NONCER0,
    #[doc = "0x60 - OTFDEC region x nonce register 1"]
    pub r2noncer1: R2NONCER1,
    #[doc = "0x64 - OTFDEC region x key register 0"]
    pub r2keyr0: R2KEYR0,
    #[doc = "0x68 - OTFDEC region x key register 1"]
    pub r2keyr1: R2KEYR1,
    #[doc = "0x6c - OTFDEC region x key register 2"]
    pub r2keyr2: R2KEYR2,
    #[doc = "0x70 - OTFDEC region x key register 3"]
    pub r2keyr3: R2KEYR3,
    _reserved19: [u8; 0x0c],
    #[doc = "0x80 - OTFDEC region x configuration register"]
    pub r3cfgr: R3CFGR,
    #[doc = "0x84 - OTFDEC region x start address register"]
    pub r3startaddr: R3STARTADDR,
    #[doc = "0x88 - OTFDEC region x end address register"]
    pub r3endaddr: R3ENDADDR,
    #[doc = "0x8c - OTFDEC region x nonce register 0"]
    pub r3noncer0: R3NONCER0,
    #[doc = "0x90 - OTFDEC region x nonce register 1"]
    pub r3noncer1: R3NONCER1,
    #[doc = "0x94 - OTFDEC region x key register 0"]
    pub r3keyr0: R3KEYR0,
    #[doc = "0x98 - OTFDEC region x key register 1"]
    pub r3keyr1: R3KEYR1,
    #[doc = "0x9c - OTFDEC region x key register 2"]
    pub r3keyr2: R3KEYR2,
    #[doc = "0xa0 - OTFDEC region x key register 3"]
    pub r3keyr3: R3KEYR3,
    _reserved28: [u8; 0x0c],
    #[doc = "0xb0 - OTFDEC region x configuration register"]
    pub r4cfgr: R4CFGR,
    #[doc = "0xb4 - OTFDEC region x start address register"]
    pub r4startaddr: R4STARTADDR,
    #[doc = "0xb8 - OTFDEC region x end address register"]
    pub r4endaddr: R4ENDADDR,
    #[doc = "0xbc - OTFDEC region x nonce register 0"]
    pub r4noncer0: R4NONCER0,
    #[doc = "0xc0 - OTFDEC region x nonce register 1"]
    pub r4noncer1: R4NONCER1,
    #[doc = "0xc4 - OTFDEC region x key register 0"]
    pub r4keyr0: R4KEYR0,
    #[doc = "0xc8 - OTFDEC region x key register 1"]
    pub r4keyr1: R4KEYR1,
    #[doc = "0xcc - OTFDEC region x key register 2"]
    pub r4keyr2: R4KEYR2,
    #[doc = "0xd0 - OTFDEC region x key register 3"]
    pub r4keyr3: R4KEYR3,
    _reserved37: [u8; 0x022c],
    #[doc = "0x300 - OTFDEC interrupt status register"]
    pub isr: ISR,
    #[doc = "0x304 - OTFDEC interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x308 - OTFDEC interrupt enable register"]
    pub ier: IER,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "OTFDEC control register"]
pub mod cr;
#[doc = "R1CFGR (rw) register accessor: an alias for `Reg<R1CFGR_SPEC>`"]
pub type R1CFGR = crate::Reg<r1cfgr::R1CFGR_SPEC>;
#[doc = "OTFDEC region x configuration register"]
pub mod r1cfgr;
#[doc = "R2CFGR (rw) register accessor: an alias for `Reg<R2CFGR_SPEC>`"]
pub type R2CFGR = crate::Reg<r2cfgr::R2CFGR_SPEC>;
#[doc = "OTFDEC region x configuration register"]
pub mod r2cfgr;
#[doc = "R3CFGR (rw) register accessor: an alias for `Reg<R3CFGR_SPEC>`"]
pub type R3CFGR = crate::Reg<r3cfgr::R3CFGR_SPEC>;
#[doc = "OTFDEC region x configuration register"]
pub mod r3cfgr;
#[doc = "R4CFGR (rw) register accessor: an alias for `Reg<R4CFGR_SPEC>`"]
pub type R4CFGR = crate::Reg<r4cfgr::R4CFGR_SPEC>;
#[doc = "OTFDEC region x configuration register"]
pub mod r4cfgr;
#[doc = "R1STARTADDR (rw) register accessor: an alias for `Reg<R1STARTADDR_SPEC>`"]
pub type R1STARTADDR = crate::Reg<r1startaddr::R1STARTADDR_SPEC>;
#[doc = "OTFDEC region x start address register"]
pub mod r1startaddr;
#[doc = "R2STARTADDR (rw) register accessor: an alias for `Reg<R2STARTADDR_SPEC>`"]
pub type R2STARTADDR = crate::Reg<r2startaddr::R2STARTADDR_SPEC>;
#[doc = "OTFDEC region x start address register"]
pub mod r2startaddr;
#[doc = "R3STARTADDR (rw) register accessor: an alias for `Reg<R3STARTADDR_SPEC>`"]
pub type R3STARTADDR = crate::Reg<r3startaddr::R3STARTADDR_SPEC>;
#[doc = "OTFDEC region x start address register"]
pub mod r3startaddr;
#[doc = "R4STARTADDR (rw) register accessor: an alias for `Reg<R4STARTADDR_SPEC>`"]
pub type R4STARTADDR = crate::Reg<r4startaddr::R4STARTADDR_SPEC>;
#[doc = "OTFDEC region x start address register"]
pub mod r4startaddr;
#[doc = "R1ENDADDR (rw) register accessor: an alias for `Reg<R1ENDADDR_SPEC>`"]
pub type R1ENDADDR = crate::Reg<r1endaddr::R1ENDADDR_SPEC>;
#[doc = "OTFDEC region x end address register"]
pub mod r1endaddr;
#[doc = "R2ENDADDR (rw) register accessor: an alias for `Reg<R2ENDADDR_SPEC>`"]
pub type R2ENDADDR = crate::Reg<r2endaddr::R2ENDADDR_SPEC>;
#[doc = "OTFDEC region x end address register"]
pub mod r2endaddr;
#[doc = "R3ENDADDR (rw) register accessor: an alias for `Reg<R3ENDADDR_SPEC>`"]
pub type R3ENDADDR = crate::Reg<r3endaddr::R3ENDADDR_SPEC>;
#[doc = "OTFDEC region x end address register"]
pub mod r3endaddr;
#[doc = "R4ENDADDR (rw) register accessor: an alias for `Reg<R4ENDADDR_SPEC>`"]
pub type R4ENDADDR = crate::Reg<r4endaddr::R4ENDADDR_SPEC>;
#[doc = "OTFDEC region x end address register"]
pub mod r4endaddr;
#[doc = "R1NONCER0 (rw) register accessor: an alias for `Reg<R1NONCER0_SPEC>`"]
pub type R1NONCER0 = crate::Reg<r1noncer0::R1NONCER0_SPEC>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r1noncer0;
#[doc = "R2NONCER0 (rw) register accessor: an alias for `Reg<R2NONCER0_SPEC>`"]
pub type R2NONCER0 = crate::Reg<r2noncer0::R2NONCER0_SPEC>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r2noncer0;
#[doc = "R3NONCER0 (rw) register accessor: an alias for `Reg<R3NONCER0_SPEC>`"]
pub type R3NONCER0 = crate::Reg<r3noncer0::R3NONCER0_SPEC>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r3noncer0;
#[doc = "R4NONCER0 (rw) register accessor: an alias for `Reg<R4NONCER0_SPEC>`"]
pub type R4NONCER0 = crate::Reg<r4noncer0::R4NONCER0_SPEC>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r4noncer0;
#[doc = "R1NONCER1 (rw) register accessor: an alias for `Reg<R1NONCER1_SPEC>`"]
pub type R1NONCER1 = crate::Reg<r1noncer1::R1NONCER1_SPEC>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r1noncer1;
#[doc = "R2NONCER1 (rw) register accessor: an alias for `Reg<R2NONCER1_SPEC>`"]
pub type R2NONCER1 = crate::Reg<r2noncer1::R2NONCER1_SPEC>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r2noncer1;
#[doc = "R3NONCER1 (rw) register accessor: an alias for `Reg<R3NONCER1_SPEC>`"]
pub type R3NONCER1 = crate::Reg<r3noncer1::R3NONCER1_SPEC>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r3noncer1;
#[doc = "R4NONCER1 (rw) register accessor: an alias for `Reg<R4NONCER1_SPEC>`"]
pub type R4NONCER1 = crate::Reg<r4noncer1::R4NONCER1_SPEC>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r4noncer1;
#[doc = "R1KEYR0 (w) register accessor: an alias for `Reg<R1KEYR0_SPEC>`"]
pub type R1KEYR0 = crate::Reg<r1keyr0::R1KEYR0_SPEC>;
#[doc = "OTFDEC region x key register 0"]
pub mod r1keyr0;
#[doc = "R2KEYR0 (w) register accessor: an alias for `Reg<R2KEYR0_SPEC>`"]
pub type R2KEYR0 = crate::Reg<r2keyr0::R2KEYR0_SPEC>;
#[doc = "OTFDEC region x key register 0"]
pub mod r2keyr0;
#[doc = "R3KEYR0 (w) register accessor: an alias for `Reg<R3KEYR0_SPEC>`"]
pub type R3KEYR0 = crate::Reg<r3keyr0::R3KEYR0_SPEC>;
#[doc = "OTFDEC region x key register 0"]
pub mod r3keyr0;
#[doc = "R4KEYR0 (w) register accessor: an alias for `Reg<R4KEYR0_SPEC>`"]
pub type R4KEYR0 = crate::Reg<r4keyr0::R4KEYR0_SPEC>;
#[doc = "OTFDEC region x key register 0"]
pub mod r4keyr0;
#[doc = "R1KEYR1 (w) register accessor: an alias for `Reg<R1KEYR1_SPEC>`"]
pub type R1KEYR1 = crate::Reg<r1keyr1::R1KEYR1_SPEC>;
#[doc = "OTFDEC region x key register 1"]
pub mod r1keyr1;
#[doc = "R2KEYR1 (w) register accessor: an alias for `Reg<R2KEYR1_SPEC>`"]
pub type R2KEYR1 = crate::Reg<r2keyr1::R2KEYR1_SPEC>;
#[doc = "OTFDEC region x key register 1"]
pub mod r2keyr1;
#[doc = "R3KEYR1 (w) register accessor: an alias for `Reg<R3KEYR1_SPEC>`"]
pub type R3KEYR1 = crate::Reg<r3keyr1::R3KEYR1_SPEC>;
#[doc = "OTFDEC region x key register 1"]
pub mod r3keyr1;
#[doc = "R4KEYR1 (w) register accessor: an alias for `Reg<R4KEYR1_SPEC>`"]
pub type R4KEYR1 = crate::Reg<r4keyr1::R4KEYR1_SPEC>;
#[doc = "OTFDEC region x key register 1"]
pub mod r4keyr1;
#[doc = "R1KEYR2 (w) register accessor: an alias for `Reg<R1KEYR2_SPEC>`"]
pub type R1KEYR2 = crate::Reg<r1keyr2::R1KEYR2_SPEC>;
#[doc = "OTFDEC region x key register 2"]
pub mod r1keyr2;
#[doc = "R2KEYR2 (w) register accessor: an alias for `Reg<R2KEYR2_SPEC>`"]
pub type R2KEYR2 = crate::Reg<r2keyr2::R2KEYR2_SPEC>;
#[doc = "OTFDEC region x key register 2"]
pub mod r2keyr2;
#[doc = "R3KEYR2 (w) register accessor: an alias for `Reg<R3KEYR2_SPEC>`"]
pub type R3KEYR2 = crate::Reg<r3keyr2::R3KEYR2_SPEC>;
#[doc = "OTFDEC region x key register 2"]
pub mod r3keyr2;
#[doc = "R4KEYR2 (w) register accessor: an alias for `Reg<R4KEYR2_SPEC>`"]
pub type R4KEYR2 = crate::Reg<r4keyr2::R4KEYR2_SPEC>;
#[doc = "OTFDEC region x key register 2"]
pub mod r4keyr2;
#[doc = "R1KEYR3 (w) register accessor: an alias for `Reg<R1KEYR3_SPEC>`"]
pub type R1KEYR3 = crate::Reg<r1keyr3::R1KEYR3_SPEC>;
#[doc = "OTFDEC region x key register 3"]
pub mod r1keyr3;
#[doc = "R2KEYR3 (w) register accessor: an alias for `Reg<R2KEYR3_SPEC>`"]
pub type R2KEYR3 = crate::Reg<r2keyr3::R2KEYR3_SPEC>;
#[doc = "OTFDEC region x key register 3"]
pub mod r2keyr3;
#[doc = "R3KEYR3 (w) register accessor: an alias for `Reg<R3KEYR3_SPEC>`"]
pub type R3KEYR3 = crate::Reg<r3keyr3::R3KEYR3_SPEC>;
#[doc = "OTFDEC region x key register 3"]
pub mod r3keyr3;
#[doc = "R4KEYR3 (w) register accessor: an alias for `Reg<R4KEYR3_SPEC>`"]
pub type R4KEYR3 = crate::Reg<r4keyr3::R4KEYR3_SPEC>;
#[doc = "OTFDEC region x key register 3"]
pub mod r4keyr3;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "OTFDEC interrupt status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "OTFDEC interrupt clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "OTFDEC interrupt enable register"]
pub mod ier;
