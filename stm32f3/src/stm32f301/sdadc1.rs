#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x0c - interrupt and status clear register"]
    pub clrisr: CLRISR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - injected channel group selection register"]
    pub jchgr: JCHGR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - configuration 0 register"]
    pub conf0r: CONF0R,
    #[doc = "0x24 - configuration 1 register"]
    pub conf1r: CONF1R,
    #[doc = "0x28 - configuration 2 register"]
    pub conf2r: CONF2R,
    _reserved8: [u8; 0x14],
    #[doc = "0x40 - channel configuration register 1"]
    pub confchr1: CONFCHR1,
    #[doc = "0x44 - channel configuration register 2"]
    pub confchr2: CONFCHR2,
    _reserved10: [u8; 0x18],
    #[doc = "0x60 - data register for injected group"]
    pub jdatar: JDATAR,
    #[doc = "0x64 - data register for the regular channel"]
    pub rdatar: RDATAR,
    _reserved12: [u8; 0x08],
    #[doc = "0x70 - SDADC1 and SDADC2 injected data register"]
    pub jdata12r: JDATA12R,
    #[doc = "0x74 - SDADC1 and SDADC2 regular data register"]
    pub rdata12r: RDATA12R,
    #[doc = "0x78 - SDADC1 and SDADC3 injected data register"]
    pub jdata13r: JDATA13R,
    #[doc = "0x7c - SDADC1 and SDADC3 regular data register"]
    pub rdata13r: RDATA13R,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "CLRISR (rw) register accessor: an alias for `Reg<CLRISR_SPEC>`"]
pub type CLRISR = crate::Reg<clrisr::CLRISR_SPEC>;
#[doc = "interrupt and status clear register"]
pub mod clrisr;
#[doc = "JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`"]
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod jchgr;
#[doc = "CONF0R (rw) register accessor: an alias for `Reg<CONF0R_SPEC>`"]
pub type CONF0R = crate::Reg<conf0r::CONF0R_SPEC>;
#[doc = "configuration 0 register"]
pub mod conf0r;
#[doc = "CONF1R (rw) register accessor: an alias for `Reg<CONF1R_SPEC>`"]
pub type CONF1R = crate::Reg<conf1r::CONF1R_SPEC>;
#[doc = "configuration 1 register"]
pub mod conf1r;
#[doc = "CONF2R (rw) register accessor: an alias for `Reg<CONF2R_SPEC>`"]
pub type CONF2R = crate::Reg<conf2r::CONF2R_SPEC>;
#[doc = "configuration 2 register"]
pub mod conf2r;
#[doc = "CONFCHR1 (rw) register accessor: an alias for `Reg<CONFCHR1_SPEC>`"]
pub type CONFCHR1 = crate::Reg<confchr1::CONFCHR1_SPEC>;
#[doc = "channel configuration register 1"]
pub mod confchr1;
#[doc = "CONFCHR2 (rw) register accessor: an alias for `Reg<CONFCHR2_SPEC>`"]
pub type CONFCHR2 = crate::Reg<confchr2::CONFCHR2_SPEC>;
#[doc = "channel configuration register 2"]
pub mod confchr2;
#[doc = "JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`"]
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod jdatar;
#[doc = "RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`"]
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod rdatar;
#[doc = "JDATA12R (r) register accessor: an alias for `Reg<JDATA12R_SPEC>`"]
pub type JDATA12R = crate::Reg<jdata12r::JDATA12R_SPEC>;
#[doc = "SDADC1 and SDADC2 injected data register"]
pub mod jdata12r;
#[doc = "RDATA12R (r) register accessor: an alias for `Reg<RDATA12R_SPEC>`"]
pub type RDATA12R = crate::Reg<rdata12r::RDATA12R_SPEC>;
#[doc = "SDADC1 and SDADC2 regular data register"]
pub mod rdata12r;
#[doc = "JDATA13R (r) register accessor: an alias for `Reg<JDATA13R_SPEC>`"]
pub type JDATA13R = crate::Reg<jdata13r::JDATA13R_SPEC>;
#[doc = "SDADC1 and SDADC3 injected data register"]
pub mod jdata13r;
#[doc = "RDATA13R (r) register accessor: an alias for `Reg<RDATA13R_SPEC>`"]
pub type RDATA13R = crate::Reg<rdata13r::RDATA13R_SPEC>;
#[doc = "SDADC1 and SDADC3 regular data register"]
pub mod rdata13r;
