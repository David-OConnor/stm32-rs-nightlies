#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES control register"]
    pub cr: CR,
    #[doc = "0x04 - AES status register"]
    pub sr: SR,
    #[doc = "0x08 - AES data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - AES data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - AES key register 0"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - AES key register 1"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - AES key register 2"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - AES key register 3"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - AES initialization vector register 0"]
    pub ivr0: IVR0,
    #[doc = "0x24 - AES initialization vector register 1"]
    pub ivr1: IVR1,
    #[doc = "0x28 - AES initialization vector register 2"]
    pub ivr2: IVR2,
    #[doc = "0x2c - AES initialization vector register 3"]
    pub ivr3: IVR3,
    #[doc = "0x30 - AES key register 4"]
    pub keyr4: KEYR4,
    #[doc = "0x34 - AES key register 5"]
    pub keyr5: KEYR5,
    #[doc = "0x38 - AES key register 6"]
    pub keyr6: KEYR6,
    #[doc = "0x3c - AES key register 7"]
    pub keyr7: KEYR7,
    #[doc = "0x40 - AES suspend registers"]
    pub susp0r: SUSP0R,
    #[doc = "0x44 - AES suspend registers"]
    pub susp1r: SUSP1R,
    #[doc = "0x48 - AES suspend registers"]
    pub susp2r: SUSP2R,
    #[doc = "0x4c - AES suspend registers"]
    pub susp3r: SUSP3R,
    #[doc = "0x50 - AES suspend registers"]
    pub susp4r: SUSP4R,
    #[doc = "0x54 - AES suspend registers"]
    pub susp5r: SUSP5R,
    #[doc = "0x58 - AES suspend registers"]
    pub susp6r: SUSP6R,
    #[doc = "0x5c - AES suspend registers"]
    pub susp7r: SUSP7R,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "AES control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "AES status register"]
pub mod sr;
#[doc = "DINR (rw) register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "AES data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "AES data output register"]
pub mod doutr;
#[doc = "KEYR0 (rw) register accessor: an alias for `Reg<KEYR0_SPEC>`"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
#[doc = "AES key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (rw) register accessor: an alias for `Reg<KEYR1_SPEC>`"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
#[doc = "AES key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (rw) register accessor: an alias for `Reg<KEYR2_SPEC>`"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
#[doc = "AES key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (rw) register accessor: an alias for `Reg<KEYR3_SPEC>`"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
#[doc = "AES key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: an alias for `Reg<IVR0_SPEC>`"]
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
#[doc = "AES initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: an alias for `Reg<IVR1_SPEC>`"]
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
#[doc = "AES initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: an alias for `Reg<IVR2_SPEC>`"]
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
#[doc = "AES initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: an alias for `Reg<IVR3_SPEC>`"]
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
#[doc = "AES initialization vector register 3"]
pub mod ivr3;
#[doc = "KEYR4 (rw) register accessor: an alias for `Reg<KEYR4_SPEC>`"]
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
#[doc = "AES key register 4"]
pub mod keyr4;
#[doc = "KEYR5 (rw) register accessor: an alias for `Reg<KEYR5_SPEC>`"]
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
#[doc = "AES key register 5"]
pub mod keyr5;
#[doc = "KEYR6 (rw) register accessor: an alias for `Reg<KEYR6_SPEC>`"]
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
#[doc = "AES key register 6"]
pub mod keyr6;
#[doc = "KEYR7 (rw) register accessor: an alias for `Reg<KEYR7_SPEC>`"]
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
#[doc = "AES key register 7"]
pub mod keyr7;
#[doc = "SUSP0R (rw) register accessor: an alias for `Reg<SUSP0R_SPEC>`"]
pub type SUSP0R = crate::Reg<susp0r::SUSP0R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp0r;
#[doc = "SUSP1R (rw) register accessor: an alias for `Reg<SUSP1R_SPEC>`"]
pub type SUSP1R = crate::Reg<susp1r::SUSP1R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp1r;
#[doc = "SUSP2R (rw) register accessor: an alias for `Reg<SUSP2R_SPEC>`"]
pub type SUSP2R = crate::Reg<susp2r::SUSP2R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp2r;
#[doc = "SUSP3R (rw) register accessor: an alias for `Reg<SUSP3R_SPEC>`"]
pub type SUSP3R = crate::Reg<susp3r::SUSP3R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp3r;
#[doc = "SUSP4R (rw) register accessor: an alias for `Reg<SUSP4R_SPEC>`"]
pub type SUSP4R = crate::Reg<susp4r::SUSP4R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp4r;
#[doc = "SUSP5R (rw) register accessor: an alias for `Reg<SUSP5R_SPEC>`"]
pub type SUSP5R = crate::Reg<susp5r::SUSP5R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp5r;
#[doc = "SUSP6R (rw) register accessor: an alias for `Reg<SUSP6R_SPEC>`"]
pub type SUSP6R = crate::Reg<susp6r::SUSP6R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp6r;
#[doc = "SUSP7R (rw) register accessor: an alias for `Reg<SUSP7R_SPEC>`"]
pub type SUSP7R = crate::Reg<susp7r::SUSP7R_SPEC>;
#[doc = "AES suspend registers"]
pub mod susp7r;
