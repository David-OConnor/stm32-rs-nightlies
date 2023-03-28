#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode Register"]
    pub mtlomr: MTLOMR,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Interrupt status Register"]
    pub mtlisr: MTLISR,
    _reserved2: [u8; 0xdc],
    #[doc = "0x100 - Tx queue operating mode Register"]
    pub mtltx_qomr: MTLTX_QOMR,
    #[doc = "0x104 - Tx queue underflow register"]
    pub mtltx_qur: MTLTX_QUR,
    #[doc = "0x108 - Tx queue debug Register"]
    pub mtltx_qdr: MTLTX_QDR,
    _reserved5: [u8; 0x20],
    #[doc = "0x12c - Queue interrupt control status Register"]
    pub mtlqicsr: MTLQICSR,
    #[doc = "0x130 - Rx queue operating mode register"]
    pub mtlrx_qomr: MTLRX_QOMR,
    #[doc = "0x134 - Rx queue missed packet and overflow counter register"]
    pub mtlrx_qmpocr: MTLRX_QMPOCR,
    #[doc = "0x138 - Rx queue debug register"]
    pub mtlrx_qdr: MTLRX_QDR,
}
#[doc = "MTLOMR (rw) register accessor: an alias for `Reg<MTLOMR_SPEC>`"]
pub type MTLOMR = crate::Reg<mtlomr::MTLOMR_SPEC>;
#[doc = "Operating mode Register"]
pub mod mtlomr;
#[doc = "MTLISR (r) register accessor: an alias for `Reg<MTLISR_SPEC>`"]
pub type MTLISR = crate::Reg<mtlisr::MTLISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod mtlisr;
#[doc = "MTLTxQOMR (rw) register accessor: an alias for `Reg<MTLTX_QOMR_SPEC>`"]
pub type MTLTX_QOMR = crate::Reg<mtltx_qomr::MTLTX_QOMR_SPEC>;
#[doc = "Tx queue operating mode Register"]
pub mod mtltx_qomr;
#[doc = "MTLTxQUR (r) register accessor: an alias for `Reg<MTLTX_QUR_SPEC>`"]
pub type MTLTX_QUR = crate::Reg<mtltx_qur::MTLTX_QUR_SPEC>;
#[doc = "Tx queue underflow register"]
pub mod mtltx_qur;
#[doc = "MTLTxQDR (r) register accessor: an alias for `Reg<MTLTX_QDR_SPEC>`"]
pub type MTLTX_QDR = crate::Reg<mtltx_qdr::MTLTX_QDR_SPEC>;
#[doc = "Tx queue debug Register"]
pub mod mtltx_qdr;
#[doc = "MTLQICSR (rw) register accessor: an alias for `Reg<MTLQICSR_SPEC>`"]
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSR_SPEC>;
#[doc = "Queue interrupt control status Register"]
pub mod mtlqicsr;
#[doc = "MTLRxQOMR (rw) register accessor: an alias for `Reg<MTLRX_QOMR_SPEC>`"]
pub type MTLRX_QOMR = crate::Reg<mtlrx_qomr::MTLRX_QOMR_SPEC>;
#[doc = "Rx queue operating mode register"]
pub mod mtlrx_qomr;
#[doc = "MTLRxQMPOCR (r) register accessor: an alias for `Reg<MTLRX_QMPOCR_SPEC>`"]
pub type MTLRX_QMPOCR = crate::Reg<mtlrx_qmpocr::MTLRX_QMPOCR_SPEC>;
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod mtlrx_qmpocr;
#[doc = "MTLRxQDR (r) register accessor: an alias for `Reg<MTLRX_QDR_SPEC>`"]
pub type MTLRX_QDR = crate::Reg<mtlrx_qdr::MTLRX_QDR_SPEC>;
#[doc = "Rx queue debug register"]
pub mod mtlrx_qdr;
