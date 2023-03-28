#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OCTOSPI control register"]
    pub octospi_cr: OCTOSPI_CR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - OCTOSPI device configuration register 1"]
    pub octospi_dcr1: OCTOSPI_DCR1,
    #[doc = "0x0c - OCTOSPI device configuration register 2"]
    pub octospi_dcr2: OCTOSPI_DCR2,
    #[doc = "0x10 - OCTOSPI device configuration register 3"]
    pub octospi_dcr3: OCTOSPI_DCR3,
    #[doc = "0x14 - OCTOSPI device configuration register 4"]
    pub octospi_dcr4: OCTOSPI_DCR4,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - OCTOSPI status register"]
    pub octospi_sr: OCTOSPI_SR,
    #[doc = "0x24 - OCTOSPI flag clear register"]
    pub octospi_fcr: OCTOSPI_FCR,
    _reserved7: [u8; 0x18],
    #[doc = "0x40 - OCTOSPI data length register"]
    pub octospi_dlr: OCTOSPI_DLR,
    _reserved8: [u8; 0x04],
    #[doc = "0x48 - OCTOSPI address register"]
    pub octospi_ar: OCTOSPI_AR,
    _reserved9: [u8; 0x04],
    #[doc = "0x50 - OCTOSPI data register"]
    pub octospi_dr: OCTOSPI_DR,
    _reserved10: [u8; 0x2c],
    #[doc = "0x80 - OCTOSPI polling status mask register"]
    pub octospi_psmkr: OCTOSPI_PSMKR,
    _reserved11: [u8; 0x04],
    #[doc = "0x88 - OCTOSPI polling status match register"]
    pub octospi_psmar: OCTOSPI_PSMAR,
    _reserved12: [u8; 0x04],
    #[doc = "0x90 - OCTOSPI polling interval register"]
    pub octospi_pir: OCTOSPI_PIR,
    _reserved13: [u8; 0x6c],
    #[doc = "0x100 - OCTOSPI communication configuration register"]
    pub octospi_ccr: OCTOSPI_CCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x108 - OCTOSPI timing configuration register"]
    pub octospi_tcr: OCTOSPI_TCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x110 - OCTOSPI instruction register"]
    pub octospi_ir: OCTOSPI_IR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x120 - OCTOSPI alternate bytes register"]
    pub octospi_abr: OCTOSPI_ABR,
    _reserved17: [u8; 0x0c],
    #[doc = "0x130 - OCTOSPI low-power timeout register"]
    pub octospi_lptr: OCTOSPI_LPTR,
    _reserved18: [u8; 0x0c],
    #[doc = "0x140 - OCTOSPI wrap communication configuration register"]
    pub octospi_wpccr: OCTOSPI_WPCCR,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - OCTOSPI wrap timing configuration register"]
    pub octospi_wptcr: OCTOSPI_WPTCR,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - OCTOSPI wrap instruction register"]
    pub octospi_wpir: OCTOSPI_WPIR,
    _reserved21: [u8; 0x0c],
    #[doc = "0x160 - OCTOSPI wrap alternate bytes register"]
    pub octospi_wpabr: OCTOSPI_WPABR,
    _reserved22: [u8; 0x1c],
    #[doc = "0x180 - OCTOSPI write communication configuration register"]
    pub octospi_wccr: OCTOSPI_WCCR,
    _reserved23: [u8; 0x04],
    #[doc = "0x188 - OCTOSPI write timing configuration register"]
    pub octospi_wtcr: OCTOSPI_WTCR,
    _reserved24: [u8; 0x04],
    #[doc = "0x190 - OCTOSPI write instruction register"]
    pub octospi_wir: OCTOSPI_WIR,
    _reserved25: [u8; 0x0c],
    #[doc = "0x1a0 - OCTOSPI write alternate bytes register"]
    pub octospi_wabr: OCTOSPI_WABR,
    _reserved26: [u8; 0x5c],
    #[doc = "0x200 - OCTOSPI HyperBus latency configuration register"]
    pub octospi_hlcr: OCTOSPI_HLCR,
}
#[doc = "OCTOSPI_CR (rw) register accessor: an alias for `Reg<OCTOSPI_CR_SPEC>`"]
pub type OCTOSPI_CR = crate::Reg<octospi_cr::OCTOSPI_CR_SPEC>;
#[doc = "OCTOSPI control register"]
pub mod octospi_cr;
#[doc = "OCTOSPI_DCR1 (rw) register accessor: an alias for `Reg<OCTOSPI_DCR1_SPEC>`"]
pub type OCTOSPI_DCR1 = crate::Reg<octospi_dcr1::OCTOSPI_DCR1_SPEC>;
#[doc = "OCTOSPI device configuration register 1"]
pub mod octospi_dcr1;
#[doc = "OCTOSPI_DCR2 (rw) register accessor: an alias for `Reg<OCTOSPI_DCR2_SPEC>`"]
pub type OCTOSPI_DCR2 = crate::Reg<octospi_dcr2::OCTOSPI_DCR2_SPEC>;
#[doc = "OCTOSPI device configuration register 2"]
pub mod octospi_dcr2;
#[doc = "OCTOSPI_DCR3 (rw) register accessor: an alias for `Reg<OCTOSPI_DCR3_SPEC>`"]
pub type OCTOSPI_DCR3 = crate::Reg<octospi_dcr3::OCTOSPI_DCR3_SPEC>;
#[doc = "OCTOSPI device configuration register 3"]
pub mod octospi_dcr3;
#[doc = "OCTOSPI_DCR4 (rw) register accessor: an alias for `Reg<OCTOSPI_DCR4_SPEC>`"]
pub type OCTOSPI_DCR4 = crate::Reg<octospi_dcr4::OCTOSPI_DCR4_SPEC>;
#[doc = "OCTOSPI device configuration register 4"]
pub mod octospi_dcr4;
#[doc = "OCTOSPI_SR (r) register accessor: an alias for `Reg<OCTOSPI_SR_SPEC>`"]
pub type OCTOSPI_SR = crate::Reg<octospi_sr::OCTOSPI_SR_SPEC>;
#[doc = "OCTOSPI status register"]
pub mod octospi_sr;
#[doc = "OCTOSPI_FCR (w) register accessor: an alias for `Reg<OCTOSPI_FCR_SPEC>`"]
pub type OCTOSPI_FCR = crate::Reg<octospi_fcr::OCTOSPI_FCR_SPEC>;
#[doc = "OCTOSPI flag clear register"]
pub mod octospi_fcr;
#[doc = "OCTOSPI_DLR (rw) register accessor: an alias for `Reg<OCTOSPI_DLR_SPEC>`"]
pub type OCTOSPI_DLR = crate::Reg<octospi_dlr::OCTOSPI_DLR_SPEC>;
#[doc = "OCTOSPI data length register"]
pub mod octospi_dlr;
#[doc = "OCTOSPI_AR (rw) register accessor: an alias for `Reg<OCTOSPI_AR_SPEC>`"]
pub type OCTOSPI_AR = crate::Reg<octospi_ar::OCTOSPI_AR_SPEC>;
#[doc = "OCTOSPI address register"]
pub mod octospi_ar;
#[doc = "OCTOSPI_DR (rw) register accessor: an alias for `Reg<OCTOSPI_DR_SPEC>`"]
pub type OCTOSPI_DR = crate::Reg<octospi_dr::OCTOSPI_DR_SPEC>;
#[doc = "OCTOSPI data register"]
pub mod octospi_dr;
#[doc = "OCTOSPI_PSMKR (rw) register accessor: an alias for `Reg<OCTOSPI_PSMKR_SPEC>`"]
pub type OCTOSPI_PSMKR = crate::Reg<octospi_psmkr::OCTOSPI_PSMKR_SPEC>;
#[doc = "OCTOSPI polling status mask register"]
pub mod octospi_psmkr;
#[doc = "OCTOSPI_PSMAR (rw) register accessor: an alias for `Reg<OCTOSPI_PSMAR_SPEC>`"]
pub type OCTOSPI_PSMAR = crate::Reg<octospi_psmar::OCTOSPI_PSMAR_SPEC>;
#[doc = "OCTOSPI polling status match register"]
pub mod octospi_psmar;
#[doc = "OCTOSPI_PIR (rw) register accessor: an alias for `Reg<OCTOSPI_PIR_SPEC>`"]
pub type OCTOSPI_PIR = crate::Reg<octospi_pir::OCTOSPI_PIR_SPEC>;
#[doc = "OCTOSPI polling interval register"]
pub mod octospi_pir;
#[doc = "OCTOSPI_CCR (rw) register accessor: an alias for `Reg<OCTOSPI_CCR_SPEC>`"]
pub type OCTOSPI_CCR = crate::Reg<octospi_ccr::OCTOSPI_CCR_SPEC>;
#[doc = "OCTOSPI communication configuration register"]
pub mod octospi_ccr;
#[doc = "OCTOSPI_TCR (rw) register accessor: an alias for `Reg<OCTOSPI_TCR_SPEC>`"]
pub type OCTOSPI_TCR = crate::Reg<octospi_tcr::OCTOSPI_TCR_SPEC>;
#[doc = "OCTOSPI timing configuration register"]
pub mod octospi_tcr;
#[doc = "OCTOSPI_IR (rw) register accessor: an alias for `Reg<OCTOSPI_IR_SPEC>`"]
pub type OCTOSPI_IR = crate::Reg<octospi_ir::OCTOSPI_IR_SPEC>;
#[doc = "OCTOSPI instruction register"]
pub mod octospi_ir;
#[doc = "OCTOSPI_ABR (rw) register accessor: an alias for `Reg<OCTOSPI_ABR_SPEC>`"]
pub type OCTOSPI_ABR = crate::Reg<octospi_abr::OCTOSPI_ABR_SPEC>;
#[doc = "OCTOSPI alternate bytes register"]
pub mod octospi_abr;
#[doc = "OCTOSPI_LPTR (rw) register accessor: an alias for `Reg<OCTOSPI_LPTR_SPEC>`"]
pub type OCTOSPI_LPTR = crate::Reg<octospi_lptr::OCTOSPI_LPTR_SPEC>;
#[doc = "OCTOSPI low-power timeout register"]
pub mod octospi_lptr;
#[doc = "OCTOSPI_WPCCR (rw) register accessor: an alias for `Reg<OCTOSPI_WPCCR_SPEC>`"]
pub type OCTOSPI_WPCCR = crate::Reg<octospi_wpccr::OCTOSPI_WPCCR_SPEC>;
#[doc = "OCTOSPI wrap communication configuration register"]
pub mod octospi_wpccr;
#[doc = "OCTOSPI_WPTCR (rw) register accessor: an alias for `Reg<OCTOSPI_WPTCR_SPEC>`"]
pub type OCTOSPI_WPTCR = crate::Reg<octospi_wptcr::OCTOSPI_WPTCR_SPEC>;
#[doc = "OCTOSPI wrap timing configuration register"]
pub mod octospi_wptcr;
#[doc = "OCTOSPI_WPIR (rw) register accessor: an alias for `Reg<OCTOSPI_WPIR_SPEC>`"]
pub type OCTOSPI_WPIR = crate::Reg<octospi_wpir::OCTOSPI_WPIR_SPEC>;
#[doc = "OCTOSPI wrap instruction register"]
pub mod octospi_wpir;
#[doc = "OCTOSPI_WPABR (rw) register accessor: an alias for `Reg<OCTOSPI_WPABR_SPEC>`"]
pub type OCTOSPI_WPABR = crate::Reg<octospi_wpabr::OCTOSPI_WPABR_SPEC>;
#[doc = "OCTOSPI wrap alternate bytes register"]
pub mod octospi_wpabr;
#[doc = "OCTOSPI_WCCR (rw) register accessor: an alias for `Reg<OCTOSPI_WCCR_SPEC>`"]
pub type OCTOSPI_WCCR = crate::Reg<octospi_wccr::OCTOSPI_WCCR_SPEC>;
#[doc = "OCTOSPI write communication configuration register"]
pub mod octospi_wccr;
#[doc = "OCTOSPI_WTCR (rw) register accessor: an alias for `Reg<OCTOSPI_WTCR_SPEC>`"]
pub type OCTOSPI_WTCR = crate::Reg<octospi_wtcr::OCTOSPI_WTCR_SPEC>;
#[doc = "OCTOSPI write timing configuration register"]
pub mod octospi_wtcr;
#[doc = "OCTOSPI_WIR (rw) register accessor: an alias for `Reg<OCTOSPI_WIR_SPEC>`"]
pub type OCTOSPI_WIR = crate::Reg<octospi_wir::OCTOSPI_WIR_SPEC>;
#[doc = "OCTOSPI write instruction register"]
pub mod octospi_wir;
#[doc = "OCTOSPI_WABR (rw) register accessor: an alias for `Reg<OCTOSPI_WABR_SPEC>`"]
pub type OCTOSPI_WABR = crate::Reg<octospi_wabr::OCTOSPI_WABR_SPEC>;
#[doc = "OCTOSPI write alternate bytes register"]
pub mod octospi_wabr;
#[doc = "OCTOSPI_HLCR (rw) register accessor: an alias for `Reg<OCTOSPI_HLCR_SPEC>`"]
pub type OCTOSPI_HLCR = crate::Reg<octospi_hlcr::OCTOSPI_HLCR_SPEC>;
#[doc = "OCTOSPI HyperBus latency configuration register"]
pub mod octospi_hlcr;
