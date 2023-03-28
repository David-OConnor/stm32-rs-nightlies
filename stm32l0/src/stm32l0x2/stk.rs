#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - SysTick reload value register"]
    pub rvr: RVR,
    #[doc = "0x08 - SysTick current value register"]
    pub cvr: CVR,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "SysTick control and status register"]
pub mod csr;
#[doc = "RVR (rw) register accessor: an alias for `Reg<RVR_SPEC>`"]
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
#[doc = "SysTick reload value register"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "SysTick current value register"]
pub mod cvr;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "SysTick calibration value register"]
pub mod calib;
