#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: BPCR,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: AWCR,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: TWCR,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: GCR,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    _reserved6: [u8; 0x04],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: BCCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: CPSR,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: CDSR,
    _reserved13: [u8; 0x38],
    #[doc = "0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer: [LAYER; 2],
}
impl RegisterBlock {
    #[doc = "0x84..0x104 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub fn layer1(&self) -> &LAYER {
        &self.layer[0]
    }
    #[doc = "0x104..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub fn layer2(&self) -> &LAYER {
        &self.layer[1]
    }
}
#[doc = "SSCR (rw) register accessor: an alias for `Reg<SSCR_SPEC>`"]
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR (rw) register accessor: an alias for `Reg<BPCR_SPEC>`"]
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR (rw) register accessor: an alias for `Reg<AWCR_SPEC>`"]
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR (rw) register accessor: an alias for `Reg<TWCR_SPEC>`"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "SRCR (rw) register accessor: an alias for `Reg<SRCR_SPEC>`"]
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR (rw) register accessor: an alias for `Reg<BCCR_SPEC>`"]
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR (rw) register accessor: an alias for `Reg<LIPCR_SPEC>`"]
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR (r) register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR (r) register accessor: an alias for `Reg<CDSR_SPEC>`"]
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub use self::layer::LAYER;
#[doc = r"Cluster"]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub mod layer;
