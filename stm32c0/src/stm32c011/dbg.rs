#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBG device ID code register"]
    pub dbg_idcode: DBG_IDCODE,
    #[doc = "0x04 - DBG configuration register"]
    pub dbg_cr: DBG_CR,
    #[doc = "0x08 - DBG APB freeze register 1"]
    pub dbg_apb_fz1: DBG_APB_FZ1,
    #[doc = "0x0c - DBG APB freeze register 2"]
    pub dbg_apb_fz2: DBG_APB_FZ2,
}
#[doc = "DBG_IDCODE (r) register accessor: an alias for `Reg<DBG_IDCODE_SPEC>`"]
pub type DBG_IDCODE = crate::Reg<dbg_idcode::DBG_IDCODE_SPEC>;
#[doc = "DBG device ID code register"]
pub mod dbg_idcode;
#[doc = "DBG_CR (rw) register accessor: an alias for `Reg<DBG_CR_SPEC>`"]
pub type DBG_CR = crate::Reg<dbg_cr::DBG_CR_SPEC>;
#[doc = "DBG configuration register"]
pub mod dbg_cr;
#[doc = "DBG_APB_FZ1 (rw) register accessor: an alias for `Reg<DBG_APB_FZ1_SPEC>`"]
pub type DBG_APB_FZ1 = crate::Reg<dbg_apb_fz1::DBG_APB_FZ1_SPEC>;
#[doc = "DBG APB freeze register 1"]
pub mod dbg_apb_fz1;
#[doc = "DBG_APB_FZ2 (rw) register accessor: an alias for `Reg<DBG_APB_FZ2_SPEC>`"]
pub type DBG_APB_FZ2 = crate::Reg<dbg_apb_fz2::DBG_APB_FZ2_SPEC>;
#[doc = "DBG APB freeze register 2"]
pub mod dbg_apb_fz2;
