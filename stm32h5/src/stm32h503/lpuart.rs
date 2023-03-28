#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_lpuart_cr1: [u8; 0x04],
    #[doc = "0x04 - LPUART control register 2"]
    pub lpuart_cr2: LPUART_CR2,
    #[doc = "0x08 - LPUART control register 3"]
    pub lpuart_cr3: LPUART_CR3,
    #[doc = "0x0c - LPUART baud rate register"]
    pub lpuart_brr: LPUART_BRR,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - LPUART request register"]
    pub lpuart_rqr: LPUART_RQR,
    _reserved_5_lpuart_isr: [u8; 0x04],
    #[doc = "0x20 - LPUART interrupt flag clear register"]
    pub lpuart_icr: LPUART_ICR,
    #[doc = "0x24 - LPUART receive data register"]
    pub lpuart_rdr: LPUART_RDR,
    #[doc = "0x28 - LPUART transmit data register"]
    pub lpuart_tdr: LPUART_TDR,
    #[doc = "0x2c - LPUART prescaler register"]
    pub lpuart_presc: LPUART_PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - LPUART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn lpuart_cr1_disabled(&self) -> &LPUART_CR1_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - LPUART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn lpuart_cr1_enabled(&self) -> &LPUART_CR1_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x1c - LPUART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lpuart_isr_disabled(&self) -> &LPUART_ISR_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - LPUART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn lpuart_isr_enabled(&self) -> &LPUART_ISR_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "LPUART_CR1_enabled (rw) register accessor: an alias for `Reg<LPUART_CR1_ENABLED_SPEC>`"]
pub type LPUART_CR1_ENABLED = crate::Reg<lpuart_cr1_enabled::LPUART_CR1_ENABLED_SPEC>;
#[doc = "LPUART control register 1 \\[alternate\\]"]
pub mod lpuart_cr1_enabled;
#[doc = "LPUART_CR1_disabled (rw) register accessor: an alias for `Reg<LPUART_CR1_DISABLED_SPEC>`"]
pub type LPUART_CR1_DISABLED = crate::Reg<lpuart_cr1_disabled::LPUART_CR1_DISABLED_SPEC>;
#[doc = "LPUART control register 1 \\[alternate\\]"]
pub mod lpuart_cr1_disabled;
#[doc = "LPUART_CR2 (rw) register accessor: an alias for `Reg<LPUART_CR2_SPEC>`"]
pub type LPUART_CR2 = crate::Reg<lpuart_cr2::LPUART_CR2_SPEC>;
#[doc = "LPUART control register 2"]
pub mod lpuart_cr2;
#[doc = "LPUART_CR3 (rw) register accessor: an alias for `Reg<LPUART_CR3_SPEC>`"]
pub type LPUART_CR3 = crate::Reg<lpuart_cr3::LPUART_CR3_SPEC>;
#[doc = "LPUART control register 3"]
pub mod lpuart_cr3;
#[doc = "LPUART_BRR (rw) register accessor: an alias for `Reg<LPUART_BRR_SPEC>`"]
pub type LPUART_BRR = crate::Reg<lpuart_brr::LPUART_BRR_SPEC>;
#[doc = "LPUART baud rate register"]
pub mod lpuart_brr;
#[doc = "LPUART_RQR (w) register accessor: an alias for `Reg<LPUART_RQR_SPEC>`"]
pub type LPUART_RQR = crate::Reg<lpuart_rqr::LPUART_RQR_SPEC>;
#[doc = "LPUART request register"]
pub mod lpuart_rqr;
#[doc = "LPUART_ISR_enabled (r) register accessor: an alias for `Reg<LPUART_ISR_ENABLED_SPEC>`"]
pub type LPUART_ISR_ENABLED = crate::Reg<lpuart_isr_enabled::LPUART_ISR_ENABLED_SPEC>;
#[doc = "LPUART interrupt and status register \\[alternate\\]"]
pub mod lpuart_isr_enabled;
#[doc = "LPUART_ISR_disabled (r) register accessor: an alias for `Reg<LPUART_ISR_DISABLED_SPEC>`"]
pub type LPUART_ISR_DISABLED = crate::Reg<lpuart_isr_disabled::LPUART_ISR_DISABLED_SPEC>;
#[doc = "LPUART interrupt and status register \\[alternate\\]"]
pub mod lpuart_isr_disabled;
#[doc = "LPUART_ICR (w) register accessor: an alias for `Reg<LPUART_ICR_SPEC>`"]
pub type LPUART_ICR = crate::Reg<lpuart_icr::LPUART_ICR_SPEC>;
#[doc = "LPUART interrupt flag clear register"]
pub mod lpuart_icr;
#[doc = "LPUART_RDR (r) register accessor: an alias for `Reg<LPUART_RDR_SPEC>`"]
pub type LPUART_RDR = crate::Reg<lpuart_rdr::LPUART_RDR_SPEC>;
#[doc = "LPUART receive data register"]
pub mod lpuart_rdr;
#[doc = "LPUART_TDR (rw) register accessor: an alias for `Reg<LPUART_TDR_SPEC>`"]
pub type LPUART_TDR = crate::Reg<lpuart_tdr::LPUART_TDR_SPEC>;
#[doc = "LPUART transmit data register"]
pub mod lpuart_tdr;
#[doc = "LPUART_PRESC (rw) register accessor: an alias for `Reg<LPUART_PRESC_SPEC>`"]
pub type LPUART_PRESC = crate::Reg<lpuart_presc::LPUART_PRESC_SPEC>;
#[doc = "LPUART prescaler register"]
pub mod lpuart_presc;
