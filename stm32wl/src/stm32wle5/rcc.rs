#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: CICR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub ahb3rstr: AHB3RSTR,
    _reserved10: [u8; 0x04],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub apb1rstr1: APB1RSTR1,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub apb1rstr2: APB1RSTR2,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x44 - APB3 peripheral reset register"]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub ahb3enr: AHB3ENR,
    _reserved17: [u8; 0x04],
    #[doc = "0x58 - APB1 peripheral clock enable register 1"]
    pub apb1enr1: APB1ENR1,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub apb1enr2: APB1ENR2,
    #[doc = "0x60 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x64 - APB3 peripheral clock enable register"]
    pub apb3enr: APB3ENR,
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep modes register"]
    pub ahb1smenr: AHB1SMENR,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep modes register"]
    pub ahb2smenr: AHB2SMENR,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb3smenr: AHB3SMENR,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - APB1 peripheral clocks enable in Sleep mode register 1"]
    pub apb1smenr1: APB1SMENR1,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep mode register 2"]
    pub apb1smenr2: APB1SMENR2,
    #[doc = "0x80 - APB2 peripheral clocks enable in Sleep mode register"]
    pub apb2smenr: APB2SMENR,
    #[doc = "0x84 - APB3 peripheral clock enable in Sleep mode register"]
    pub apb3smenr: APB3SMENR,
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    pub ccipr: CCIPR,
    _reserved29: [u8; 0x04],
    #[doc = "0x90 - Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x94 - Control/status register"]
    pub csr: CSR,
    _reserved31: [u8; 0x70],
    #[doc = "0x108 - Extended clock recovery register"]
    pub extcfgr: EXTCFGR,
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
#[doc = "PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`"]
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
#[doc = "AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3RSTR (rw) register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1RSTR1 (rw) register accessor: an alias for `Reg<APB1RSTR1_SPEC>`"]
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1_SPEC>;
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1RSTR2 (rw) register accessor: an alias for `Reg<APB1RSTR2_SPEC>`"]
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2_SPEC>;
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "APB3 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3ENR (rw) register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1ENR1 (rw) register accessor: an alias for `Reg<APB1ENR1_SPEC>`"]
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1_SPEC>;
#[doc = "APB1 peripheral clock enable register 1"]
pub mod apb1enr1;
#[doc = "APB1ENR2 (rw) register accessor: an alias for `Reg<APB1ENR2_SPEC>`"]
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2_SPEC>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB3ENR (rw) register accessor: an alias for `Reg<APB3ENR_SPEC>`"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "APB3 peripheral clock enable register"]
pub mod apb3enr;
#[doc = "AHB1SMENR (rw) register accessor: an alias for `Reg<AHB1SMENR_SPEC>`"]
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENR_SPEC>;
#[doc = "AHB1 peripheral clocks enable in Sleep modes register"]
pub mod ahb1smenr;
#[doc = "AHB2SMENR (rw) register accessor: an alias for `Reg<AHB2SMENR_SPEC>`"]
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENR_SPEC>;
#[doc = "AHB2 peripheral clocks enable in Sleep modes register"]
pub mod ahb2smenr;
#[doc = "AHB3SMENR (rw) register accessor: an alias for `Reg<AHB3SMENR_SPEC>`"]
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENR_SPEC>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1 (rw) register accessor: an alias for `Reg<APB1SMENR1_SPEC>`"]
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 1"]
pub mod apb1smenr1;
#[doc = "APB1SMENR2 (rw) register accessor: an alias for `Reg<APB1SMENR2_SPEC>`"]
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR (rw) register accessor: an alias for `Reg<APB2SMENR_SPEC>`"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
#[doc = "APB2 peripheral clocks enable in Sleep mode register"]
pub mod apb2smenr;
#[doc = "APB3SMENR (rw) register accessor: an alias for `Reg<APB3SMENR_SPEC>`"]
pub type APB3SMENR = crate::Reg<apb3smenr::APB3SMENR_SPEC>;
#[doc = "APB3 peripheral clock enable in Sleep mode register"]
pub mod apb3smenr;
#[doc = "CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register"]
pub mod csr;
#[doc = "EXTCFGR (rw) register accessor: an alias for `Reg<EXTCFGR_SPEC>`"]
pub type EXTCFGR = crate::Reg<extcfgr::EXTCFGR_SPEC>;
#[doc = "Extended clock recovery register"]
pub mod extcfgr;
