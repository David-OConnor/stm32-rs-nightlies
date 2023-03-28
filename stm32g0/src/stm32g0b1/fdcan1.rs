#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN core release register"]
    pub crel: CREL,
    #[doc = "0x04 - FDCAN endian register"]
    pub endn: ENDN,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FDCAN data bit timing and prescaler register"]
    pub dbtp: DBTP,
    #[doc = "0x10 - FDCAN test register"]
    pub test: TEST,
    #[doc = "0x14 - FDCAN RAM watchdog register"]
    pub rwd: RWD,
    #[doc = "0x18 - FDCAN CC control register"]
    pub cccr: CCCR,
    #[doc = "0x1c - FDCAN nominal bit timing and prescaler register"]
    pub nbtp: NBTP,
    #[doc = "0x20 - FDCAN timestamp counter configuration register"]
    pub tscc: TSCC,
    #[doc = "0x24 - FDCAN timestamp counter value register"]
    pub tscv: TSCV,
    #[doc = "0x28 - FDCAN timeout counter configuration register"]
    pub tocc: TOCC,
    #[doc = "0x2c - FDCAN timeout counter value register"]
    pub tocv: TOCV,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - FDCAN error counter register"]
    pub ecr: ECR,
    #[doc = "0x44 - FDCAN protocol status register"]
    pub psr: PSR,
    #[doc = "0x48 - FDCAN transmitter delay compensation register"]
    pub tdcr: TDCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x50 - FDCAN interrupt register"]
    pub ir: IR,
    #[doc = "0x54 - FDCAN interrupt enable register"]
    pub ie: IE,
    #[doc = "0x58 - FDCAN interrupt line select register"]
    pub ils: ILS,
    #[doc = "0x5c - FDCAN interrupt line enable register"]
    pub ile: ILE,
    _reserved18: [u8; 0x20],
    #[doc = "0x80 - FDCAN global filter configuration register"]
    pub rxgfc: RXGFC,
    #[doc = "0x84 - FDCAN extended ID and mask register"]
    pub xidam: XIDAM,
    #[doc = "0x88 - FDCAN high-priority message status register"]
    pub hpms: HPMS,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - FDCAN Rx FIFO 0 status register"]
    pub rxf0s: RXF0S,
    #[doc = "0x94 - CAN Rx FIFO 0 acknowledge register"]
    pub rxf0a: RXF0A,
    #[doc = "0x98 - FDCAN Rx FIFO 1 status register"]
    pub rxf1s: RXF1S,
    #[doc = "0x9c - FDCAN Rx FIFO 1 acknowledge register"]
    pub rxf1a: RXF1A,
    _reserved25: [u8; 0x20],
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    pub txbc: TXBC,
    #[doc = "0xc4 - FDCAN Tx FIFO/queue status register"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - FDCAN Tx buffer request pending register"]
    pub txbrp: TXBRP,
    #[doc = "0xcc - FDCAN Tx buffer add request register"]
    pub txbar: TXBAR,
    #[doc = "0xd0 - FDCAN Tx buffer cancellation request register"]
    pub txbcr: TXBCR,
    #[doc = "0xd4 - FDCAN Tx buffer transmission occurred register"]
    pub txbto: TXBTO,
    #[doc = "0xd8 - FDCAN Tx buffer cancellation finished register"]
    pub txbcf: TXBCF,
    #[doc = "0xdc - FDCAN Tx buffer transmission interrupt enable register"]
    pub txbtie: TXBTIE,
    #[doc = "0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register"]
    pub txbcie: TXBCIE,
    #[doc = "0xe4 - FDCAN Tx event FIFO status register"]
    pub txefs: TXEFS,
    #[doc = "0xe8 - FDCAN Tx event FIFO acknowledge register"]
    pub txefa: TXEFA,
    _reserved36: [u8; 0x14],
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    pub ckdiv: CKDIV,
}
#[doc = "CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "FDCAN core release register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "FDCAN endian register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "FDCAN data bit timing and prescaler register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "FDCAN test register"]
pub mod test;
#[doc = "RWD (rw) register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "FDCAN RAM watchdog register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "FDCAN CC control register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "FDCAN nominal bit timing and prescaler register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "FDCAN timestamp counter configuration register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "FDCAN timestamp counter value register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "FDCAN timeout counter configuration register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "FDCAN timeout counter value register"]
pub mod tocv;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "FDCAN error counter register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "FDCAN protocol status register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "FDCAN transmitter delay compensation register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "FDCAN interrupt register"]
pub mod ir;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "FDCAN interrupt enable register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "FDCAN interrupt line select register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "FDCAN interrupt line enable register"]
pub mod ile;
#[doc = "RXGFC (rw) register accessor: an alias for `Reg<RXGFC_SPEC>`"]
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
#[doc = "FDCAN global filter configuration register"]
pub mod rxgfc;
#[doc = "XIDAM (rw) register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "FDCAN extended ID and mask register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "FDCAN high-priority message status register"]
pub mod hpms;
#[doc = "RXF0S (r) register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "FDCAN Rx FIFO 0 status register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "CAN Rx FIFO 0 acknowledge register"]
pub mod rxf0a;
#[doc = "RXF1S (r) register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "FDCAN Rx FIFO 1 status register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
pub mod rxf1a;
#[doc = "TXBC (rw) register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "FDCAN Tx FIFO/queue status register"]
pub mod txfqs;
#[doc = "TXBRP (r) register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "FDCAN Tx buffer request pending register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "FDCAN Tx buffer add request register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "FDCAN Tx buffer cancellation request register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "FDCAN Tx buffer transmission occurred register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "FDCAN Tx buffer cancellation finished register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
pub mod txbcie;
#[doc = "TXEFS (r) register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "FDCAN Tx event FIFO status register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "FDCAN Tx event FIFO acknowledge register"]
pub mod txefa;
#[doc = "CKDIV (rw) register accessor: an alias for `Reg<CKDIV_SPEC>`"]
pub type CKDIV = crate::Reg<ckdiv::CKDIV_SPEC>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
