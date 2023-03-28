#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x14 - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x18 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x1c - GPIO reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x20 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x28 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x2c - GPIO clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x30 - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x34 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x38 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x3c - GPIO clock enable in sleep mode register"]
    pub iopsmen: IOPSMEN,
    #[doc = "0x40 - AHB peripheral clock enable in sleep mode register"]
    pub ahbsmenr: AHBSMENR,
    #[doc = "0x44 - APB2 peripheral clock enable in sleep mode register"]
    pub apb2smenr: APB2SMENR,
    #[doc = "0x48 - APB1 peripheral clock enable in sleep mode register"]
    pub apb1smenr: APB1SMENR,
    #[doc = "0x4c - Clock configuration register"]
    pub ccipr: CCIPR,
    #[doc = "0x50 - Control and status register"]
    pub csr: CSR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "CIER (r) register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "IOPRSTR (rw) register accessor: an alias for `Reg<IOPRSTR_SPEC>`"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "IOPENR (rw) register accessor: an alias for `Reg<IOPENR_SPEC>`"]
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "IOPSMEN (rw) register accessor: an alias for `Reg<IOPSMEN_SPEC>`"]
pub type IOPSMEN = crate::Reg<iopsmen::IOPSMEN_SPEC>;
#[doc = "GPIO clock enable in sleep mode register"]
pub mod iopsmen;
#[doc = "AHBSMENR (rw) register accessor: an alias for `Reg<AHBSMENR_SPEC>`"]
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB2SMENR (rw) register accessor: an alias for `Reg<APB2SMENR_SPEC>`"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub mod apb2smenr;
#[doc = "APB1SMENR (rw) register accessor: an alias for `Reg<APB1SMENR_SPEC>`"]
pub type APB1SMENR = crate::Reg<apb1smenr::APB1SMENR_SPEC>;
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub mod apb1smenr;
#[doc = "CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Clock configuration register"]
pub mod ccipr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
