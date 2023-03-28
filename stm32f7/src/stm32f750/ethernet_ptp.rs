#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register"]
    pub ptptscr: PTPTSCR,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptpssir: PTPSSIR,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptptshr: PTPTSHR,
    #[doc = "0x0c - Ethernet PTP time stamp low register"]
    pub ptptslr: PTPTSLR,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptptshur: PTPTSHUR,
    #[doc = "0x14 - Ethernet PTP time stamp low update register"]
    pub ptptslur: PTPTSLUR,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptptsar: PTPTSAR,
    #[doc = "0x1c - Ethernet PTP target time high register"]
    pub ptptthr: PTPTTHR,
    #[doc = "0x20 - Ethernet PTP target time low register"]
    pub ptpttlr: PTPTTLR,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Ethernet PTP time stamp status register"]
    pub ptptssr: PTPTSSR,
    #[doc = "0x2c - Ethernet PTP PPS control register"]
    pub ptpppscr: PTPPPSCR,
}
#[doc = "PTPTSCR (rw) register accessor: an alias for `Reg<PTPTSCR_SPEC>`"]
pub type PTPTSCR = crate::Reg<ptptscr::PTPTSCR_SPEC>;
#[doc = "Ethernet PTP time stamp control register"]
pub mod ptptscr;
#[doc = "PTPSSIR (rw) register accessor: an alias for `Reg<PTPSSIR_SPEC>`"]
pub type PTPSSIR = crate::Reg<ptpssir::PTPSSIR_SPEC>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "PTPTSHR (r) register accessor: an alias for `Reg<PTPTSHR_SPEC>`"]
pub type PTPTSHR = crate::Reg<ptptshr::PTPTSHR_SPEC>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "PTPTSLR (r) register accessor: an alias for `Reg<PTPTSLR_SPEC>`"]
pub type PTPTSLR = crate::Reg<ptptslr::PTPTSLR_SPEC>;
#[doc = "Ethernet PTP time stamp low register"]
pub mod ptptslr;
#[doc = "PTPTSHUR (rw) register accessor: an alias for `Reg<PTPTSHUR_SPEC>`"]
pub type PTPTSHUR = crate::Reg<ptptshur::PTPTSHUR_SPEC>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "PTPTSLUR (rw) register accessor: an alias for `Reg<PTPTSLUR_SPEC>`"]
pub type PTPTSLUR = crate::Reg<ptptslur::PTPTSLUR_SPEC>;
#[doc = "Ethernet PTP time stamp low update register"]
pub mod ptptslur;
#[doc = "PTPTSAR (rw) register accessor: an alias for `Reg<PTPTSAR_SPEC>`"]
pub type PTPTSAR = crate::Reg<ptptsar::PTPTSAR_SPEC>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "PTPTTHR (rw) register accessor: an alias for `Reg<PTPTTHR_SPEC>`"]
pub type PTPTTHR = crate::Reg<ptptthr::PTPTTHR_SPEC>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "PTPTTLR (rw) register accessor: an alias for `Reg<PTPTTLR_SPEC>`"]
pub type PTPTTLR = crate::Reg<ptpttlr::PTPTTLR_SPEC>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
#[doc = "PTPTSSR (r) register accessor: an alias for `Reg<PTPTSSR_SPEC>`"]
pub type PTPTSSR = crate::Reg<ptptssr::PTPTSSR_SPEC>;
#[doc = "Ethernet PTP time stamp status register"]
pub mod ptptssr;
#[doc = "PTPPPSCR (r) register accessor: an alias for `Reg<PTPPPSCR_SPEC>`"]
pub type PTPPPSCR = crate::Reg<ptpppscr::PTPPPSCR_SPEC>;
#[doc = "Ethernet PTP PPS control register"]
pub mod ptpppscr;
