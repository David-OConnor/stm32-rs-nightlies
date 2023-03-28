#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - MDMA channel x interrupt/status register"]
    pub isr: ISR,
    #[doc = "0x04 - MDMA channel x interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - MDMA Channel x error status register"]
    pub esr: ESR,
    #[doc = "0x0c - This register is used to control the concerned channel."]
    pub cr: CR,
    #[doc = "0x10 - This register is used to configure the concerned channel."]
    pub tcr: TCR,
    #[doc = "0x14 - MDMA Channel x block number of data register"]
    pub bndtr: BNDTR,
    #[doc = "0x18 - MDMA channel x source address register"]
    pub sar: SAR,
    #[doc = "0x1c - MDMA channel x destination address register"]
    pub dar: DAR,
    #[doc = "0x20 - MDMA channel x Block Repeat address Update register"]
    pub brur: BRUR,
    #[doc = "0x24 - MDMA channel x Link Address register"]
    pub lar: LAR,
    #[doc = "0x28 - MDMA channel x Trigger and Bus selection Register"]
    pub tbr: TBR,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - MDMA channel x Mask address register"]
    pub mar: MAR,
    #[doc = "0x34 - MDMA channel x Mask Data register"]
    pub mdr: MDR,
    _reserved_end: [u8; 0x08],
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod ifcr;
#[doc = "ESR (r) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod esr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod cr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod tcr;
#[doc = "BNDTR (rw) register accessor: an alias for `Reg<BNDTR_SPEC>`"]
pub type BNDTR = crate::Reg<bndtr::BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod bndtr;
#[doc = "SAR (rw) register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod dar;
#[doc = "BRUR (rw) register accessor: an alias for `Reg<BRUR_SPEC>`"]
pub type BRUR = crate::Reg<brur::BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod brur;
#[doc = "LAR (rw) register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod lar;
#[doc = "TBR (rw) register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod tbr;
#[doc = "MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mar;
#[doc = "MDR (rw) register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdr;
