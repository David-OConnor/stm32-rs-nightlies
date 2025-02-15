#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - device configuration register"]
    pub dcr1: DCR1,
    #[doc = "0x0c - device configuration register 2"]
    pub dcr2: DCR2,
    #[doc = "0x10 - device configuration register 3"]
    pub dcr3: DCR3,
    #[doc = "0x14 - Device configuration register 4"]
    pub dcr4: DCR4,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - status register"]
    pub sr: SR,
    #[doc = "0x24 - flag clear register"]
    pub fcr: FCR,
    _reserved7: [u8; 0x18],
    #[doc = "0x40 - data length register"]
    pub dlr: DLR,
    _reserved8: [u8; 0x04],
    #[doc = "0x48 - address register"]
    pub ar: AR,
    _reserved9: [u8; 0x04],
    #[doc = "0x50 - data register"]
    pub dr: DR,
    _reserved10: [u8; 0x2c],
    #[doc = "0x80 - polling status mask register"]
    pub psmkr: PSMKR,
    _reserved11: [u8; 0x04],
    #[doc = "0x88 - polling status match register"]
    pub psmar: PSMAR,
    _reserved12: [u8; 0x04],
    #[doc = "0x90 - polling interval register"]
    pub pir: PIR,
    _reserved13: [u8; 0x6c],
    #[doc = "0x100 - communication configuration register"]
    pub ccr: CCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x108 - timing configuration register"]
    pub tcr: TCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x110 - instruction register"]
    pub ir: IR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x120 - alternate bytes register"]
    pub abr: ABR,
    _reserved17: [u8; 0x0c],
    #[doc = "0x130 - low-power timeout register"]
    pub lptr: LPTR,
    _reserved18: [u8; 0x0c],
    #[doc = "0x140 - wrap communication configuration register"]
    pub wpccr: WPCCR,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - Wrap timing configuration register"]
    pub wptcr: WPTCR,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - Wrap instruction register"]
    pub wpir: WPIR,
    _reserved21: [u8; 0x0c],
    #[doc = "0x160 - Wrap alternate bytes register"]
    pub wpabr: WPABR,
    _reserved22: [u8; 0x1c],
    #[doc = "0x180 - write communication configuration register"]
    pub wccr: WCCR,
    _reserved23: [u8; 0x04],
    #[doc = "0x188 - write timing configuration register"]
    pub wtcr: WTCR,
    _reserved24: [u8; 0x04],
    #[doc = "0x190 - write instruction register"]
    pub wir: WIR,
    _reserved25: [u8; 0x0c],
    #[doc = "0x1a0 - write alternate bytes register"]
    pub wabr: WABR,
    _reserved26: [u8; 0x5c],
    #[doc = "0x200 - HyperBusTM latency configuration register"]
    pub hlcr: HLCR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DCR1 (rw) register accessor: an alias for `Reg<DCR1_SPEC>`"]
pub type DCR1 = crate::Reg<dcr1::DCR1_SPEC>;
#[doc = "device configuration register"]
pub mod dcr1;
#[doc = "DCR2 (rw) register accessor: an alias for `Reg<DCR2_SPEC>`"]
pub type DCR2 = crate::Reg<dcr2::DCR2_SPEC>;
#[doc = "device configuration register 2"]
pub mod dcr2;
#[doc = "DCR3 (rw) register accessor: an alias for `Reg<DCR3_SPEC>`"]
pub type DCR3 = crate::Reg<dcr3::DCR3_SPEC>;
#[doc = "device configuration register 3"]
pub mod dcr3;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: an alias for `Reg<DLR_SPEC>`"]
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
#[doc = "data length register"]
pub mod dlr;
#[doc = "AR (rw) register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "address register"]
pub mod ar;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: an alias for `Reg<PSMKR_SPEC>`"]
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: an alias for `Reg<PSMAR_SPEC>`"]
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`"]
pub type PIR = crate::Reg<pir::PIR_SPEC>;
#[doc = "polling interval register"]
pub mod pir;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "timing configuration register"]
pub mod tcr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "instruction register"]
pub mod ir;
#[doc = "ABR (rw) register accessor: an alias for `Reg<ABR_SPEC>`"]
pub type ABR = crate::Reg<abr::ABR_SPEC>;
#[doc = "alternate bytes register"]
pub mod abr;
#[doc = "LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`"]
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
#[doc = "low-power timeout register"]
pub mod lptr;
#[doc = "WCCR (rw) register accessor: an alias for `Reg<WCCR_SPEC>`"]
pub type WCCR = crate::Reg<wccr::WCCR_SPEC>;
#[doc = "write communication configuration register"]
pub mod wccr;
#[doc = "WTCR (rw) register accessor: an alias for `Reg<WTCR_SPEC>`"]
pub type WTCR = crate::Reg<wtcr::WTCR_SPEC>;
#[doc = "write timing configuration register"]
pub mod wtcr;
#[doc = "WIR (rw) register accessor: an alias for `Reg<WIR_SPEC>`"]
pub type WIR = crate::Reg<wir::WIR_SPEC>;
#[doc = "write instruction register"]
pub mod wir;
#[doc = "WABR (rw) register accessor: an alias for `Reg<WABR_SPEC>`"]
pub type WABR = crate::Reg<wabr::WABR_SPEC>;
#[doc = "write alternate bytes register"]
pub mod wabr;
#[doc = "HLCR (rw) register accessor: an alias for `Reg<HLCR_SPEC>`"]
pub type HLCR = crate::Reg<hlcr::HLCR_SPEC>;
#[doc = "HyperBusTM latency configuration register"]
pub mod hlcr;
#[doc = "DCR4 (rw) register accessor: an alias for `Reg<DCR4_SPEC>`"]
pub type DCR4 = crate::Reg<dcr4::DCR4_SPEC>;
#[doc = "Device configuration register 4"]
pub mod dcr4;
#[doc = "WPCCR (rw) register accessor: an alias for `Reg<WPCCR_SPEC>`"]
pub type WPCCR = crate::Reg<wpccr::WPCCR_SPEC>;
#[doc = "wrap communication configuration register"]
pub mod wpccr;
#[doc = "WPTCR (rw) register accessor: an alias for `Reg<WPTCR_SPEC>`"]
pub type WPTCR = crate::Reg<wptcr::WPTCR_SPEC>;
#[doc = "Wrap timing configuration register"]
pub mod wptcr;
#[doc = "WPIR (rw) register accessor: an alias for `Reg<WPIR_SPEC>`"]
pub type WPIR = crate::Reg<wpir::WPIR_SPEC>;
#[doc = "Wrap instruction register"]
pub mod wpir;
#[doc = "WPABR (rw) register accessor: an alias for `Reg<WPABR_SPEC>`"]
pub type WPABR = crate::Reg<wpabr::WPABR_SPEC>;
#[doc = "Wrap alternate bytes register"]
pub mod wpabr;
