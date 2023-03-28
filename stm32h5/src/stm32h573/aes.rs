#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES control register"]
    pub aes_cr: AES_CR,
    #[doc = "0x04 - AES status register"]
    pub aes_sr: AES_SR,
    #[doc = "0x08 - AES data input register"]
    pub aes_dinr: AES_DINR,
    #[doc = "0x0c - AES data output register"]
    pub aes_doutr: AES_DOUTR,
    #[doc = "0x10 - AES key register 0"]
    pub aes_keyr0: AES_KEYR0,
    #[doc = "0x14 - AES key register 1"]
    pub aes_keyr1: AES_KEYR1,
    #[doc = "0x18 - AES key register 2"]
    pub aes_keyr2: AES_KEYR2,
    #[doc = "0x1c - AES key register 3"]
    pub aes_keyr3: AES_KEYR3,
    #[doc = "0x20 - AES initialization vector register 0"]
    pub aes_ivr0: AES_IVR0,
    #[doc = "0x24 - AES initialization vector register 1"]
    pub aes_ivr1: AES_IVR1,
    #[doc = "0x28 - AES initialization vector register 2"]
    pub aes_ivr2: AES_IVR2,
    #[doc = "0x2c - AES initialization vector register 3"]
    pub aes_ivr3: AES_IVR3,
    #[doc = "0x30 - AES key register 4"]
    pub aes_keyr4: AES_KEYR4,
    #[doc = "0x34 - AES key register 5"]
    pub aes_keyr5: AES_KEYR5,
    #[doc = "0x38 - AES key register 6"]
    pub aes_keyr6: AES_KEYR6,
    #[doc = "0x3c - AES key register 7"]
    pub aes_keyr7: AES_KEYR7,
    #[doc = "0x40 - AES suspend registers"]
    pub aes_susp0r: AES_SUSP0R,
    #[doc = "0x44 - AES suspend registers"]
    pub aes_susp1r: AES_SUSP1R,
    #[doc = "0x48 - AES suspend registers"]
    pub aes_susp2r: AES_SUSP2R,
    #[doc = "0x4c - AES suspend registers"]
    pub aes_susp3r: AES_SUSP3R,
    #[doc = "0x50 - AES suspend registers"]
    pub aes_susp4r: AES_SUSP4R,
    #[doc = "0x54 - AES suspend registers"]
    pub aes_susp5r: AES_SUSP5R,
    #[doc = "0x58 - AES suspend registers"]
    pub aes_susp6r: AES_SUSP6R,
    #[doc = "0x5c - AES suspend registers"]
    pub aes_susp7r: AES_SUSP7R,
    _reserved24: [u8; 0x02a0],
    #[doc = "0x300 - AES interrupt enable register"]
    pub aes_ier: AES_IER,
    #[doc = "0x304 - AES interrupt status register"]
    pub aes_isr: AES_ISR,
    #[doc = "0x308 - AES interrupt clear register"]
    pub aes_icr: AES_ICR,
}
#[doc = "AES_CR (rw) register accessor: an alias for `Reg<AES_CR_SPEC>`"]
pub type AES_CR = crate::Reg<aes_cr::AES_CR_SPEC>;
#[doc = "AES control register"]
pub mod aes_cr;
#[doc = "AES_SR (r) register accessor: an alias for `Reg<AES_SR_SPEC>`"]
pub type AES_SR = crate::Reg<aes_sr::AES_SR_SPEC>;
#[doc = "AES status register"]
pub mod aes_sr;
#[doc = "AES_DINR (w) register accessor: an alias for `Reg<AES_DINR_SPEC>`"]
pub type AES_DINR = crate::Reg<aes_dinr::AES_DINR_SPEC>;
#[doc = "AES data input register"]
pub mod aes_dinr;
#[doc = "AES_DOUTR (r) register accessor: an alias for `Reg<AES_DOUTR_SPEC>`"]
pub type AES_DOUTR = crate::Reg<aes_doutr::AES_DOUTR_SPEC>;
#[doc = "AES data output register"]
pub mod aes_doutr;
#[doc = "AES_KEYR0 (w) register accessor: an alias for `Reg<AES_KEYR0_SPEC>`"]
pub type AES_KEYR0 = crate::Reg<aes_keyr0::AES_KEYR0_SPEC>;
#[doc = "AES key register 0"]
pub mod aes_keyr0;
#[doc = "AES_KEYR1 (w) register accessor: an alias for `Reg<AES_KEYR1_SPEC>`"]
pub type AES_KEYR1 = crate::Reg<aes_keyr1::AES_KEYR1_SPEC>;
#[doc = "AES key register 1"]
pub mod aes_keyr1;
#[doc = "AES_KEYR2 (w) register accessor: an alias for `Reg<AES_KEYR2_SPEC>`"]
pub type AES_KEYR2 = crate::Reg<aes_keyr2::AES_KEYR2_SPEC>;
#[doc = "AES key register 2"]
pub mod aes_keyr2;
#[doc = "AES_KEYR3 (w) register accessor: an alias for `Reg<AES_KEYR3_SPEC>`"]
pub type AES_KEYR3 = crate::Reg<aes_keyr3::AES_KEYR3_SPEC>;
#[doc = "AES key register 3"]
pub mod aes_keyr3;
#[doc = "AES_IVR0 (rw) register accessor: an alias for `Reg<AES_IVR0_SPEC>`"]
pub type AES_IVR0 = crate::Reg<aes_ivr0::AES_IVR0_SPEC>;
#[doc = "AES initialization vector register 0"]
pub mod aes_ivr0;
#[doc = "AES_IVR1 (rw) register accessor: an alias for `Reg<AES_IVR1_SPEC>`"]
pub type AES_IVR1 = crate::Reg<aes_ivr1::AES_IVR1_SPEC>;
#[doc = "AES initialization vector register 1"]
pub mod aes_ivr1;
#[doc = "AES_IVR2 (rw) register accessor: an alias for `Reg<AES_IVR2_SPEC>`"]
pub type AES_IVR2 = crate::Reg<aes_ivr2::AES_IVR2_SPEC>;
#[doc = "AES initialization vector register 2"]
pub mod aes_ivr2;
#[doc = "AES_IVR3 (rw) register accessor: an alias for `Reg<AES_IVR3_SPEC>`"]
pub type AES_IVR3 = crate::Reg<aes_ivr3::AES_IVR3_SPEC>;
#[doc = "AES initialization vector register 3"]
pub mod aes_ivr3;
#[doc = "AES_KEYR4 (w) register accessor: an alias for `Reg<AES_KEYR4_SPEC>`"]
pub type AES_KEYR4 = crate::Reg<aes_keyr4::AES_KEYR4_SPEC>;
#[doc = "AES key register 4"]
pub mod aes_keyr4;
#[doc = "AES_KEYR5 (w) register accessor: an alias for `Reg<AES_KEYR5_SPEC>`"]
pub type AES_KEYR5 = crate::Reg<aes_keyr5::AES_KEYR5_SPEC>;
#[doc = "AES key register 5"]
pub mod aes_keyr5;
#[doc = "AES_KEYR6 (w) register accessor: an alias for `Reg<AES_KEYR6_SPEC>`"]
pub type AES_KEYR6 = crate::Reg<aes_keyr6::AES_KEYR6_SPEC>;
#[doc = "AES key register 6"]
pub mod aes_keyr6;
#[doc = "AES_KEYR7 (w) register accessor: an alias for `Reg<AES_KEYR7_SPEC>`"]
pub type AES_KEYR7 = crate::Reg<aes_keyr7::AES_KEYR7_SPEC>;
#[doc = "AES key register 7"]
pub mod aes_keyr7;
#[doc = "AES_SUSP0R (rw) register accessor: an alias for `Reg<AES_SUSP0R_SPEC>`"]
pub type AES_SUSP0R = crate::Reg<aes_susp0r::AES_SUSP0R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp0r;
#[doc = "AES_SUSP1R (rw) register accessor: an alias for `Reg<AES_SUSP1R_SPEC>`"]
pub type AES_SUSP1R = crate::Reg<aes_susp1r::AES_SUSP1R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp1r;
#[doc = "AES_SUSP2R (rw) register accessor: an alias for `Reg<AES_SUSP2R_SPEC>`"]
pub type AES_SUSP2R = crate::Reg<aes_susp2r::AES_SUSP2R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp2r;
#[doc = "AES_SUSP3R (rw) register accessor: an alias for `Reg<AES_SUSP3R_SPEC>`"]
pub type AES_SUSP3R = crate::Reg<aes_susp3r::AES_SUSP3R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp3r;
#[doc = "AES_SUSP4R (rw) register accessor: an alias for `Reg<AES_SUSP4R_SPEC>`"]
pub type AES_SUSP4R = crate::Reg<aes_susp4r::AES_SUSP4R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp4r;
#[doc = "AES_SUSP5R (rw) register accessor: an alias for `Reg<AES_SUSP5R_SPEC>`"]
pub type AES_SUSP5R = crate::Reg<aes_susp5r::AES_SUSP5R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp5r;
#[doc = "AES_SUSP6R (rw) register accessor: an alias for `Reg<AES_SUSP6R_SPEC>`"]
pub type AES_SUSP6R = crate::Reg<aes_susp6r::AES_SUSP6R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp6r;
#[doc = "AES_SUSP7R (rw) register accessor: an alias for `Reg<AES_SUSP7R_SPEC>`"]
pub type AES_SUSP7R = crate::Reg<aes_susp7r::AES_SUSP7R_SPEC>;
#[doc = "AES suspend registers"]
pub mod aes_susp7r;
#[doc = "AES_IER (rw) register accessor: an alias for `Reg<AES_IER_SPEC>`"]
pub type AES_IER = crate::Reg<aes_ier::AES_IER_SPEC>;
#[doc = "AES interrupt enable register"]
pub mod aes_ier;
#[doc = "AES_ISR (r) register accessor: an alias for `Reg<AES_ISR_SPEC>`"]
pub type AES_ISR = crate::Reg<aes_isr::AES_ISR_SPEC>;
#[doc = "AES interrupt status register"]
pub mod aes_isr;
#[doc = "AES_ICR (w) register accessor: an alias for `Reg<AES_ICR_SPEC>`"]
pub type AES_ICR = crate::Reg<aes_icr::AES_ICR_SPEC>;
#[doc = "AES interrupt clear register"]
pub mod aes_icr;
