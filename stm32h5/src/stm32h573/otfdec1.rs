#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTFDEC control register"]
    pub otfdec_cr: OTFDEC_CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - OTFDEC_PRIVCFGR"]
    pub otfdec_privcfgr: OTFDEC_PRIVCFGR,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - OTFDEC region 1 configuration register"]
    pub otfdec_r1cfgr: OTFDEC_R1CFGR,
    #[doc = "0x24 - OTFDEC region 1 start address register"]
    pub otfdec_r1startaddr: OTFDEC_R1STARTADDR,
    #[doc = "0x28 - OTFDEC region 1 end address register"]
    pub otfdec_r1endaddr: OTFDEC_R1ENDADDR,
    #[doc = "0x2c - OTFDEC region 1 nonce register 0"]
    pub otfdec_r1noncer0: OTFDEC_R1NONCER0,
    #[doc = "0x30 - OTFDEC region 1 nonce register 1"]
    pub otfdec_r1noncer1: OTFDEC_R1NONCER1,
    #[doc = "0x34 - OTFDEC region 1 key register 0"]
    pub otfdec_r1keyr0: OTFDEC_R1KEYR0,
    #[doc = "0x38 - OTFDEC region 1 key register 1"]
    pub otfdec_r1keyr1: OTFDEC_R1KEYR1,
    #[doc = "0x3c - OTFDEC region 1 key register 2"]
    pub otfdec_r1keyr2: OTFDEC_R1KEYR2,
    #[doc = "0x40 - OTFDEC region 1 key register 3"]
    pub otfdec_r1keyr3: OTFDEC_R1KEYR3,
    _reserved11: [u8; 0x0c],
    #[doc = "0x50 - OTFDEC region 2 configuration register"]
    pub otfdec_r2cfgr: OTFDEC_R2CFGR,
    #[doc = "0x54 - OTFDEC region 2 start address register"]
    pub otfdec_r2startaddr: OTFDEC_R2STARTADDR,
    #[doc = "0x58 - OTFDEC region 2 end address register"]
    pub otfdec_r2endaddr: OTFDEC_R2ENDADDR,
    #[doc = "0x5c - OTFDEC region 2 nonce register 0"]
    pub otfdec_r2noncer0: OTFDEC_R2NONCER0,
    #[doc = "0x60 - OTFDEC region 2 nonce register 1"]
    pub otfdec_r2noncer1: OTFDEC_R2NONCER1,
    #[doc = "0x64 - OTFDEC region 2 key register 0"]
    pub otfdec_r2keyr0: OTFDEC_R2KEYR0,
    #[doc = "0x68 - OTFDEC region 2 key register 1"]
    pub otfdec_r2keyr1: OTFDEC_R2KEYR1,
    #[doc = "0x6c - OTFDEC region 2 key register 2"]
    pub otfdec_r2keyr2: OTFDEC_R2KEYR2,
    #[doc = "0x70 - OTFDEC region 2 key register 3"]
    pub otfdec_r2keyr3: OTFDEC_R2KEYR3,
    _reserved20: [u8; 0x0c],
    #[doc = "0x80 - OTFDEC region 3 configuration register"]
    pub otfdec_r3cfgr: OTFDEC_R3CFGR,
    #[doc = "0x84 - OTFDEC region 3 start address register"]
    pub otfdec_r3startaddr: OTFDEC_R3STARTADDR,
    #[doc = "0x88 - OTFDEC region 3 end address register"]
    pub otfdec_r3endaddr: OTFDEC_R3ENDADDR,
    #[doc = "0x8c - OTFDEC region 3 nonce register 0"]
    pub otfdec_r3noncer0: OTFDEC_R3NONCER0,
    #[doc = "0x90 - OTFDEC region 3 nonce register 1"]
    pub otfdec_r3noncer1: OTFDEC_R3NONCER1,
    #[doc = "0x94 - OTFDEC region 3 key register 0"]
    pub otfdec_r3keyr0: OTFDEC_R3KEYR0,
    #[doc = "0x98 - OTFDEC region 3 key register 1"]
    pub otfdec_r3keyr1: OTFDEC_R3KEYR1,
    #[doc = "0x9c - OTFDEC region 3 key register 2"]
    pub otfdec_r3keyr2: OTFDEC_R3KEYR2,
    #[doc = "0xa0 - OTFDEC region 3 key register 3"]
    pub otfdec_r3keyr3: OTFDEC_R3KEYR3,
    _reserved29: [u8; 0x0c],
    #[doc = "0xb0 - OTFDEC region 4 configuration register"]
    pub otfdec_r4cfgr: OTFDEC_R4CFGR,
    #[doc = "0xb4 - OTFDEC region 4 start address register"]
    pub otfdec_r4startaddr: OTFDEC_R4STARTADDR,
    #[doc = "0xb8 - OTFDEC region 4 end address register"]
    pub otfdec_r4endaddr: OTFDEC_R4ENDADDR,
    #[doc = "0xbc - OTFDEC region 4 nonce register 0"]
    pub otfdec_r4noncer0: OTFDEC_R4NONCER0,
    #[doc = "0xc0 - OTFDEC region 4 nonce register 1"]
    pub otfdec_r4noncer1: OTFDEC_R4NONCER1,
    #[doc = "0xc4 - OTFDEC region 4 key register 0"]
    pub otfdec_r4keyr0: OTFDEC_R4KEYR0,
    #[doc = "0xc8 - OTFDEC region 4 key register 1"]
    pub otfdec_r4keyr1: OTFDEC_R4KEYR1,
    #[doc = "0xcc - OTFDEC region 4 key register 2"]
    pub otfdec_r4keyr2: OTFDEC_R4KEYR2,
    #[doc = "0xd0 - OTFDEC region 4 key register 3"]
    pub otfdec_r4keyr3: OTFDEC_R4KEYR3,
    _reserved38: [u8; 0x022c],
    #[doc = "0x300 - OTFDEC interrupt status register"]
    pub otfdec_isr: OTFDEC_ISR,
    #[doc = "0x304 - OTFDEC interrupt clear register"]
    pub otfdec_icr: OTFDEC_ICR,
    #[doc = "0x308 - OTFDEC interrupt enable register"]
    pub otfdec_ier: OTFDEC_IER,
}
#[doc = "OTFDEC_CR (rw) register accessor: an alias for `Reg<OTFDEC_CR_SPEC>`"]
pub type OTFDEC_CR = crate::Reg<otfdec_cr::OTFDEC_CR_SPEC>;
#[doc = "OTFDEC control register"]
pub mod otfdec_cr;
#[doc = "OTFDEC_PRIVCFGR (rw) register accessor: an alias for `Reg<OTFDEC_PRIVCFGR_SPEC>`"]
pub type OTFDEC_PRIVCFGR = crate::Reg<otfdec_privcfgr::OTFDEC_PRIVCFGR_SPEC>;
#[doc = "OTFDEC_PRIVCFGR"]
pub mod otfdec_privcfgr;
#[doc = "OTFDEC_R1CFGR (rw) register accessor: an alias for `Reg<OTFDEC_R1CFGR_SPEC>`"]
pub type OTFDEC_R1CFGR = crate::Reg<otfdec_r1cfgr::OTFDEC_R1CFGR_SPEC>;
#[doc = "OTFDEC region 1 configuration register"]
pub mod otfdec_r1cfgr;
#[doc = "OTFDEC_R1STARTADDR (rw) register accessor: an alias for `Reg<OTFDEC_R1STARTADDR_SPEC>`"]
pub type OTFDEC_R1STARTADDR = crate::Reg<otfdec_r1startaddr::OTFDEC_R1STARTADDR_SPEC>;
#[doc = "OTFDEC region 1 start address register"]
pub mod otfdec_r1startaddr;
#[doc = "OTFDEC_R1ENDADDR (rw) register accessor: an alias for `Reg<OTFDEC_R1ENDADDR_SPEC>`"]
pub type OTFDEC_R1ENDADDR = crate::Reg<otfdec_r1endaddr::OTFDEC_R1ENDADDR_SPEC>;
#[doc = "OTFDEC region 1 end address register"]
pub mod otfdec_r1endaddr;
#[doc = "OTFDEC_R1NONCER0 (rw) register accessor: an alias for `Reg<OTFDEC_R1NONCER0_SPEC>`"]
pub type OTFDEC_R1NONCER0 = crate::Reg<otfdec_r1noncer0::OTFDEC_R1NONCER0_SPEC>;
#[doc = "OTFDEC region 1 nonce register 0"]
pub mod otfdec_r1noncer0;
#[doc = "OTFDEC_R1NONCER1 (rw) register accessor: an alias for `Reg<OTFDEC_R1NONCER1_SPEC>`"]
pub type OTFDEC_R1NONCER1 = crate::Reg<otfdec_r1noncer1::OTFDEC_R1NONCER1_SPEC>;
#[doc = "OTFDEC region 1 nonce register 1"]
pub mod otfdec_r1noncer1;
#[doc = "OTFDEC_R1KEYR0 (w) register accessor: an alias for `Reg<OTFDEC_R1KEYR0_SPEC>`"]
pub type OTFDEC_R1KEYR0 = crate::Reg<otfdec_r1keyr0::OTFDEC_R1KEYR0_SPEC>;
#[doc = "OTFDEC region 1 key register 0"]
pub mod otfdec_r1keyr0;
#[doc = "OTFDEC_R1KEYR1 (w) register accessor: an alias for `Reg<OTFDEC_R1KEYR1_SPEC>`"]
pub type OTFDEC_R1KEYR1 = crate::Reg<otfdec_r1keyr1::OTFDEC_R1KEYR1_SPEC>;
#[doc = "OTFDEC region 1 key register 1"]
pub mod otfdec_r1keyr1;
#[doc = "OTFDEC_R1KEYR2 (w) register accessor: an alias for `Reg<OTFDEC_R1KEYR2_SPEC>`"]
pub type OTFDEC_R1KEYR2 = crate::Reg<otfdec_r1keyr2::OTFDEC_R1KEYR2_SPEC>;
#[doc = "OTFDEC region 1 key register 2"]
pub mod otfdec_r1keyr2;
#[doc = "OTFDEC_R1KEYR3 (w) register accessor: an alias for `Reg<OTFDEC_R1KEYR3_SPEC>`"]
pub type OTFDEC_R1KEYR3 = crate::Reg<otfdec_r1keyr3::OTFDEC_R1KEYR3_SPEC>;
#[doc = "OTFDEC region 1 key register 3"]
pub mod otfdec_r1keyr3;
#[doc = "OTFDEC_R2CFGR (rw) register accessor: an alias for `Reg<OTFDEC_R2CFGR_SPEC>`"]
pub type OTFDEC_R2CFGR = crate::Reg<otfdec_r2cfgr::OTFDEC_R2CFGR_SPEC>;
#[doc = "OTFDEC region 2 configuration register"]
pub mod otfdec_r2cfgr;
#[doc = "OTFDEC_R2STARTADDR (rw) register accessor: an alias for `Reg<OTFDEC_R2STARTADDR_SPEC>`"]
pub type OTFDEC_R2STARTADDR = crate::Reg<otfdec_r2startaddr::OTFDEC_R2STARTADDR_SPEC>;
#[doc = "OTFDEC region 2 start address register"]
pub mod otfdec_r2startaddr;
#[doc = "OTFDEC_R2ENDADDR (rw) register accessor: an alias for `Reg<OTFDEC_R2ENDADDR_SPEC>`"]
pub type OTFDEC_R2ENDADDR = crate::Reg<otfdec_r2endaddr::OTFDEC_R2ENDADDR_SPEC>;
#[doc = "OTFDEC region 2 end address register"]
pub mod otfdec_r2endaddr;
#[doc = "OTFDEC_R2NONCER0 (rw) register accessor: an alias for `Reg<OTFDEC_R2NONCER0_SPEC>`"]
pub type OTFDEC_R2NONCER0 = crate::Reg<otfdec_r2noncer0::OTFDEC_R2NONCER0_SPEC>;
#[doc = "OTFDEC region 2 nonce register 0"]
pub mod otfdec_r2noncer0;
#[doc = "OTFDEC_R2NONCER1 (rw) register accessor: an alias for `Reg<OTFDEC_R2NONCER1_SPEC>`"]
pub type OTFDEC_R2NONCER1 = crate::Reg<otfdec_r2noncer1::OTFDEC_R2NONCER1_SPEC>;
#[doc = "OTFDEC region 2 nonce register 1"]
pub mod otfdec_r2noncer1;
#[doc = "OTFDEC_R2KEYR0 (w) register accessor: an alias for `Reg<OTFDEC_R2KEYR0_SPEC>`"]
pub type OTFDEC_R2KEYR0 = crate::Reg<otfdec_r2keyr0::OTFDEC_R2KEYR0_SPEC>;
#[doc = "OTFDEC region 2 key register 0"]
pub mod otfdec_r2keyr0;
#[doc = "OTFDEC_R2KEYR1 (w) register accessor: an alias for `Reg<OTFDEC_R2KEYR1_SPEC>`"]
pub type OTFDEC_R2KEYR1 = crate::Reg<otfdec_r2keyr1::OTFDEC_R2KEYR1_SPEC>;
#[doc = "OTFDEC region 2 key register 1"]
pub mod otfdec_r2keyr1;
#[doc = "OTFDEC_R2KEYR2 (w) register accessor: an alias for `Reg<OTFDEC_R2KEYR2_SPEC>`"]
pub type OTFDEC_R2KEYR2 = crate::Reg<otfdec_r2keyr2::OTFDEC_R2KEYR2_SPEC>;
#[doc = "OTFDEC region 2 key register 2"]
pub mod otfdec_r2keyr2;
#[doc = "OTFDEC_R2KEYR3 (w) register accessor: an alias for `Reg<OTFDEC_R2KEYR3_SPEC>`"]
pub type OTFDEC_R2KEYR3 = crate::Reg<otfdec_r2keyr3::OTFDEC_R2KEYR3_SPEC>;
#[doc = "OTFDEC region 2 key register 3"]
pub mod otfdec_r2keyr3;
#[doc = "OTFDEC_R3CFGR (rw) register accessor: an alias for `Reg<OTFDEC_R3CFGR_SPEC>`"]
pub type OTFDEC_R3CFGR = crate::Reg<otfdec_r3cfgr::OTFDEC_R3CFGR_SPEC>;
#[doc = "OTFDEC region 3 configuration register"]
pub mod otfdec_r3cfgr;
#[doc = "OTFDEC_R3STARTADDR (rw) register accessor: an alias for `Reg<OTFDEC_R3STARTADDR_SPEC>`"]
pub type OTFDEC_R3STARTADDR = crate::Reg<otfdec_r3startaddr::OTFDEC_R3STARTADDR_SPEC>;
#[doc = "OTFDEC region 3 start address register"]
pub mod otfdec_r3startaddr;
#[doc = "OTFDEC_R3ENDADDR (rw) register accessor: an alias for `Reg<OTFDEC_R3ENDADDR_SPEC>`"]
pub type OTFDEC_R3ENDADDR = crate::Reg<otfdec_r3endaddr::OTFDEC_R3ENDADDR_SPEC>;
#[doc = "OTFDEC region 3 end address register"]
pub mod otfdec_r3endaddr;
#[doc = "OTFDEC_R3NONCER0 (rw) register accessor: an alias for `Reg<OTFDEC_R3NONCER0_SPEC>`"]
pub type OTFDEC_R3NONCER0 = crate::Reg<otfdec_r3noncer0::OTFDEC_R3NONCER0_SPEC>;
#[doc = "OTFDEC region 3 nonce register 0"]
pub mod otfdec_r3noncer0;
#[doc = "OTFDEC_R3NONCER1 (rw) register accessor: an alias for `Reg<OTFDEC_R3NONCER1_SPEC>`"]
pub type OTFDEC_R3NONCER1 = crate::Reg<otfdec_r3noncer1::OTFDEC_R3NONCER1_SPEC>;
#[doc = "OTFDEC region 3 nonce register 1"]
pub mod otfdec_r3noncer1;
#[doc = "OTFDEC_R3KEYR0 (w) register accessor: an alias for `Reg<OTFDEC_R3KEYR0_SPEC>`"]
pub type OTFDEC_R3KEYR0 = crate::Reg<otfdec_r3keyr0::OTFDEC_R3KEYR0_SPEC>;
#[doc = "OTFDEC region 3 key register 0"]
pub mod otfdec_r3keyr0;
#[doc = "OTFDEC_R3KEYR1 (w) register accessor: an alias for `Reg<OTFDEC_R3KEYR1_SPEC>`"]
pub type OTFDEC_R3KEYR1 = crate::Reg<otfdec_r3keyr1::OTFDEC_R3KEYR1_SPEC>;
#[doc = "OTFDEC region 3 key register 1"]
pub mod otfdec_r3keyr1;
#[doc = "OTFDEC_R3KEYR2 (w) register accessor: an alias for `Reg<OTFDEC_R3KEYR2_SPEC>`"]
pub type OTFDEC_R3KEYR2 = crate::Reg<otfdec_r3keyr2::OTFDEC_R3KEYR2_SPEC>;
#[doc = "OTFDEC region 3 key register 2"]
pub mod otfdec_r3keyr2;
#[doc = "OTFDEC_R3KEYR3 (w) register accessor: an alias for `Reg<OTFDEC_R3KEYR3_SPEC>`"]
pub type OTFDEC_R3KEYR3 = crate::Reg<otfdec_r3keyr3::OTFDEC_R3KEYR3_SPEC>;
#[doc = "OTFDEC region 3 key register 3"]
pub mod otfdec_r3keyr3;
#[doc = "OTFDEC_R4CFGR (rw) register accessor: an alias for `Reg<OTFDEC_R4CFGR_SPEC>`"]
pub type OTFDEC_R4CFGR = crate::Reg<otfdec_r4cfgr::OTFDEC_R4CFGR_SPEC>;
#[doc = "OTFDEC region 4 configuration register"]
pub mod otfdec_r4cfgr;
#[doc = "OTFDEC_R4STARTADDR (rw) register accessor: an alias for `Reg<OTFDEC_R4STARTADDR_SPEC>`"]
pub type OTFDEC_R4STARTADDR = crate::Reg<otfdec_r4startaddr::OTFDEC_R4STARTADDR_SPEC>;
#[doc = "OTFDEC region 4 start address register"]
pub mod otfdec_r4startaddr;
#[doc = "OTFDEC_R4ENDADDR (rw) register accessor: an alias for `Reg<OTFDEC_R4ENDADDR_SPEC>`"]
pub type OTFDEC_R4ENDADDR = crate::Reg<otfdec_r4endaddr::OTFDEC_R4ENDADDR_SPEC>;
#[doc = "OTFDEC region 4 end address register"]
pub mod otfdec_r4endaddr;
#[doc = "OTFDEC_R4NONCER0 (rw) register accessor: an alias for `Reg<OTFDEC_R4NONCER0_SPEC>`"]
pub type OTFDEC_R4NONCER0 = crate::Reg<otfdec_r4noncer0::OTFDEC_R4NONCER0_SPEC>;
#[doc = "OTFDEC region 4 nonce register 0"]
pub mod otfdec_r4noncer0;
#[doc = "OTFDEC_R4NONCER1 (rw) register accessor: an alias for `Reg<OTFDEC_R4NONCER1_SPEC>`"]
pub type OTFDEC_R4NONCER1 = crate::Reg<otfdec_r4noncer1::OTFDEC_R4NONCER1_SPEC>;
#[doc = "OTFDEC region 4 nonce register 1"]
pub mod otfdec_r4noncer1;
#[doc = "OTFDEC_R4KEYR0 (w) register accessor: an alias for `Reg<OTFDEC_R4KEYR0_SPEC>`"]
pub type OTFDEC_R4KEYR0 = crate::Reg<otfdec_r4keyr0::OTFDEC_R4KEYR0_SPEC>;
#[doc = "OTFDEC region 4 key register 0"]
pub mod otfdec_r4keyr0;
#[doc = "OTFDEC_R4KEYR1 (w) register accessor: an alias for `Reg<OTFDEC_R4KEYR1_SPEC>`"]
pub type OTFDEC_R4KEYR1 = crate::Reg<otfdec_r4keyr1::OTFDEC_R4KEYR1_SPEC>;
#[doc = "OTFDEC region 4 key register 1"]
pub mod otfdec_r4keyr1;
#[doc = "OTFDEC_R4KEYR2 (w) register accessor: an alias for `Reg<OTFDEC_R4KEYR2_SPEC>`"]
pub type OTFDEC_R4KEYR2 = crate::Reg<otfdec_r4keyr2::OTFDEC_R4KEYR2_SPEC>;
#[doc = "OTFDEC region 4 key register 2"]
pub mod otfdec_r4keyr2;
#[doc = "OTFDEC_R4KEYR3 (w) register accessor: an alias for `Reg<OTFDEC_R4KEYR3_SPEC>`"]
pub type OTFDEC_R4KEYR3 = crate::Reg<otfdec_r4keyr3::OTFDEC_R4KEYR3_SPEC>;
#[doc = "OTFDEC region 4 key register 3"]
pub mod otfdec_r4keyr3;
#[doc = "OTFDEC_ISR (r) register accessor: an alias for `Reg<OTFDEC_ISR_SPEC>`"]
pub type OTFDEC_ISR = crate::Reg<otfdec_isr::OTFDEC_ISR_SPEC>;
#[doc = "OTFDEC interrupt status register"]
pub mod otfdec_isr;
#[doc = "OTFDEC_ICR (w) register accessor: an alias for `Reg<OTFDEC_ICR_SPEC>`"]
pub type OTFDEC_ICR = crate::Reg<otfdec_icr::OTFDEC_ICR_SPEC>;
#[doc = "OTFDEC interrupt clear register"]
pub mod otfdec_icr;
#[doc = "OTFDEC_IER (rw) register accessor: an alias for `Reg<OTFDEC_IER_SPEC>`"]
pub type OTFDEC_IER = crate::Reg<otfdec_ier::OTFDEC_IER_SPEC>;
#[doc = "OTFDEC interrupt enable register"]
pub mod otfdec_ier;
