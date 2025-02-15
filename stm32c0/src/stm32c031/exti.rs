#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - EXTI falling trigger selection register 1"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - EXTI software interrupt event register 1"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI rising edge pending register 1"]
    pub rpr1: RPR1,
    #[doc = "0x10 - EXTI falling edge pending register 1"]
    pub fpr1: FPR1,
    _reserved5: [u8; 0x4c],
    #[doc = "0x60 - EXTI external interrupt selection register"]
    pub exticr1: EXTICR1,
    #[doc = "0x64 - EXTI external interrupt selection register"]
    pub exticr2: EXTICR2,
    #[doc = "0x68 - EXTI external interrupt selection register"]
    pub exticr3: EXTICR3,
    #[doc = "0x6c - EXTI external interrupt selection register"]
    pub exticr4: EXTICR4,
    _reserved9: [u8; 0x10],
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    pub imr1: IMR1,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub emr1: EMR1,
}
#[doc = "RTSR1 (rw) register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "EXTI falling trigger selection register 1"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "EXTI software interrupt event register 1"]
pub mod swier1;
#[doc = "RPR1 (rw) register accessor: an alias for `Reg<RPR1_SPEC>`"]
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
#[doc = "EXTI rising edge pending register 1"]
pub mod rpr1;
#[doc = "FPR1 (rw) register accessor: an alias for `Reg<FPR1_SPEC>`"]
pub type FPR1 = crate::Reg<fpr1::FPR1_SPEC>;
#[doc = "EXTI falling edge pending register 1"]
pub mod fpr1;
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
