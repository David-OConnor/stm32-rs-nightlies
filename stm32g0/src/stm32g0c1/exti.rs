#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI rising edge pending register"]
    pub rpr1: RPR1,
    #[doc = "0x10 - EXTI falling edge pending register"]
    pub fpr1: FPR1,
    _reserved5: [u8; 0x14],
    #[doc = "0x28 - EXTI rising trigger selection register 2"]
    pub rtsr2: RTSR2,
    #[doc = "0x2c - EXTI falling trigger selection register 2"]
    pub ftsr2: FTSR2,
    #[doc = "0x30 - EXTI software interrupt event register 2"]
    pub swier2: SWIER2,
    #[doc = "0x34 - EXTI rising edge pending register 2"]
    pub rpr2: RPR2,
    #[doc = "0x38 - EXTI falling edge pending register 2"]
    pub fpr2: FPR2,
    _reserved10: [u8; 0x24],
    #[doc = "0x60 - EXTI external interrupt selection register"]
    pub exticr1: EXTICR1,
    #[doc = "0x64 - EXTI external interrupt selection register"]
    pub exticr2: EXTICR2,
    #[doc = "0x68 - EXTI external interrupt selection register"]
    pub exticr3: EXTICR3,
    #[doc = "0x6c - EXTI external interrupt selection register"]
    pub exticr4: EXTICR4,
    _reserved14: [u8; 0x10],
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    pub imr1: IMR1,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub emr1: EMR1,
    _reserved16: [u8; 0x08],
    #[doc = "0x90 - EXTI CPU wakeup with interrupt mask register"]
    pub imr2: IMR2,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub emr2: EMR2,
}
#[doc = "RTSR1 (rw) register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "RPR1 (rw) register accessor: an alias for `Reg<RPR1_SPEC>`"]
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "FPR1 (rw) register accessor: an alias for `Reg<FPR1_SPEC>`"]
pub type FPR1 = crate::Reg<fpr1::FPR1_SPEC>;
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "RTSR2 (rw) register accessor: an alias for `Reg<RTSR2_SPEC>`"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
#[doc = "EXTI rising trigger selection register 2"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: an alias for `Reg<FTSR2_SPEC>`"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
#[doc = "EXTI falling trigger selection register 2"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: an alias for `Reg<SWIER2_SPEC>`"]
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
#[doc = "EXTI software interrupt event register 2"]
pub mod swier2;
#[doc = "RPR2 (rw) register accessor: an alias for `Reg<RPR2_SPEC>`"]
pub type RPR2 = crate::Reg<rpr2::RPR2_SPEC>;
#[doc = "EXTI rising edge pending register 2"]
pub mod rpr2;
#[doc = "FPR2 (rw) register accessor: an alias for `Reg<FPR2_SPEC>`"]
pub type FPR2 = crate::Reg<fpr2::FPR2_SPEC>;
#[doc = "EXTI falling edge pending register 2"]
pub mod fpr2;
#[doc = "EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "IMR1 (rw) register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 (rw) register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr1;
#[doc = "IMR2 (rw) register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "EMR2 (rw) register accessor: an alias for `Reg<EMR2_SPEC>`"]
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr2;
