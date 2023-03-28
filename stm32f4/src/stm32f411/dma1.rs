#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: LISR,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: HISR,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: LIFCR,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: HIFCR,
    #[doc = "0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    pub st: [ST; 8],
}
#[doc = "LISR (r) register accessor: an alias for `Reg<LISR_SPEC>`"]
pub type LISR = crate::Reg<lisr::LISR_SPEC>;
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "HISR (r) register accessor: an alias for `Reg<HISR_SPEC>`"]
pub type HISR = crate::Reg<hisr::HISR_SPEC>;
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "LIFCR (w) register accessor: an alias for `Reg<LIFCR_SPEC>`"]
pub type LIFCR = crate::Reg<lifcr::LIFCR_SPEC>;
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "HIFCR (w) register accessor: an alias for `Reg<HIFCR_SPEC>`"]
pub type HIFCR = crate::Reg<hifcr::HIFCR_SPEC>;
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub use self::st::ST;
#[doc = r"Cluster"]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub mod st;
