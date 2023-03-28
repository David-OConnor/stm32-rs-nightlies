#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CEC control register"]
    pub cr: CR,
    #[doc = "0x04 - This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0."]
    pub cfgr: CFGR,
    #[doc = "0x08 - CEC Tx data register"]
    pub txdr: TXDR,
    #[doc = "0x0c - CEC Rx Data Register"]
    pub rxdr: RXDR,
    #[doc = "0x10 - CEC Interrupt and Status Register"]
    pub isr: ISR,
    #[doc = "0x14 - CEC interrupt enable register"]
    pub ier: IER,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CEC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0."]
pub mod cfgr;
#[doc = "TXDR (w) register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "CEC Tx data register"]
pub mod txdr;
#[doc = "RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "CEC Rx Data Register"]
pub mod rxdr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "CEC Interrupt and Status Register"]
pub mod isr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CEC interrupt enable register"]
pub mod ier;
