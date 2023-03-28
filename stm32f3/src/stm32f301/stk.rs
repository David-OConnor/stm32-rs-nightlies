#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - SysTick reload value register"]
    pub load: LOAD,
    #[doc = "0x08 - SysTick current value register"]
    pub val: VAL,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SysTick control and status register"]
pub mod ctrl;
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "SysTick reload value register"]
pub mod load;
#[doc = "VAL (rw) register accessor: an alias for `Reg<VAL_SPEC>`"]
pub type VAL = crate::Reg<val::VAL_SPEC>;
#[doc = "SysTick current value register"]
pub mod val;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "SysTick calibration value register"]
pub mod calib;
