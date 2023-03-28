#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCC clock control register"]
    pub cr: CR,
    #[doc = "0x04 - RCC internal clock source calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - RCC clock configuration register"]
    pub cfgr: CFGR,
    _reserved3: [u8; 0x0c],
    #[doc = "0x18 - RCC clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - RCC clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - RCC clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x24 - RCC I/O port reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x28 - RCC AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x2c - RCC APB peripheral reset register 1"]
    pub apbrstr1: APBRSTR1,
    #[doc = "0x30 - RCC APB peripheral reset register 2"]
    pub apbrstr2: APBRSTR2,
    #[doc = "0x34 - RCC I/O port clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x38 - RCC AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x3c - RCC APB peripheral clock enable register 1"]
    pub apbenr1: APBENR1,
    #[doc = "0x40 - RCC APB peripheral clock enable register 2"]
    pub apbenr2: APBENR2,
    #[doc = "0x44 - RCC I/O port in Sleep mode clock enable register"]
    pub iopsmenr: IOPSMENR,
    #[doc = "0x48 - RCC AHB peripheral clock enable in Sleep/Stop mode register"]
    pub ahbsmenr: AHBSMENR,
    #[doc = "0x4c - RCC APB peripheral clock enable in Sleep/Stop mode register 1"]
    pub apbsmenr1: APBSMENR1,
    #[doc = "0x50 - RCC APB peripheral clock enable in Sleep/Stop mode register 2"]
    pub apbsmenr2: APBSMENR2,
    #[doc = "0x54 - RCC peripherals independent clock configuration register"]
    pub ccipr: CCIPR,
    _reserved19: [u8; 0x04],
    #[doc = "0x5c - RCC control/status register 1"]
    pub csr1: CSR1,
    #[doc = "0x60 - RCC control/status register 2"]
    pub csr2: CSR2,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RCC clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "RCC internal clock source calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "RCC clock configuration register"]
pub mod cfgr;
#[doc = "CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "RCC clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "RCC clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "RCC clock interrupt clear register"]
pub mod cicr;
#[doc = "IOPRSTR (rw) register accessor: an alias for `Reg<IOPRSTR_SPEC>`"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
#[doc = "RCC I/O port reset register"]
pub mod ioprstr;
#[doc = "AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "RCC AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APBRSTR1 (rw) register accessor: an alias for `Reg<APBRSTR1_SPEC>`"]
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
#[doc = "RCC APB peripheral reset register 1"]
pub mod apbrstr1;
#[doc = "APBRSTR2 (rw) register accessor: an alias for `Reg<APBRSTR2_SPEC>`"]
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
#[doc = "RCC APB peripheral reset register 2"]
pub mod apbrstr2;
#[doc = "IOPENR (rw) register accessor: an alias for `Reg<IOPENR_SPEC>`"]
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
#[doc = "RCC I/O port clock enable register"]
pub mod iopenr;
#[doc = "AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "RCC AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APBENR1 (rw) register accessor: an alias for `Reg<APBENR1_SPEC>`"]
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
#[doc = "RCC APB peripheral clock enable register 1"]
pub mod apbenr1;
#[doc = "APBENR2 (rw) register accessor: an alias for `Reg<APBENR2_SPEC>`"]
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
#[doc = "RCC APB peripheral clock enable register 2"]
pub mod apbenr2;
#[doc = "IOPSMENR (rw) register accessor: an alias for `Reg<IOPSMENR_SPEC>`"]
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENR_SPEC>;
#[doc = "RCC I/O port in Sleep mode clock enable register"]
pub mod iopsmenr;
#[doc = "AHBSMENR (rw) register accessor: an alias for `Reg<AHBSMENR_SPEC>`"]
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
#[doc = "RCC AHB peripheral clock enable in Sleep/Stop mode register"]
pub mod ahbsmenr;
#[doc = "APBSMENR1 (rw) register accessor: an alias for `Reg<APBSMENR1_SPEC>`"]
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1_SPEC>;
#[doc = "RCC APB peripheral clock enable in Sleep/Stop mode register 1"]
pub mod apbsmenr1;
#[doc = "APBSMENR2 (rw) register accessor: an alias for `Reg<APBSMENR2_SPEC>`"]
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2_SPEC>;
#[doc = "RCC APB peripheral clock enable in Sleep/Stop mode register 2"]
pub mod apbsmenr2;
#[doc = "CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "RCC peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "CSR1 (rw) register accessor: an alias for `Reg<CSR1_SPEC>`"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "RCC control/status register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: an alias for `Reg<CSR2_SPEC>`"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "RCC control/status register 2"]
pub mod csr2;
