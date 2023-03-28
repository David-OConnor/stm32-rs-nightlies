#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x04 - OPAMP2 control/status register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x08 - OPAMP3 control/status register"]
    pub opamp3_csr: OPAMP3_CSR,
    #[doc = "0x0c - OPAMP4 control/status register"]
    pub opamp4_csr: OPAMP4_CSR,
    #[doc = "0x10 - OPAMP5 control/status register"]
    pub opamp5_csr: OPAMP5_CSR,
    #[doc = "0x14 - OPAMP6 control/status register"]
    pub opamp6_csr: OPAMP6_CSR,
    #[doc = "0x18 - OPAMP1 control/status register"]
    pub opamp1_tcmr: OPAMP1_TCMR,
    #[doc = "0x1c - OPAMP2 control/status register"]
    pub opamp2_tcmr: OPAMP2_TCMR,
    #[doc = "0x20 - OPAMP3 control/status register"]
    pub opamp3_tcmr: OPAMP3_TCMR,
    #[doc = "0x24 - OPAMP4 control/status register"]
    pub opamp4_tcmr: OPAMP4_TCMR,
    #[doc = "0x28 - OPAMP5 control/status register"]
    pub opamp5_tcmr: OPAMP5_TCMR,
    #[doc = "0x2c - OPAMP6 control/status register"]
    pub opamp6_tcmr: OPAMP6_TCMR,
}
#[doc = "OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: an alias for `Reg<OPAMP3_CSR_SPEC>`"]
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSR_SPEC>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_csr;
#[doc = "OPAMP4_CSR (rw) register accessor: an alias for `Reg<OPAMP4_CSR_SPEC>`"]
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSR_SPEC>;
#[doc = "OPAMP4 control/status register"]
pub mod opamp4_csr;
#[doc = "OPAMP5_CSR (rw) register accessor: an alias for `Reg<OPAMP5_CSR_SPEC>`"]
pub type OPAMP5_CSR = crate::Reg<opamp5_csr::OPAMP5_CSR_SPEC>;
#[doc = "OPAMP5 control/status register"]
pub mod opamp5_csr;
#[doc = "OPAMP6_CSR (rw) register accessor: an alias for `Reg<OPAMP6_CSR_SPEC>`"]
pub type OPAMP6_CSR = crate::Reg<opamp6_csr::OPAMP6_CSR_SPEC>;
#[doc = "OPAMP6 control/status register"]
pub mod opamp6_csr;
#[doc = "OPAMP1_TCMR (rw) register accessor: an alias for `Reg<OPAMP1_TCMR_SPEC>`"]
pub type OPAMP1_TCMR = crate::Reg<opamp1_tcmr::OPAMP1_TCMR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_tcmr;
#[doc = "OPAMP2_TCMR (rw) register accessor: an alias for `Reg<OPAMP2_TCMR_SPEC>`"]
pub type OPAMP2_TCMR = crate::Reg<opamp2_tcmr::OPAMP2_TCMR_SPEC>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_tcmr;
#[doc = "OPAMP3_TCMR (rw) register accessor: an alias for `Reg<OPAMP3_TCMR_SPEC>`"]
pub type OPAMP3_TCMR = crate::Reg<opamp3_tcmr::OPAMP3_TCMR_SPEC>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_tcmr;
#[doc = "OPAMP4_TCMR (rw) register accessor: an alias for `Reg<OPAMP4_TCMR_SPEC>`"]
pub type OPAMP4_TCMR = crate::Reg<opamp4_tcmr::OPAMP4_TCMR_SPEC>;
#[doc = "OPAMP4 control/status register"]
pub mod opamp4_tcmr;
#[doc = "OPAMP5_TCMR (rw) register accessor: an alias for `Reg<OPAMP5_TCMR_SPEC>`"]
pub type OPAMP5_TCMR = crate::Reg<opamp5_tcmr::OPAMP5_TCMR_SPEC>;
#[doc = "OPAMP5 control/status register"]
pub mod opamp5_tcmr;
#[doc = "OPAMP6_TCMR (rw) register accessor: an alias for `Reg<OPAMP6_TCMR_SPEC>`"]
pub type OPAMP6_TCMR = crate::Reg<opamp6_tcmr::OPAMP6_TCMR_SPEC>;
#[doc = "OPAMP6 control/status register"]
pub mod opamp6_tcmr;
