#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Clock configuration register (RCC_CFGR)"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Clock interrupt register (RCC_CIR)"]
    pub cir: CIR,
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)"]
    pub ahbenr: AHBENR,
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)"]
    pub apb2enr: APB2ENR,
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)"]
    pub apb1enr: APB1ENR,
    #[doc = "0x20 - Backup domain control register (RCC_BDCR)"]
    pub bdcr: BDCR,
    #[doc = "0x24 - Control/status register (RCC_CSR)"]
    pub csr: CSR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register (RCC_CFGR)"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "Clock interrupt register (RCC_CIR)"]
pub mod cir;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
pub mod apb1rstr;
#[doc = "AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
pub mod apb1enr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register (RCC_BDCR)"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register (RCC_CSR)"]
pub mod csr;
