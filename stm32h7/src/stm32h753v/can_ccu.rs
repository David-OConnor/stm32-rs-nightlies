#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Calibration Unit Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - Calibration Configuration Register"]
    pub ccfg: CCFG,
    #[doc = "0x08 - Calibration Status Register"]
    pub cstat: CSTAT,
    #[doc = "0x0c - Calibration Watchdog Register"]
    pub cwd: CWD,
    #[doc = "0x10 - Clock Calibration Unit Interrupt Register"]
    pub ir: IR,
    #[doc = "0x14 - Clock Calibration Unit Interrupt Enable Register"]
    pub ie: IE,
}
#[doc = "CREL (rw) register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Clock Calibration Unit Core Release Register"]
pub mod crel;
#[doc = "CCFG (rw) register accessor: an alias for `Reg<CCFG_SPEC>`"]
pub type CCFG = crate::Reg<ccfg::CCFG_SPEC>;
#[doc = "Calibration Configuration Register"]
pub mod ccfg;
#[doc = "CSTAT (rw) register accessor: an alias for `Reg<CSTAT_SPEC>`"]
pub type CSTAT = crate::Reg<cstat::CSTAT_SPEC>;
#[doc = "Calibration Status Register"]
pub mod cstat;
#[doc = "CWD (rw) register accessor: an alias for `Reg<CWD_SPEC>`"]
pub type CWD = crate::Reg<cwd::CWD_SPEC>;
#[doc = "Calibration Watchdog Register"]
pub mod cwd;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Clock Calibration Unit Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Clock Calibration Unit Interrupt Enable Register"]
pub mod ie;
