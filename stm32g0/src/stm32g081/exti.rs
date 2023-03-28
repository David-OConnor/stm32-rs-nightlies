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
    _reserved11: [u8; 0x08],
    #[doc = "0x90 - EXTI CPU wakeup with interrupt mask register"]
    pub imr2: IMR2,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub emr2: EMR2,
    _reserved13: [u8; 0x0340],
    #[doc = "0x3d8 - Hardware configuration registers"]
    pub hwcfgr7: HWCFGR7,
    #[doc = "0x3dc - Hardware configuration registers"]
    pub hwcfgr6: HWCFGR6,
    #[doc = "0x3e0 - Hardware configuration registers"]
    pub hwcfgr5: HWCFGR5,
    #[doc = "0x3e4 - Hardware configuration registers"]
    pub hwcfgr4: HWCFGR4,
    #[doc = "0x3e8 - Hardware configuration registers"]
    pub hwcfgr3: HWCFGR3,
    #[doc = "0x3ec - Hardware configuration registers"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Hardware configuration registers"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - AES version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - AES identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - AES size ID register"]
    pub sidr: SIDR,
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
#[doc = "HWCFGR7 (rw) register accessor: an alias for `Reg<HWCFGR7_SPEC>`"]
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "HWCFGR6 (rw) register accessor: an alias for `Reg<HWCFGR6_SPEC>`"]
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "HWCFGR5 (rw) register accessor: an alias for `Reg<HWCFGR5_SPEC>`"]
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "HWCFGR4 (rw) register accessor: an alias for `Reg<HWCFGR4_SPEC>`"]
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "HWCFGR3 (rw) register accessor: an alias for `Reg<HWCFGR3_SPEC>`"]
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "HWCFGR2 (rw) register accessor: an alias for `Reg<HWCFGR2_SPEC>`"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr1;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "AES version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "AES identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "AES size ID register"]
pub mod sidr;
