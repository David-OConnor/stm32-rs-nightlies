#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAES control register"]
    pub saes_cr: SAES_CR,
    #[doc = "0x04 - SAES status register"]
    pub saes_sr: SAES_SR,
    #[doc = "0x08 - SAES data input register"]
    pub saes_dinr: SAES_DINR,
    #[doc = "0x0c - SAES data output register"]
    pub saes_doutr: SAES_DOUTR,
    #[doc = "0x10 - SAES key register 0"]
    pub saes_keyr0: SAES_KEYR0,
    #[doc = "0x14 - SAES key register 1"]
    pub saes_keyr1: SAES_KEYR1,
    #[doc = "0x18 - SAES key register 2"]
    pub saes_keyr2: SAES_KEYR2,
    #[doc = "0x1c - SAES key register 3"]
    pub saes_keyr3: SAES_KEYR3,
    #[doc = "0x20 - SAES initialization vector register 0"]
    pub saes_ivr0: SAES_IVR0,
    #[doc = "0x24 - SAES initialization vector register 1"]
    pub saes_ivr1: SAES_IVR1,
    #[doc = "0x28 - SAES initialization vector register 2"]
    pub saes_ivr2: SAES_IVR2,
    #[doc = "0x2c - SAES initialization vector register 3"]
    pub saes_ivr3: SAES_IVR3,
    #[doc = "0x30 - SAES key register 4"]
    pub saes_keyr4: SAES_KEYR4,
    #[doc = "0x34 - SAES key register 5"]
    pub saes_keyr5: SAES_KEYR5,
    #[doc = "0x38 - SAES key register 6"]
    pub saes_keyr6: SAES_KEYR6,
    #[doc = "0x3c - SAES key register 7"]
    pub saes_keyr7: SAES_KEYR7,
    #[doc = "0x40 - SAES suspend registers"]
    pub saes_susp0r: SAES_SUSP0R,
    #[doc = "0x44 - SAES suspend registers"]
    pub saes_susp1r: SAES_SUSP1R,
    #[doc = "0x48 - SAES suspend registers"]
    pub saes_susp2r: SAES_SUSP2R,
    #[doc = "0x4c - SAES suspend registers"]
    pub saes_susp3r: SAES_SUSP3R,
    #[doc = "0x50 - SAES suspend registers"]
    pub saes_susp4r: SAES_SUSP4R,
    #[doc = "0x54 - SAES suspend registers"]
    pub saes_susp5r: SAES_SUSP5R,
    #[doc = "0x58 - SAES suspend registers"]
    pub saes_susp6r: SAES_SUSP6R,
    #[doc = "0x5c - SAES suspend registers"]
    pub saes_susp7r: SAES_SUSP7R,
    _reserved24: [u8; 0x02a0],
    #[doc = "0x300 - SAES interrupt enable register"]
    pub saes_ier: SAES_IER,
    #[doc = "0x304 - SAES interrupt status register"]
    pub saes_isr: SAES_ISR,
    #[doc = "0x308 - SAES interrupt clear register"]
    pub saes_icr: SAES_ICR,
}
#[doc = "SAES_CR (rw) register accessor: an alias for `Reg<SAES_CR_SPEC>`"]
pub type SAES_CR = crate::Reg<saes_cr::SAES_CR_SPEC>;
#[doc = "SAES control register"]
pub mod saes_cr;
#[doc = "SAES_SR (r) register accessor: an alias for `Reg<SAES_SR_SPEC>`"]
pub type SAES_SR = crate::Reg<saes_sr::SAES_SR_SPEC>;
#[doc = "SAES status register"]
pub mod saes_sr;
#[doc = "SAES_DINR (w) register accessor: an alias for `Reg<SAES_DINR_SPEC>`"]
pub type SAES_DINR = crate::Reg<saes_dinr::SAES_DINR_SPEC>;
#[doc = "SAES data input register"]
pub mod saes_dinr;
#[doc = "SAES_DOUTR (r) register accessor: an alias for `Reg<SAES_DOUTR_SPEC>`"]
pub type SAES_DOUTR = crate::Reg<saes_doutr::SAES_DOUTR_SPEC>;
#[doc = "SAES data output register"]
pub mod saes_doutr;
#[doc = "SAES_KEYR0 (w) register accessor: an alias for `Reg<SAES_KEYR0_SPEC>`"]
pub type SAES_KEYR0 = crate::Reg<saes_keyr0::SAES_KEYR0_SPEC>;
#[doc = "SAES key register 0"]
pub mod saes_keyr0;
#[doc = "SAES_KEYR1 (w) register accessor: an alias for `Reg<SAES_KEYR1_SPEC>`"]
pub type SAES_KEYR1 = crate::Reg<saes_keyr1::SAES_KEYR1_SPEC>;
#[doc = "SAES key register 1"]
pub mod saes_keyr1;
#[doc = "SAES_KEYR2 (w) register accessor: an alias for `Reg<SAES_KEYR2_SPEC>`"]
pub type SAES_KEYR2 = crate::Reg<saes_keyr2::SAES_KEYR2_SPEC>;
#[doc = "SAES key register 2"]
pub mod saes_keyr2;
#[doc = "SAES_KEYR3 (w) register accessor: an alias for `Reg<SAES_KEYR3_SPEC>`"]
pub type SAES_KEYR3 = crate::Reg<saes_keyr3::SAES_KEYR3_SPEC>;
#[doc = "SAES key register 3"]
pub mod saes_keyr3;
#[doc = "SAES_IVR0 (rw) register accessor: an alias for `Reg<SAES_IVR0_SPEC>`"]
pub type SAES_IVR0 = crate::Reg<saes_ivr0::SAES_IVR0_SPEC>;
#[doc = "SAES initialization vector register 0"]
pub mod saes_ivr0;
#[doc = "SAES_IVR1 (rw) register accessor: an alias for `Reg<SAES_IVR1_SPEC>`"]
pub type SAES_IVR1 = crate::Reg<saes_ivr1::SAES_IVR1_SPEC>;
#[doc = "SAES initialization vector register 1"]
pub mod saes_ivr1;
#[doc = "SAES_IVR2 (rw) register accessor: an alias for `Reg<SAES_IVR2_SPEC>`"]
pub type SAES_IVR2 = crate::Reg<saes_ivr2::SAES_IVR2_SPEC>;
#[doc = "SAES initialization vector register 2"]
pub mod saes_ivr2;
#[doc = "SAES_IVR3 (rw) register accessor: an alias for `Reg<SAES_IVR3_SPEC>`"]
pub type SAES_IVR3 = crate::Reg<saes_ivr3::SAES_IVR3_SPEC>;
#[doc = "SAES initialization vector register 3"]
pub mod saes_ivr3;
#[doc = "SAES_KEYR4 (w) register accessor: an alias for `Reg<SAES_KEYR4_SPEC>`"]
pub type SAES_KEYR4 = crate::Reg<saes_keyr4::SAES_KEYR4_SPEC>;
#[doc = "SAES key register 4"]
pub mod saes_keyr4;
#[doc = "SAES_KEYR5 (w) register accessor: an alias for `Reg<SAES_KEYR5_SPEC>`"]
pub type SAES_KEYR5 = crate::Reg<saes_keyr5::SAES_KEYR5_SPEC>;
#[doc = "SAES key register 5"]
pub mod saes_keyr5;
#[doc = "SAES_KEYR6 (w) register accessor: an alias for `Reg<SAES_KEYR6_SPEC>`"]
pub type SAES_KEYR6 = crate::Reg<saes_keyr6::SAES_KEYR6_SPEC>;
#[doc = "SAES key register 6"]
pub mod saes_keyr6;
#[doc = "SAES_KEYR7 (w) register accessor: an alias for `Reg<SAES_KEYR7_SPEC>`"]
pub type SAES_KEYR7 = crate::Reg<saes_keyr7::SAES_KEYR7_SPEC>;
#[doc = "SAES key register 7"]
pub mod saes_keyr7;
#[doc = "SAES_SUSP0R (rw) register accessor: an alias for `Reg<SAES_SUSP0R_SPEC>`"]
pub type SAES_SUSP0R = crate::Reg<saes_susp0r::SAES_SUSP0R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp0r;
#[doc = "SAES_SUSP1R (rw) register accessor: an alias for `Reg<SAES_SUSP1R_SPEC>`"]
pub type SAES_SUSP1R = crate::Reg<saes_susp1r::SAES_SUSP1R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp1r;
#[doc = "SAES_SUSP2R (rw) register accessor: an alias for `Reg<SAES_SUSP2R_SPEC>`"]
pub type SAES_SUSP2R = crate::Reg<saes_susp2r::SAES_SUSP2R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp2r;
#[doc = "SAES_SUSP3R (rw) register accessor: an alias for `Reg<SAES_SUSP3R_SPEC>`"]
pub type SAES_SUSP3R = crate::Reg<saes_susp3r::SAES_SUSP3R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp3r;
#[doc = "SAES_SUSP4R (rw) register accessor: an alias for `Reg<SAES_SUSP4R_SPEC>`"]
pub type SAES_SUSP4R = crate::Reg<saes_susp4r::SAES_SUSP4R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp4r;
#[doc = "SAES_SUSP5R (rw) register accessor: an alias for `Reg<SAES_SUSP5R_SPEC>`"]
pub type SAES_SUSP5R = crate::Reg<saes_susp5r::SAES_SUSP5R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp5r;
#[doc = "SAES_SUSP6R (rw) register accessor: an alias for `Reg<SAES_SUSP6R_SPEC>`"]
pub type SAES_SUSP6R = crate::Reg<saes_susp6r::SAES_SUSP6R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp6r;
#[doc = "SAES_SUSP7R (rw) register accessor: an alias for `Reg<SAES_SUSP7R_SPEC>`"]
pub type SAES_SUSP7R = crate::Reg<saes_susp7r::SAES_SUSP7R_SPEC>;
#[doc = "SAES suspend registers"]
pub mod saes_susp7r;
#[doc = "SAES_IER (rw) register accessor: an alias for `Reg<SAES_IER_SPEC>`"]
pub type SAES_IER = crate::Reg<saes_ier::SAES_IER_SPEC>;
#[doc = "SAES interrupt enable register"]
pub mod saes_ier;
#[doc = "SAES_ISR (r) register accessor: an alias for `Reg<SAES_ISR_SPEC>`"]
pub type SAES_ISR = crate::Reg<saes_isr::SAES_ISR_SPEC>;
#[doc = "SAES interrupt status register"]
pub mod saes_isr;
#[doc = "SAES_ICR (w) register accessor: an alias for `Reg<SAES_ICR_SPEC>`"]
pub type SAES_ICR = crate::Reg<saes_icr::SAES_ICR_SPEC>;
#[doc = "SAES interrupt clear register"]
pub mod saes_icr;
