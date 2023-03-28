#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioi_moder: GPIOI_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioi_otyper: GPIOI_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioi_ospeedr: GPIOI_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioi_pupdr: GPIOI_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioi_idr: GPIOI_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioi_odr: GPIOI_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioi_bsrr: GPIOI_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioi_lckr: GPIOI_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioi_afrl: GPIOI_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioi_afrh: GPIOI_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioi_brr: GPIOI_BRR,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioi_hwcfgr10: GPIOI_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioi_hwcfgr9: GPIOI_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioi_hwcfgr8: GPIOI_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioi_hwcfgr7: GPIOI_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioi_hwcfgr6: GPIOI_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioi_hwcfgr5: GPIOI_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioi_hwcfgr4: GPIOI_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioi_hwcfgr3: GPIOI_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioi_hwcfgr2: GPIOI_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioi_hwcfgr1: GPIOI_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioi_hwcfgr0: GPIOI_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioi_verr: GPIOI_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioi_ipidr: GPIOI_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioi_sidr: GPIOI_SIDR,
}
#[doc = "GPIOI_MODER (rw) register accessor: an alias for `Reg<GPIOI_MODER_SPEC>`"]
pub type GPIOI_MODER = crate::Reg<gpioi_moder::GPIOI_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioi_moder;
#[doc = "GPIOI_OTYPER (rw) register accessor: an alias for `Reg<GPIOI_OTYPER_SPEC>`"]
pub type GPIOI_OTYPER = crate::Reg<gpioi_otyper::GPIOI_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioi_otyper;
#[doc = "GPIOI_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOI_OSPEEDR_SPEC>`"]
pub type GPIOI_OSPEEDR = crate::Reg<gpioi_ospeedr::GPIOI_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioi_ospeedr;
#[doc = "GPIOI_PUPDR (rw) register accessor: an alias for `Reg<GPIOI_PUPDR_SPEC>`"]
pub type GPIOI_PUPDR = crate::Reg<gpioi_pupdr::GPIOI_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioi_pupdr;
#[doc = "GPIOI_IDR (r) register accessor: an alias for `Reg<GPIOI_IDR_SPEC>`"]
pub type GPIOI_IDR = crate::Reg<gpioi_idr::GPIOI_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioi_idr;
#[doc = "GPIOI_ODR (rw) register accessor: an alias for `Reg<GPIOI_ODR_SPEC>`"]
pub type GPIOI_ODR = crate::Reg<gpioi_odr::GPIOI_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioi_odr;
#[doc = "GPIOI_BSRR (w) register accessor: an alias for `Reg<GPIOI_BSRR_SPEC>`"]
pub type GPIOI_BSRR = crate::Reg<gpioi_bsrr::GPIOI_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioi_bsrr;
#[doc = "GPIOI_LCKR (rw) register accessor: an alias for `Reg<GPIOI_LCKR_SPEC>`"]
pub type GPIOI_LCKR = crate::Reg<gpioi_lckr::GPIOI_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioi_lckr;
#[doc = "GPIOI_AFRL (rw) register accessor: an alias for `Reg<GPIOI_AFRL_SPEC>`"]
pub type GPIOI_AFRL = crate::Reg<gpioi_afrl::GPIOI_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioi_afrl;
#[doc = "GPIOI_AFRH (rw) register accessor: an alias for `Reg<GPIOI_AFRH_SPEC>`"]
pub type GPIOI_AFRH = crate::Reg<gpioi_afrh::GPIOI_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioi_afrh;
#[doc = "GPIOI_BRR (w) register accessor: an alias for `Reg<GPIOI_BRR_SPEC>`"]
pub type GPIOI_BRR = crate::Reg<gpioi_brr::GPIOI_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioi_brr;
#[doc = "GPIOI_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR10_SPEC>`"]
pub type GPIOI_HWCFGR10 = crate::Reg<gpioi_hwcfgr10::GPIOI_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioi_hwcfgr10;
#[doc = "GPIOI_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR9_SPEC>`"]
pub type GPIOI_HWCFGR9 = crate::Reg<gpioi_hwcfgr9::GPIOI_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioi_hwcfgr9;
#[doc = "GPIOI_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR8_SPEC>`"]
pub type GPIOI_HWCFGR8 = crate::Reg<gpioi_hwcfgr8::GPIOI_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioi_hwcfgr8;
#[doc = "GPIOI_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR7_SPEC>`"]
pub type GPIOI_HWCFGR7 = crate::Reg<gpioi_hwcfgr7::GPIOI_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioi_hwcfgr7;
#[doc = "GPIOI_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR6_SPEC>`"]
pub type GPIOI_HWCFGR6 = crate::Reg<gpioi_hwcfgr6::GPIOI_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioi_hwcfgr6;
#[doc = "GPIOI_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR5_SPEC>`"]
pub type GPIOI_HWCFGR5 = crate::Reg<gpioi_hwcfgr5::GPIOI_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioi_hwcfgr5;
#[doc = "GPIOI_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR4_SPEC>`"]
pub type GPIOI_HWCFGR4 = crate::Reg<gpioi_hwcfgr4::GPIOI_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioi_hwcfgr4;
#[doc = "GPIOI_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR3_SPEC>`"]
pub type GPIOI_HWCFGR3 = crate::Reg<gpioi_hwcfgr3::GPIOI_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioi_hwcfgr3;
#[doc = "GPIOI_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR2_SPEC>`"]
pub type GPIOI_HWCFGR2 = crate::Reg<gpioi_hwcfgr2::GPIOI_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioi_hwcfgr2;
#[doc = "GPIOI_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR1_SPEC>`"]
pub type GPIOI_HWCFGR1 = crate::Reg<gpioi_hwcfgr1::GPIOI_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioi_hwcfgr1;
#[doc = "GPIOI_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOI_HWCFGR0_SPEC>`"]
pub type GPIOI_HWCFGR0 = crate::Reg<gpioi_hwcfgr0::GPIOI_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioi_hwcfgr0;
#[doc = "GPIOI_VERR (r) register accessor: an alias for `Reg<GPIOI_VERR_SPEC>`"]
pub type GPIOI_VERR = crate::Reg<gpioi_verr::GPIOI_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioi_verr;
#[doc = "GPIOI_IPIDR (r) register accessor: an alias for `Reg<GPIOI_IPIDR_SPEC>`"]
pub type GPIOI_IPIDR = crate::Reg<gpioi_ipidr::GPIOI_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioi_ipidr;
#[doc = "GPIOI_SIDR (r) register accessor: an alias for `Reg<GPIOI_SIDR_SPEC>`"]
pub type GPIOI_SIDR = crate::Reg<gpioi_sidr::GPIOI_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioi_sidr;
