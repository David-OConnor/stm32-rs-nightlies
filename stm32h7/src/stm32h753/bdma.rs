#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
    pub ch: [CH; 8],
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
pub mod ch;
