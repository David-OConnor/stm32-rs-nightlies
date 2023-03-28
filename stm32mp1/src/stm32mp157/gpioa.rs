#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioa_moder: GPIOA_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioa_otyper: GPIOA_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioa_ospeedr: GPIOA_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioa_pupdr: GPIOA_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioa_idr: GPIOA_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioa_odr: GPIOA_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioa_bsrr: GPIOA_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioa_lckr: GPIOA_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioa_afrl: GPIOA_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioa_afrh: GPIOA_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioa_brr: GPIOA_BRR,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioa_hwcfgr10: GPIOA_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioa_hwcfgr9: GPIOA_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioa_hwcfgr8: GPIOA_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioa_hwcfgr7: GPIOA_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioa_hwcfgr6: GPIOA_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioa_hwcfgr5: GPIOA_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioa_hwcfgr4: GPIOA_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioa_hwcfgr3: GPIOA_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioa_hwcfgr2: GPIOA_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioa_hwcfgr1: GPIOA_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioa_hwcfgr0: GPIOA_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioa_verr: GPIOA_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioa_ipidr: GPIOA_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioa_sidr: GPIOA_SIDR,
}
#[doc = "GPIOA_MODER (rw) register accessor: an alias for `Reg<GPIOA_MODER_SPEC>`"]
pub type GPIOA_MODER = crate::Reg<gpioa_moder::GPIOA_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioa_moder;
#[doc = "GPIOA_OTYPER (rw) register accessor: an alias for `Reg<GPIOA_OTYPER_SPEC>`"]
pub type GPIOA_OTYPER = crate::Reg<gpioa_otyper::GPIOA_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioa_otyper;
#[doc = "GPIOA_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOA_OSPEEDR_SPEC>`"]
pub type GPIOA_OSPEEDR = crate::Reg<gpioa_ospeedr::GPIOA_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioa_ospeedr;
#[doc = "GPIOA_PUPDR (rw) register accessor: an alias for `Reg<GPIOA_PUPDR_SPEC>`"]
pub type GPIOA_PUPDR = crate::Reg<gpioa_pupdr::GPIOA_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioa_pupdr;
#[doc = "GPIOA_IDR (r) register accessor: an alias for `Reg<GPIOA_IDR_SPEC>`"]
pub type GPIOA_IDR = crate::Reg<gpioa_idr::GPIOA_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioa_idr;
#[doc = "GPIOA_ODR (rw) register accessor: an alias for `Reg<GPIOA_ODR_SPEC>`"]
pub type GPIOA_ODR = crate::Reg<gpioa_odr::GPIOA_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioa_odr;
#[doc = "GPIOA_BSRR (w) register accessor: an alias for `Reg<GPIOA_BSRR_SPEC>`"]
pub type GPIOA_BSRR = crate::Reg<gpioa_bsrr::GPIOA_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioa_bsrr;
#[doc = "GPIOA_LCKR (rw) register accessor: an alias for `Reg<GPIOA_LCKR_SPEC>`"]
pub type GPIOA_LCKR = crate::Reg<gpioa_lckr::GPIOA_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioa_lckr;
#[doc = "GPIOA_AFRL (rw) register accessor: an alias for `Reg<GPIOA_AFRL_SPEC>`"]
pub type GPIOA_AFRL = crate::Reg<gpioa_afrl::GPIOA_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioa_afrl;
#[doc = "GPIOA_AFRH (rw) register accessor: an alias for `Reg<GPIOA_AFRH_SPEC>`"]
pub type GPIOA_AFRH = crate::Reg<gpioa_afrh::GPIOA_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioa_afrh;
#[doc = "GPIOA_BRR (w) register accessor: an alias for `Reg<GPIOA_BRR_SPEC>`"]
pub type GPIOA_BRR = crate::Reg<gpioa_brr::GPIOA_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioa_brr;
#[doc = "GPIOA_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR10_SPEC>`"]
pub type GPIOA_HWCFGR10 = crate::Reg<gpioa_hwcfgr10::GPIOA_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioa_hwcfgr10;
#[doc = "GPIOA_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR9_SPEC>`"]
pub type GPIOA_HWCFGR9 = crate::Reg<gpioa_hwcfgr9::GPIOA_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioa_hwcfgr9;
#[doc = "GPIOA_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR8_SPEC>`"]
pub type GPIOA_HWCFGR8 = crate::Reg<gpioa_hwcfgr8::GPIOA_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioa_hwcfgr8;
#[doc = "GPIOA_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR7_SPEC>`"]
pub type GPIOA_HWCFGR7 = crate::Reg<gpioa_hwcfgr7::GPIOA_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioa_hwcfgr7;
#[doc = "GPIOA_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR6_SPEC>`"]
pub type GPIOA_HWCFGR6 = crate::Reg<gpioa_hwcfgr6::GPIOA_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioa_hwcfgr6;
#[doc = "GPIOA_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR5_SPEC>`"]
pub type GPIOA_HWCFGR5 = crate::Reg<gpioa_hwcfgr5::GPIOA_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioa_hwcfgr5;
#[doc = "GPIOA_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR4_SPEC>`"]
pub type GPIOA_HWCFGR4 = crate::Reg<gpioa_hwcfgr4::GPIOA_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioa_hwcfgr4;
#[doc = "GPIOA_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR3_SPEC>`"]
pub type GPIOA_HWCFGR3 = crate::Reg<gpioa_hwcfgr3::GPIOA_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioa_hwcfgr3;
#[doc = "GPIOA_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR2_SPEC>`"]
pub type GPIOA_HWCFGR2 = crate::Reg<gpioa_hwcfgr2::GPIOA_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioa_hwcfgr2;
#[doc = "GPIOA_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR1_SPEC>`"]
pub type GPIOA_HWCFGR1 = crate::Reg<gpioa_hwcfgr1::GPIOA_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioa_hwcfgr1;
#[doc = "GPIOA_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOA_HWCFGR0_SPEC>`"]
pub type GPIOA_HWCFGR0 = crate::Reg<gpioa_hwcfgr0::GPIOA_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioa_hwcfgr0;
#[doc = "GPIOA_VERR (r) register accessor: an alias for `Reg<GPIOA_VERR_SPEC>`"]
pub type GPIOA_VERR = crate::Reg<gpioa_verr::GPIOA_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioa_verr;
#[doc = "GPIOA_IPIDR (r) register accessor: an alias for `Reg<GPIOA_IPIDR_SPEC>`"]
pub type GPIOA_IPIDR = crate::Reg<gpioa_ipidr::GPIOA_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioa_ipidr;
#[doc = "GPIOA_SIDR (r) register accessor: an alias for `Reg<GPIOA_SIDR_SPEC>`"]
pub type GPIOA_SIDR = crate::Reg<gpioa_sidr::GPIOA_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioa_sidr;
