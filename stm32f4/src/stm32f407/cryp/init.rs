#[doc = r"Register block"]
#[repr(C)]
pub struct INIT {
    #[doc = "0x00 - initialization vector registers"]
    pub ivlr: IVLR,
    #[doc = "0x04 - initialization vector registers"]
    pub ivrr: IVRR,
}
#[doc = "IVLR (rw) register accessor: an alias for `Reg<IVLR_SPEC>`"]
pub type IVLR = crate::Reg<ivlr::IVLR_SPEC>;
#[doc = "initialization vector registers"]
pub mod ivlr;
#[doc = "IVRR (rw) register accessor: an alias for `Reg<IVRR_SPEC>`"]
pub type IVRR = crate::Reg<ivrr::IVRR_SPEC>;
#[doc = "initialization vector registers"]
pub mod ivrr;
