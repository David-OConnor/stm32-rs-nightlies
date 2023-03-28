#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    #[doc = "0x04 - USART control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - USART control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - USART baud rate register"]
    pub brr: BRR,
    #[doc = "0x10 - USART guard time and prescaler register"]
    pub gtpr: GTPR,
    #[doc = "0x14 - USART receiver timeout register"]
    pub rtor: RTOR,
    #[doc = "0x18 - USART request register"]
    pub rqr: RQR,
    _reserved_7_isr: [u8; 0x04],
    #[doc = "0x20 - USART interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x24 - USART receive data register"]
    pub rdr: RDR,
    #[doc = "0x28 - USART transmit data register"]
    pub tdr: TDR,
    #[doc = "0x2c - USART prescaler register"]
    pub presc: PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - USART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - USART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x1c - USART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - USART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "CR1_enabled (rw) register accessor: an alias for `Reg<CR1_ENABLED_SPEC>`"]
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>;
#[doc = "USART control register 1 \\[alternate\\]"]
pub mod cr1_enabled;
#[doc = "CR1_disabled (rw) register accessor: an alias for `Reg<CR1_DISABLED_SPEC>`"]
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>;
#[doc = "USART control register 1 \\[alternate\\]"]
pub mod cr1_disabled;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "USART control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "USART control register 3"]
pub mod cr3;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "USART baud rate register"]
pub mod brr;
#[doc = "GTPR (rw) register accessor: an alias for `Reg<GTPR_SPEC>`"]
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
#[doc = "USART guard time and prescaler register"]
pub mod gtpr;
#[doc = "RTOR (rw) register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "USART receiver timeout register"]
pub mod rtor;
#[doc = "RQR (w) register accessor: an alias for `Reg<RQR_SPEC>`"]
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
#[doc = "USART request register"]
pub mod rqr;
#[doc = "ISR_enabled (r) register accessor: an alias for `Reg<ISR_ENABLED_SPEC>`"]
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLED_SPEC>;
#[doc = "USART interrupt and status register \\[alternate\\]"]
pub mod isr_enabled;
#[doc = "ISR_disabled (r) register accessor: an alias for `Reg<ISR_DISABLED_SPEC>`"]
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLED_SPEC>;
#[doc = "USART interrupt and status register \\[alternate\\]"]
pub mod isr_disabled;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "USART interrupt flag clear register"]
pub mod icr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "USART receive data register"]
pub mod rdr;
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "USART transmit data register"]
pub mod tdr;
#[doc = "PRESC (rw) register accessor: an alias for `Reg<PRESC_SPEC>`"]
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
#[doc = "USART prescaler register"]
pub mod presc;
