#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RAMCFG memory 1 control register"]
    pub ramcfg_m1cr: RAMCFG_M1CR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - RAMCFG memory interrupt status register"]
    pub ramcfg_m1isr: RAMCFG_M1ISR,
    _reserved2: [u8; 0x1c],
    #[doc = "0x28 - RAMCFG memory 1 erase key register"]
    pub ramcfg_m1erkeyr: RAMCFG_M1ERKEYR,
    _reserved3: [u8; 0x14],
    #[doc = "0x40 - RAMCFG memory 2 control register"]
    pub ramcfg_m2cr: RAMCFG_M2CR,
    #[doc = "0x44 - RAMCFG memory 2 interrupt enable register"]
    pub ramcfg_m2ier: RAMCFG_M2IER,
    #[doc = "0x48 - RAMCFG memory interrupt status register"]
    pub ramcfg_m2isr: RAMCFG_M2ISR,
    #[doc = "0x4c - RAMCFG memory 2 ECC single error address register"]
    pub ramcfg_m2sear: RAMCFG_M2SEAR,
    #[doc = "0x50 - RAMCFG memory 2 ECC double error address register"]
    pub ramcfg_m2dear: RAMCFG_M2DEAR,
    #[doc = "0x54 - RAMCFG memory 2 interrupt clear register 2"]
    pub ramcfg_m2icr: RAMCFG_M2ICR,
    #[doc = "0x58 - RAMCFG memory 2 write protection register 1"]
    pub ramcfg_m2wpr1: RAMCFG_M2WPR1,
    _reserved10: [u8; 0x08],
    #[doc = "0x64 - RAMCFG memory 2 ECC key register"]
    pub ramcfg_m2ecckeyr: RAMCFG_M2ECCKEYR,
    #[doc = "0x68 - RAMCFG memory 2 erase key register"]
    pub ramcfg_m2erkeyr: RAMCFG_M2ERKEYR,
    _reserved12: [u8; 0x18],
    #[doc = "0x84 - RAMCFG memory 3 interrupt enable register"]
    pub ramcfg_m3ier: RAMCFG_M3IER,
    #[doc = "0x88 - RAMCFG memory interrupt status register"]
    pub ramcfg_m3isr: RAMCFG_M3ISR,
    #[doc = "0x8c - RAMCFG memory 3 ECC single error address register"]
    pub ramcfg_m3sear: RAMCFG_M3SEAR,
    #[doc = "0x90 - RAMCFG memory 3 ECC double error address register"]
    pub ramcfg_m3dear: RAMCFG_M3DEAR,
    #[doc = "0x94 - RAMCFG memory 3 interrupt clear register 3"]
    pub ramcfg_m3icr: RAMCFG_M3ICR,
    _reserved17: [u8; 0x0c],
    #[doc = "0xa4 - RAMCFG memory 3 ECC key register"]
    pub ramcfg_m3ecckeyr: RAMCFG_M3ECCKEYR,
    #[doc = "0xa8 - RAMCFG memory 3 erase key register"]
    pub ramcfg_m3erkeyr: RAMCFG_M3ERKEYR,
    _reserved19: [u8; 0x3c],
    #[doc = "0xe8 - RAMCFG memory 4 erase key register"]
    pub ramcfg_m4erkeyr: RAMCFG_M4ERKEYR,
    _reserved20: [u8; 0x14],
    #[doc = "0x100 - RAMCFG memory 5 control register"]
    pub ramcfg_m5cr: RAMCFG_M5CR,
    #[doc = "0x104 - RAMCFG memory 5 interrupt enable register"]
    pub ramcfg_m5ier: RAMCFG_M5IER,
    #[doc = "0x108 - RAMCFG memory interrupt status register"]
    pub ramcfg_m5isr: RAMCFG_M5ISR,
    #[doc = "0x10c - RAMCFG memory 5 ECC single error address register"]
    pub ramcfg_m5sear: RAMCFG_M5SEAR,
    #[doc = "0x110 - RAMCFG memory 5 ECC double error address register"]
    pub ramcfg_m5dear: RAMCFG_M5DEAR,
    #[doc = "0x114 - RAMCFG memory 5 interrupt clear register 5"]
    pub ramcfg_m5icr: RAMCFG_M5ICR,
    _reserved26: [u8; 0x0c],
    #[doc = "0x124 - RAMCFG memory 5 ECC key register"]
    pub ramcfg_m5ecckeyr: RAMCFG_M5ECCKEYR,
    #[doc = "0x128 - RAMCFG memory 5 erase key register"]
    pub ramcfg_m5erkeyr: RAMCFG_M5ERKEYR,
}
#[doc = "RAMCFG_M1CR (rw) register accessor: an alias for `Reg<RAMCFG_M1CR_SPEC>`"]
pub type RAMCFG_M1CR = crate::Reg<ramcfg_m1cr::RAMCFG_M1CR_SPEC>;
#[doc = "RAMCFG memory 1 control register"]
pub mod ramcfg_m1cr;
#[doc = "RAMCFG_M1ISR (r) register accessor: an alias for `Reg<RAMCFG_M1ISR_SPEC>`"]
pub type RAMCFG_M1ISR = crate::Reg<ramcfg_m1isr::RAMCFG_M1ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod ramcfg_m1isr;
#[doc = "RAMCFG_M1ERKEYR (w) register accessor: an alias for `Reg<RAMCFG_M1ERKEYR_SPEC>`"]
pub type RAMCFG_M1ERKEYR = crate::Reg<ramcfg_m1erkeyr::RAMCFG_M1ERKEYR_SPEC>;
#[doc = "RAMCFG memory 1 erase key register"]
pub mod ramcfg_m1erkeyr;
#[doc = "RAMCFG_M2CR (rw) register accessor: an alias for `Reg<RAMCFG_M2CR_SPEC>`"]
pub type RAMCFG_M2CR = crate::Reg<ramcfg_m2cr::RAMCFG_M2CR_SPEC>;
#[doc = "RAMCFG memory 2 control register"]
pub mod ramcfg_m2cr;
#[doc = "RAMCFG_M2IER (rw) register accessor: an alias for `Reg<RAMCFG_M2IER_SPEC>`"]
pub type RAMCFG_M2IER = crate::Reg<ramcfg_m2ier::RAMCFG_M2IER_SPEC>;
#[doc = "RAMCFG memory 2 interrupt enable register"]
pub mod ramcfg_m2ier;
#[doc = "RAMCFG_M2ISR (r) register accessor: an alias for `Reg<RAMCFG_M2ISR_SPEC>`"]
pub type RAMCFG_M2ISR = crate::Reg<ramcfg_m2isr::RAMCFG_M2ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod ramcfg_m2isr;
#[doc = "RAMCFG_M2SEAR (r) register accessor: an alias for `Reg<RAMCFG_M2SEAR_SPEC>`"]
pub type RAMCFG_M2SEAR = crate::Reg<ramcfg_m2sear::RAMCFG_M2SEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC single error address register"]
pub mod ramcfg_m2sear;
#[doc = "RAMCFG_M2DEAR (r) register accessor: an alias for `Reg<RAMCFG_M2DEAR_SPEC>`"]
pub type RAMCFG_M2DEAR = crate::Reg<ramcfg_m2dear::RAMCFG_M2DEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC double error address register"]
pub mod ramcfg_m2dear;
#[doc = "RAMCFG_M2ICR (rw) register accessor: an alias for `Reg<RAMCFG_M2ICR_SPEC>`"]
pub type RAMCFG_M2ICR = crate::Reg<ramcfg_m2icr::RAMCFG_M2ICR_SPEC>;
#[doc = "RAMCFG memory 2 interrupt clear register 2"]
pub mod ramcfg_m2icr;
#[doc = "RAMCFG_M2WPR1 (rw) register accessor: an alias for `Reg<RAMCFG_M2WPR1_SPEC>`"]
pub type RAMCFG_M2WPR1 = crate::Reg<ramcfg_m2wpr1::RAMCFG_M2WPR1_SPEC>;
#[doc = "RAMCFG memory 2 write protection register 1"]
pub mod ramcfg_m2wpr1;
#[doc = "RAMCFG_M2ECCKEYR (w) register accessor: an alias for `Reg<RAMCFG_M2ECCKEYR_SPEC>`"]
pub type RAMCFG_M2ECCKEYR = crate::Reg<ramcfg_m2ecckeyr::RAMCFG_M2ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 2 ECC key register"]
pub mod ramcfg_m2ecckeyr;
#[doc = "RAMCFG_M2ERKEYR (w) register accessor: an alias for `Reg<RAMCFG_M2ERKEYR_SPEC>`"]
pub type RAMCFG_M2ERKEYR = crate::Reg<ramcfg_m2erkeyr::RAMCFG_M2ERKEYR_SPEC>;
#[doc = "RAMCFG memory 2 erase key register"]
pub mod ramcfg_m2erkeyr;
#[doc = "RAMCFG_M3IER (rw) register accessor: an alias for `Reg<RAMCFG_M3IER_SPEC>`"]
pub type RAMCFG_M3IER = crate::Reg<ramcfg_m3ier::RAMCFG_M3IER_SPEC>;
#[doc = "RAMCFG memory 3 interrupt enable register"]
pub mod ramcfg_m3ier;
#[doc = "RAMCFG_M3ISR (r) register accessor: an alias for `Reg<RAMCFG_M3ISR_SPEC>`"]
pub type RAMCFG_M3ISR = crate::Reg<ramcfg_m3isr::RAMCFG_M3ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod ramcfg_m3isr;
#[doc = "RAMCFG_M3SEAR (r) register accessor: an alias for `Reg<RAMCFG_M3SEAR_SPEC>`"]
pub type RAMCFG_M3SEAR = crate::Reg<ramcfg_m3sear::RAMCFG_M3SEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC single error address register"]
pub mod ramcfg_m3sear;
#[doc = "RAMCFG_M3DEAR (r) register accessor: an alias for `Reg<RAMCFG_M3DEAR_SPEC>`"]
pub type RAMCFG_M3DEAR = crate::Reg<ramcfg_m3dear::RAMCFG_M3DEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC double error address register"]
pub mod ramcfg_m3dear;
#[doc = "RAMCFG_M3ICR (rw) register accessor: an alias for `Reg<RAMCFG_M3ICR_SPEC>`"]
pub type RAMCFG_M3ICR = crate::Reg<ramcfg_m3icr::RAMCFG_M3ICR_SPEC>;
#[doc = "RAMCFG memory 3 interrupt clear register 3"]
pub mod ramcfg_m3icr;
#[doc = "RAMCFG_M3ECCKEYR (w) register accessor: an alias for `Reg<RAMCFG_M3ECCKEYR_SPEC>`"]
pub type RAMCFG_M3ECCKEYR = crate::Reg<ramcfg_m3ecckeyr::RAMCFG_M3ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 3 ECC key register"]
pub mod ramcfg_m3ecckeyr;
#[doc = "RAMCFG_M3ERKEYR (w) register accessor: an alias for `Reg<RAMCFG_M3ERKEYR_SPEC>`"]
pub type RAMCFG_M3ERKEYR = crate::Reg<ramcfg_m3erkeyr::RAMCFG_M3ERKEYR_SPEC>;
#[doc = "RAMCFG memory 3 erase key register"]
pub mod ramcfg_m3erkeyr;
#[doc = "RAMCFG_M4ERKEYR (w) register accessor: an alias for `Reg<RAMCFG_M4ERKEYR_SPEC>`"]
pub type RAMCFG_M4ERKEYR = crate::Reg<ramcfg_m4erkeyr::RAMCFG_M4ERKEYR_SPEC>;
#[doc = "RAMCFG memory 4 erase key register"]
pub mod ramcfg_m4erkeyr;
#[doc = "RAMCFG_M5CR (rw) register accessor: an alias for `Reg<RAMCFG_M5CR_SPEC>`"]
pub type RAMCFG_M5CR = crate::Reg<ramcfg_m5cr::RAMCFG_M5CR_SPEC>;
#[doc = "RAMCFG memory 5 control register"]
pub mod ramcfg_m5cr;
#[doc = "RAMCFG_M5IER (rw) register accessor: an alias for `Reg<RAMCFG_M5IER_SPEC>`"]
pub type RAMCFG_M5IER = crate::Reg<ramcfg_m5ier::RAMCFG_M5IER_SPEC>;
#[doc = "RAMCFG memory 5 interrupt enable register"]
pub mod ramcfg_m5ier;
#[doc = "RAMCFG_M5ISR (r) register accessor: an alias for `Reg<RAMCFG_M5ISR_SPEC>`"]
pub type RAMCFG_M5ISR = crate::Reg<ramcfg_m5isr::RAMCFG_M5ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod ramcfg_m5isr;
#[doc = "RAMCFG_M5SEAR (r) register accessor: an alias for `Reg<RAMCFG_M5SEAR_SPEC>`"]
pub type RAMCFG_M5SEAR = crate::Reg<ramcfg_m5sear::RAMCFG_M5SEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC single error address register"]
pub mod ramcfg_m5sear;
#[doc = "RAMCFG_M5DEAR (r) register accessor: an alias for `Reg<RAMCFG_M5DEAR_SPEC>`"]
pub type RAMCFG_M5DEAR = crate::Reg<ramcfg_m5dear::RAMCFG_M5DEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC double error address register"]
pub mod ramcfg_m5dear;
#[doc = "RAMCFG_M5ICR (rw) register accessor: an alias for `Reg<RAMCFG_M5ICR_SPEC>`"]
pub type RAMCFG_M5ICR = crate::Reg<ramcfg_m5icr::RAMCFG_M5ICR_SPEC>;
#[doc = "RAMCFG memory 5 interrupt clear register 5"]
pub mod ramcfg_m5icr;
#[doc = "RAMCFG_M5ECCKEYR (w) register accessor: an alias for `Reg<RAMCFG_M5ECCKEYR_SPEC>`"]
pub type RAMCFG_M5ECCKEYR = crate::Reg<ramcfg_m5ecckeyr::RAMCFG_M5ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 5 ECC key register"]
pub mod ramcfg_m5ecckeyr;
#[doc = "RAMCFG_M5ERKEYR (w) register accessor: an alias for `Reg<RAMCFG_M5ERKEYR_SPEC>`"]
pub type RAMCFG_M5ERKEYR = crate::Reg<ramcfg_m5erkeyr::RAMCFG_M5ERKEYR_SPEC>;
#[doc = "RAMCFG memory 5 erase key register"]
pub mod ramcfg_m5erkeyr;
