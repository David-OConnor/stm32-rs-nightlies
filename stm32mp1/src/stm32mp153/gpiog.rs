#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiog_moder: GPIOG_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiog_otyper: GPIOG_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiog_ospeedr: GPIOG_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiog_pupdr: GPIOG_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiog_idr: GPIOG_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiog_odr: GPIOG_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiog_bsrr: GPIOG_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiog_lckr: GPIOG_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiog_afrl: GPIOG_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiog_afrh: GPIOG_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiog_brr: GPIOG_BRR,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiog_hwcfgr10: GPIOG_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr9: GPIOG_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr8: GPIOG_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiog_hwcfgr7: GPIOG_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiog_hwcfgr6: GPIOG_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiog_hwcfgr5: GPIOG_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiog_hwcfgr4: GPIOG_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiog_hwcfgr3: GPIOG_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiog_hwcfgr2: GPIOG_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiog_hwcfgr1: GPIOG_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiog_hwcfgr0: GPIOG_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiog_verr: GPIOG_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiog_ipidr: GPIOG_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiog_sidr: GPIOG_SIDR,
}
#[doc = "GPIOG_MODER (rw) register accessor: an alias for `Reg<GPIOG_MODER_SPEC>`"]
pub type GPIOG_MODER = crate::Reg<gpiog_moder::GPIOG_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiog_moder;
#[doc = "GPIOG_OTYPER (rw) register accessor: an alias for `Reg<GPIOG_OTYPER_SPEC>`"]
pub type GPIOG_OTYPER = crate::Reg<gpiog_otyper::GPIOG_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiog_otyper;
#[doc = "GPIOG_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOG_OSPEEDR_SPEC>`"]
pub type GPIOG_OSPEEDR = crate::Reg<gpiog_ospeedr::GPIOG_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiog_ospeedr;
#[doc = "GPIOG_PUPDR (rw) register accessor: an alias for `Reg<GPIOG_PUPDR_SPEC>`"]
pub type GPIOG_PUPDR = crate::Reg<gpiog_pupdr::GPIOG_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiog_pupdr;
#[doc = "GPIOG_IDR (r) register accessor: an alias for `Reg<GPIOG_IDR_SPEC>`"]
pub type GPIOG_IDR = crate::Reg<gpiog_idr::GPIOG_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiog_idr;
#[doc = "GPIOG_ODR (rw) register accessor: an alias for `Reg<GPIOG_ODR_SPEC>`"]
pub type GPIOG_ODR = crate::Reg<gpiog_odr::GPIOG_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiog_odr;
#[doc = "GPIOG_BSRR (w) register accessor: an alias for `Reg<GPIOG_BSRR_SPEC>`"]
pub type GPIOG_BSRR = crate::Reg<gpiog_bsrr::GPIOG_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiog_bsrr;
#[doc = "GPIOG_LCKR (rw) register accessor: an alias for `Reg<GPIOG_LCKR_SPEC>`"]
pub type GPIOG_LCKR = crate::Reg<gpiog_lckr::GPIOG_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiog_lckr;
#[doc = "GPIOG_AFRL (rw) register accessor: an alias for `Reg<GPIOG_AFRL_SPEC>`"]
pub type GPIOG_AFRL = crate::Reg<gpiog_afrl::GPIOG_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiog_afrl;
#[doc = "GPIOG_AFRH (rw) register accessor: an alias for `Reg<GPIOG_AFRH_SPEC>`"]
pub type GPIOG_AFRH = crate::Reg<gpiog_afrh::GPIOG_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiog_afrh;
#[doc = "GPIOG_BRR (w) register accessor: an alias for `Reg<GPIOG_BRR_SPEC>`"]
pub type GPIOG_BRR = crate::Reg<gpiog_brr::GPIOG_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiog_brr;
#[doc = "GPIOG_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR10_SPEC>`"]
pub type GPIOG_HWCFGR10 = crate::Reg<gpiog_hwcfgr10::GPIOG_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiog_hwcfgr10;
#[doc = "GPIOG_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR9_SPEC>`"]
pub type GPIOG_HWCFGR9 = crate::Reg<gpiog_hwcfgr9::GPIOG_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr9;
#[doc = "GPIOG_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR8_SPEC>`"]
pub type GPIOG_HWCFGR8 = crate::Reg<gpiog_hwcfgr8::GPIOG_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr8;
#[doc = "GPIOG_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR7_SPEC>`"]
pub type GPIOG_HWCFGR7 = crate::Reg<gpiog_hwcfgr7::GPIOG_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiog_hwcfgr7;
#[doc = "GPIOG_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR6_SPEC>`"]
pub type GPIOG_HWCFGR6 = crate::Reg<gpiog_hwcfgr6::GPIOG_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiog_hwcfgr6;
#[doc = "GPIOG_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR5_SPEC>`"]
pub type GPIOG_HWCFGR5 = crate::Reg<gpiog_hwcfgr5::GPIOG_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiog_hwcfgr5;
#[doc = "GPIOG_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR4_SPEC>`"]
pub type GPIOG_HWCFGR4 = crate::Reg<gpiog_hwcfgr4::GPIOG_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiog_hwcfgr4;
#[doc = "GPIOG_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR3_SPEC>`"]
pub type GPIOG_HWCFGR3 = crate::Reg<gpiog_hwcfgr3::GPIOG_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiog_hwcfgr3;
#[doc = "GPIOG_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR2_SPEC>`"]
pub type GPIOG_HWCFGR2 = crate::Reg<gpiog_hwcfgr2::GPIOG_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiog_hwcfgr2;
#[doc = "GPIOG_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR1_SPEC>`"]
pub type GPIOG_HWCFGR1 = crate::Reg<gpiog_hwcfgr1::GPIOG_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiog_hwcfgr1;
#[doc = "GPIOG_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOG_HWCFGR0_SPEC>`"]
pub type GPIOG_HWCFGR0 = crate::Reg<gpiog_hwcfgr0::GPIOG_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiog_hwcfgr0;
#[doc = "GPIOG_VERR (r) register accessor: an alias for `Reg<GPIOG_VERR_SPEC>`"]
pub type GPIOG_VERR = crate::Reg<gpiog_verr::GPIOG_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiog_verr;
#[doc = "GPIOG_IPIDR (r) register accessor: an alias for `Reg<GPIOG_IPIDR_SPEC>`"]
pub type GPIOG_IPIDR = crate::Reg<gpiog_ipidr::GPIOG_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiog_ipidr;
#[doc = "GPIOG_SIDR (r) register accessor: an alias for `Reg<GPIOG_SIDR_SPEC>`"]
pub type GPIOG_SIDR = crate::Reg<gpiog_sidr::GPIOG_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiog_sidr;
