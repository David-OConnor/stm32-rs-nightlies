#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCC clock control register"]
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - RCC HSI calibration register"]
    pub hsicfgr: HSICFGR,
    #[doc = "0x14 - RCC clock recovery RC register"]
    pub crrcr: CRRCR,
    #[doc = "0x18 - RCC CSI calibration register"]
    pub csicfgr: CSICFGR,
    #[doc = "0x1c - RCC clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x20 - RCC CPU domain clock configuration register 2"]
    pub cfgr2: CFGR2,
    _reserved6: [u8; 0x04],
    #[doc = "0x28 - RCC PLL clock source selection register"]
    pub pll1cfgr: PLL1CFGR,
    #[doc = "0x2c - RCC PLL clock source selection register"]
    pub pll2cfgr: PLL2CFGR,
    #[doc = "0x30 - RCC PLL clock source selection register"]
    pub pll3cfgr: PLL3CFGR,
    #[doc = "0x34 - RCC PLL1 dividers register"]
    pub pll1divr: PLL1DIVR,
    #[doc = "0x38 - RCC PLL1 fractional divider register"]
    pub pll1fracr: PLL1FRACR,
    #[doc = "0x3c - RCC PLL1 dividers register"]
    pub pll2divr: PLL2DIVR,
    #[doc = "0x40 - RCC PLL2 fractional divider register"]
    pub pll2fracr: PLL2FRACR,
    #[doc = "0x44 - RCC PLL3 dividers register"]
    pub pll3divr: PLL3DIVR,
    #[doc = "0x48 - RCC PLL3 fractional divider register"]
    pub pll3fracr: PLL3FRACR,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - RCC clock source interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x54 - RCC clock source interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x58 - RCC clock source interrupt clear register"]
    pub cicr: CICR,
    _reserved18: [u8; 0x04],
    #[doc = "0x60 - RCC AHB1 reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x64 - RCC AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    _reserved20: [u8; 0x04],
    #[doc = "0x6c - RCC AHB4 peripheral reset register"]
    pub ahb4rstr: AHB4RSTR,
    _reserved21: [u8; 0x04],
    #[doc = "0x74 - RCC APB1 peripheral low reset register"]
    pub apb1lrstr: APB1LRSTR,
    #[doc = "0x78 - RCC APB1 peripheral high reset register"]
    pub apb1hrstr: APB1HRSTR,
    #[doc = "0x7c - RCC APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x80 - RCC APB4 peripheral reset register"]
    pub apb3rstr: APB3RSTR,
    _reserved25: [u8; 0x04],
    #[doc = "0x88 - RCC AHB1 peripherals clock register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x8c - RCC AHB2 peripheral clock register"]
    pub ahb2enr: AHB2ENR,
    _reserved27: [u8; 0x04],
    #[doc = "0x94 - RCC AHB4 peripheral clock register"]
    pub ahb4enr: AHB4ENR,
    _reserved28: [u8; 0x04],
    #[doc = "0x9c - RCC APB1 peripheral clock register"]
    pub apb1lenr: APB1LENR,
    #[doc = "0xa0 - RCC APB1 peripheral clock register"]
    pub apb1henr: APB1HENR,
    #[doc = "0xa4 - RCC APB2 peripheral clock register"]
    pub apb2enr: APB2ENR,
    #[doc = "0xa8 - RCC APB4 peripheral clock register"]
    pub apb3enr: APB3ENR,
    _reserved32: [u8; 0x04],
    #[doc = "0xb0 - RCC AHB1 sleep clock register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0xb4 - RCC AHB2 sleep clock register"]
    pub ahb2lpenr: AHB2LPENR,
    _reserved34: [u8; 0x04],
    #[doc = "0xbc - RCC AHB4 sleep clock register"]
    pub ahb4lpenr: AHB4LPENR,
    _reserved35: [u8; 0x04],
    #[doc = "0xc4 - RCC APB1 sleep clock register"]
    pub apb1llpenr: APB1LLPENR,
    #[doc = "0xc8 - RCC APB1 sleep clock register"]
    pub apb1hlpenr: APB1HLPENR,
    #[doc = "0xcc - RCC APB2 sleep clock register"]
    pub apb2lpenr: APB2LPENR,
    #[doc = "0xd0 - RCC APB4 sleep clock register"]
    pub apb3lpenr: APB3LPENR,
    _reserved39: [u8; 0x04],
    #[doc = "0xd8 - RCC kernel clock configuration register"]
    pub ccipr1: CCIPR1,
    #[doc = "0xdc - RCC kernel clock configuration register"]
    pub ccipr2: CCIPR2,
    #[doc = "0xe0 - RCC kernel clock configuration register"]
    pub ccipr3: CCIPR3,
    #[doc = "0xe4 - RCC kernel clock configuration register"]
    pub ccipr4: CCIPR4,
    #[doc = "0xe8 - RCC kernel clock configuration register"]
    pub ccipr5: CCIPR5,
    _reserved44: [u8; 0x04],
    #[doc = "0xf0 - RCC Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0xf4 - RCC reset status register"]
    pub rsr: RSR,
    _reserved46: [u8; 0x18],
    #[doc = "0x110 - RCC secure configuration register"]
    pub seccfgr: SECCFGR,
    #[doc = "0x114 - RCC privilege configuration register"]
    pub privcfgr: PRIVCFGR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RCC clock control register"]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: an alias for `Reg<HSICFGR_SPEC>`"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
#[doc = "RCC HSI calibration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: an alias for `Reg<CRRCR_SPEC>`"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "RCC clock recovery RC register"]
pub mod crrcr;
#[doc = "CSICFGR (rw) register accessor: an alias for `Reg<CSICFGR_SPEC>`"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
#[doc = "RCC CSI calibration register"]
pub mod csicfgr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "RCC clock configuration register"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "RCC CPU domain clock configuration register 2"]
pub mod cfgr2;
#[doc = "PLL1CFGR (rw) register accessor: an alias for `Reg<PLL1CFGR_SPEC>`"]
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGR_SPEC>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll1cfgr;
#[doc = "PLL2CFGR (rw) register accessor: an alias for `Reg<PLL2CFGR_SPEC>`"]
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGR_SPEC>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll2cfgr;
#[doc = "PLL3CFGR (rw) register accessor: an alias for `Reg<PLL3CFGR_SPEC>`"]
pub type PLL3CFGR = crate::Reg<pll3cfgr::PLL3CFGR_SPEC>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll3cfgr;
#[doc = "PLL1DIVR (rw) register accessor: an alias for `Reg<PLL1DIVR_SPEC>`"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: an alias for `Reg<PLL1FRACR_SPEC>`"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
#[doc = "RCC PLL1 fractional divider register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: an alias for `Reg<PLL2DIVR_SPEC>`"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: an alias for `Reg<PLL2FRACR_SPEC>`"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
#[doc = "RCC PLL2 fractional divider register"]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: an alias for `Reg<PLL3DIVR_SPEC>`"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVR_SPEC>;
#[doc = "RCC PLL3 dividers register"]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: an alias for `Reg<PLL3FRACR_SPEC>`"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACR_SPEC>;
#[doc = "RCC PLL3 fractional divider register"]
pub mod pll3fracr;
#[doc = "CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "RCC clock source interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "RCC clock source interrupt flag register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "RCC clock source interrupt clear register"]
pub mod cicr;
#[doc = "AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "RCC AHB1 reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "RCC AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: an alias for `Reg<AHB4RSTR_SPEC>`"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTR_SPEC>;
#[doc = "RCC AHB4 peripheral reset register"]
pub mod ahb4rstr;
#[doc = "APB1LRSTR (rw) register accessor: an alias for `Reg<APB1LRSTR_SPEC>`"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
#[doc = "RCC APB1 peripheral low reset register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: an alias for `Reg<APB1HRSTR_SPEC>`"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
#[doc = "RCC APB1 peripheral high reset register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "RCC APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "RCC APB4 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "RCC AHB1 peripherals clock register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "RCC AHB2 peripheral clock register"]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: an alias for `Reg<AHB4ENR_SPEC>`"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENR_SPEC>;
#[doc = "RCC AHB4 peripheral clock register"]
pub mod ahb4enr;
#[doc = "APB1LENR (rw) register accessor: an alias for `Reg<APB1LENR_SPEC>`"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1lenr;
#[doc = "APB1HENR (rw) register accessor: an alias for `Reg<APB1HENR_SPEC>`"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1henr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "RCC APB2 peripheral clock register"]
pub mod apb2enr;
#[doc = "APB3ENR (rw) register accessor: an alias for `Reg<APB3ENR_SPEC>`"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "RCC APB4 peripheral clock register"]
pub mod apb3enr;
#[doc = "AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 sleep clock register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: an alias for `Reg<AHB2LPENR_SPEC>`"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 sleep clock register"]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: an alias for `Reg<AHB4LPENR_SPEC>`"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>;
#[doc = "RCC AHB4 sleep clock register"]
pub mod ahb4lpenr;
#[doc = "APB1LLPENR (rw) register accessor: an alias for `Reg<APB1LLPENR_SPEC>`"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1llpenr;
#[doc = "APB1HLPENR (rw) register accessor: an alias for `Reg<APB1HLPENR_SPEC>`"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "RCC APB2 sleep clock register"]
pub mod apb2lpenr;
#[doc = "APB3LPENR (rw) register accessor: an alias for `Reg<APB3LPENR_SPEC>`"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
#[doc = "RCC APB4 sleep clock register"]
pub mod apb3lpenr;
#[doc = "CCIPR1 (rw) register accessor: an alias for `Reg<CCIPR1_SPEC>`"]
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr1;
#[doc = "CCIPR2 (rw) register accessor: an alias for `Reg<CCIPR2_SPEC>`"]
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr2;
#[doc = "CCIPR3 (rw) register accessor: an alias for `Reg<CCIPR3_SPEC>`"]
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr3;
#[doc = "CCIPR4 (rw) register accessor: an alias for `Reg<CCIPR4_SPEC>`"]
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr4;
#[doc = "CCIPR5 (rw) register accessor: an alias for `Reg<CCIPR5_SPEC>`"]
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr5;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RCC Backup domain control register"]
pub mod bdcr;
#[doc = "RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "RCC reset status register"]
pub mod rsr;
#[doc = "SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
#[doc = "RCC secure configuration register"]
pub mod seccfgr;
#[doc = "PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "RCC privilege configuration register"]
pub mod privcfgr;
