#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub fdcan_crel: FDCAN_CREL,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub fdcan_endn: FDCAN_ENDN,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    pub fdcan_dbtp: FDCAN_DBTP,
    #[doc = "0x10 - FDCAN Test Register"]
    pub fdcan_test: FDCAN_TEST,
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    pub fdcan_rwd: FDCAN_RWD,
    #[doc = "0x18 - FDCAN CC Control Register"]
    pub fdcan_cccr: FDCAN_CCCR,
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    pub fdcan_nbtp: FDCAN_NBTP,
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    pub fdcan_tscc: FDCAN_TSCC,
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    pub fdcan_tscv: FDCAN_TSCV,
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    pub fdcan_tocc: FDCAN_TOCC,
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    pub fdcan_tocv: FDCAN_TOCV,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - FDCAN Error Counter Register"]
    pub fdcan_ecr: FDCAN_ECR,
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    pub fdcan_psr: FDCAN_PSR,
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    pub fdcan_tdcr: FDCAN_TDCR,
    _reserved14: [u8; 0x04],
    #[doc = "0x50 - FDCAN Interrupt Register"]
    pub fdcan_ir: FDCAN_IR,
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    pub fdcan_ie: FDCAN_IE,
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    pub fdcan_ils: FDCAN_ILS,
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    pub fdcan_ile: FDCAN_ILE,
    _reserved18: [u8; 0x20],
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    pub fdcan_gfc: FDCAN_GFC,
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    pub fdcan_sidfc: FDCAN_SIDFC,
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    pub fdcan_xidfc: FDCAN_XIDFC,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    pub fdcan_xidam: FDCAN_XIDAM,
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    pub fdcan_hpms: FDCAN_HPMS,
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    pub fdcan_ndat1: FDCAN_NDAT1,
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    pub fdcan_ndat2: FDCAN_NDAT2,
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    pub fdcan_rxf0c: FDCAN_RXF0C,
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    pub fdcan_rxf0s: FDCAN_RXF0S,
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    pub fdcan_rxf0a: FDCAN_RXF0A,
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    pub fdcan_rxbc: FDCAN_RXBC,
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    pub fdcan_rxf1c: FDCAN_RXF1C,
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    pub fdcan_rxf1s: FDCAN_RXF1S,
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub fdcan_rxf1a: FDCAN_RXF1A,
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    pub fdcan_rxesc: FDCAN_RXESC,
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub fdcan_txbc: FDCAN_TXBC,
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    pub fdcan_txfqs: FDCAN_TXFQS,
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    pub fdcan_txesc: FDCAN_TXESC,
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    pub fdcan_txbrp: FDCAN_TXBRP,
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    pub fdcan_txbar: FDCAN_TXBAR,
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    pub fdcan_txbcr: FDCAN_TXBCR,
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub fdcan_txbto: FDCAN_TXBTO,
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    pub fdcan_txbcf: FDCAN_TXBCF,
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub fdcan_txbtie: FDCAN_TXBTIE,
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub fdcan_txbcie: FDCAN_TXBCIE,
    _reserved43: [u8; 0x08],
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    pub fdcan_txefc: FDCAN_TXEFC,
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    pub fdcan_txefs: FDCAN_TXEFS,
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub fdcan_txefa: FDCAN_TXEFA,
    _reserved46: [u8; 0x04],
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    pub fdcan_tttmc: FDCAN_TTTMC,
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    pub fdcan_ttrmc: FDCAN_TTRMC,
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    pub fdcan_ttocf: FDCAN_TTOCF,
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    pub fdcan_ttmlm: FDCAN_TTMLM,
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    pub fdcan_turcf: FDCAN_TURCF,
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    pub fdcan_ttocn: FDCAN_TTOCN,
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    pub can_ttgtp: CAN_TTGTP,
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    pub fdcan_tttmk: FDCAN_TTTMK,
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    pub fdcan_ttir: FDCAN_TTIR,
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    pub fdcan_ttie: FDCAN_TTIE,
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    pub fdcan_ttils: FDCAN_TTILS,
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    pub fdcan_ttost: FDCAN_TTOST,
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    pub fdcan_turna: FDCAN_TURNA,
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    pub fdcan_ttlgt: FDCAN_TTLGT,
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    pub fdcan_ttctc: FDCAN_TTCTC,
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    pub fdcan_ttcpt: FDCAN_TTCPT,
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    pub fdcan_ttcsm: FDCAN_TTCSM,
    _reserved63: [u8; 0x01bc],
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    pub fdcan_ttts: FDCAN_TTTS,
}
#[doc = "FDCAN_CREL (r) register accessor: an alias for `Reg<FDCAN_CREL_SPEC>`"]
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CREL_SPEC>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_crel;
#[doc = "FDCAN_ENDN (r) register accessor: an alias for `Reg<FDCAN_ENDN_SPEC>`"]
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDN_SPEC>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_endn;
#[doc = "FDCAN_DBTP (r) register accessor: an alias for `Reg<FDCAN_DBTP_SPEC>`"]
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTP_SPEC>;
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (r) register accessor: an alias for `Reg<FDCAN_TEST_SPEC>`"]
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TEST_SPEC>;
#[doc = "FDCAN Test Register"]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (r) register accessor: an alias for `Reg<FDCAN_RWD_SPEC>`"]
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWD_SPEC>;
#[doc = "FDCAN RAM Watchdog Register"]
pub mod fdcan_rwd;
#[doc = "FDCAN_CCCR (rw) register accessor: an alias for `Reg<FDCAN_CCCR_SPEC>`"]
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCR_SPEC>;
#[doc = "FDCAN CC Control Register"]
pub mod fdcan_cccr;
#[doc = "FDCAN_NBTP (rw) register accessor: an alias for `Reg<FDCAN_NBTP_SPEC>`"]
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTP_SPEC>;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod fdcan_nbtp;
#[doc = "FDCAN_TSCC (rw) register accessor: an alias for `Reg<FDCAN_TSCC_SPEC>`"]
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCC_SPEC>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod fdcan_tscc;
#[doc = "FDCAN_TSCV (rw) register accessor: an alias for `Reg<FDCAN_TSCV_SPEC>`"]
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCV_SPEC>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod fdcan_tscv;
#[doc = "FDCAN_TOCC (rw) register accessor: an alias for `Reg<FDCAN_TOCC_SPEC>`"]
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCC_SPEC>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod fdcan_tocc;
#[doc = "FDCAN_TOCV (rw) register accessor: an alias for `Reg<FDCAN_TOCV_SPEC>`"]
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCV_SPEC>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod fdcan_tocv;
#[doc = "FDCAN_ECR (rw) register accessor: an alias for `Reg<FDCAN_ECR_SPEC>`"]
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECR_SPEC>;
#[doc = "FDCAN Error Counter Register"]
pub mod fdcan_ecr;
#[doc = "FDCAN_PSR (rw) register accessor: an alias for `Reg<FDCAN_PSR_SPEC>`"]
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSR_SPEC>;
#[doc = "FDCAN Protocol Status Register"]
pub mod fdcan_psr;
#[doc = "FDCAN_TDCR (r) register accessor: an alias for `Reg<FDCAN_TDCR_SPEC>`"]
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCR_SPEC>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (r) register accessor: an alias for `Reg<FDCAN_IR_SPEC>`"]
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IR_SPEC>;
#[doc = "FDCAN Interrupt Register"]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (rw) register accessor: an alias for `Reg<FDCAN_IE_SPEC>`"]
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IE_SPEC>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (r) register accessor: an alias for `Reg<FDCAN_ILS_SPEC>`"]
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILS_SPEC>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: an alias for `Reg<FDCAN_ILE_SPEC>`"]
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILE_SPEC>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod fdcan_ile;
#[doc = "FDCAN_GFC (rw) register accessor: an alias for `Reg<FDCAN_GFC_SPEC>`"]
pub type FDCAN_GFC = crate::Reg<fdcan_gfc::FDCAN_GFC_SPEC>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod fdcan_gfc;
#[doc = "FDCAN_SIDFC (rw) register accessor: an alias for `Reg<FDCAN_SIDFC_SPEC>`"]
pub type FDCAN_SIDFC = crate::Reg<fdcan_sidfc::FDCAN_SIDFC_SPEC>;
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod fdcan_sidfc;
#[doc = "FDCAN_XIDFC (rw) register accessor: an alias for `Reg<FDCAN_XIDFC_SPEC>`"]
pub type FDCAN_XIDFC = crate::Reg<fdcan_xidfc::FDCAN_XIDFC_SPEC>;
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod fdcan_xidfc;
#[doc = "FDCAN_XIDAM (rw) register accessor: an alias for `Reg<FDCAN_XIDAM_SPEC>`"]
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAM_SPEC>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod fdcan_xidam;
#[doc = "FDCAN_HPMS (r) register accessor: an alias for `Reg<FDCAN_HPMS_SPEC>`"]
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMS_SPEC>;
#[doc = "FDCAN High Priority Message Status Register"]
pub mod fdcan_hpms;
#[doc = "FDCAN_NDAT1 (r) register accessor: an alias for `Reg<FDCAN_NDAT1_SPEC>`"]
pub type FDCAN_NDAT1 = crate::Reg<fdcan_ndat1::FDCAN_NDAT1_SPEC>;
#[doc = "FDCAN New Data 1 Register"]
pub mod fdcan_ndat1;
#[doc = "FDCAN_NDAT2 (r) register accessor: an alias for `Reg<FDCAN_NDAT2_SPEC>`"]
pub type FDCAN_NDAT2 = crate::Reg<fdcan_ndat2::FDCAN_NDAT2_SPEC>;
#[doc = "FDCAN New Data 2 Register"]
pub mod fdcan_ndat2;
#[doc = "FDCAN_RXF0C (rw) register accessor: an alias for `Reg<FDCAN_RXF0C_SPEC>`"]
pub type FDCAN_RXF0C = crate::Reg<fdcan_rxf0c::FDCAN_RXF0C_SPEC>;
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod fdcan_rxf0c;
#[doc = "FDCAN_RXF0S (rw) register accessor: an alias for `Reg<FDCAN_RXF0S_SPEC>`"]
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0S_SPEC>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod fdcan_rxf0s;
#[doc = "FDCAN_RXF0A (rw) register accessor: an alias for `Reg<FDCAN_RXF0A_SPEC>`"]
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0A_SPEC>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN_RXBC (rw) register accessor: an alias for `Reg<FDCAN_RXBC_SPEC>`"]
pub type FDCAN_RXBC = crate::Reg<fdcan_rxbc::FDCAN_RXBC_SPEC>;
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod fdcan_rxbc;
#[doc = "FDCAN_RXF1C (rw) register accessor: an alias for `Reg<FDCAN_RXF1C_SPEC>`"]
pub type FDCAN_RXF1C = crate::Reg<fdcan_rxf1c::FDCAN_RXF1C_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod fdcan_rxf1c;
#[doc = "FDCAN_RXF1S (rw) register accessor: an alias for `Reg<FDCAN_RXF1S_SPEC>`"]
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1S_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN_RXF1A (rw) register accessor: an alias for `Reg<FDCAN_RXF1A_SPEC>`"]
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1A_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN_RXESC (rw) register accessor: an alias for `Reg<FDCAN_RXESC_SPEC>`"]
pub type FDCAN_RXESC = crate::Reg<fdcan_rxesc::FDCAN_RXESC_SPEC>;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod fdcan_rxesc;
#[doc = "FDCAN_TXBC (rw) register accessor: an alias for `Reg<FDCAN_TXBC_SPEC>`"]
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBC_SPEC>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod fdcan_txbc;
#[doc = "FDCAN_TXFQS (r) register accessor: an alias for `Reg<FDCAN_TXFQS_SPEC>`"]
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQS_SPEC>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod fdcan_txfqs;
#[doc = "FDCAN_TXESC (rw) register accessor: an alias for `Reg<FDCAN_TXESC_SPEC>`"]
pub type FDCAN_TXESC = crate::Reg<fdcan_txesc::FDCAN_TXESC_SPEC>;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod fdcan_txesc;
#[doc = "FDCAN_TXBRP (r) register accessor: an alias for `Reg<FDCAN_TXBRP_SPEC>`"]
pub type FDCAN_TXBRP = crate::Reg<fdcan_txbrp::FDCAN_TXBRP_SPEC>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod fdcan_txbrp;
#[doc = "FDCAN_TXBAR (rw) register accessor: an alias for `Reg<FDCAN_TXBAR_SPEC>`"]
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBAR_SPEC>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod fdcan_txbar;
#[doc = "FDCAN_TXBCR (rw) register accessor: an alias for `Reg<FDCAN_TXBCR_SPEC>`"]
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCR_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN_TXBTO (rw) register accessor: an alias for `Reg<FDCAN_TXBTO_SPEC>`"]
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTO_SPEC>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod fdcan_txbto;
#[doc = "FDCAN_TXBCF (r) register accessor: an alias for `Reg<FDCAN_TXBCF_SPEC>`"]
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCF_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN_TXBTIE (rw) register accessor: an alias for `Reg<FDCAN_TXBTIE_SPEC>`"]
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIE_SPEC>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN_TXBCIE (rw) register accessor: an alias for `Reg<FDCAN_TXBCIE_SPEC>`"]
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIE_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN_TXEFC (rw) register accessor: an alias for `Reg<FDCAN_TXEFC_SPEC>`"]
pub type FDCAN_TXEFC = crate::Reg<fdcan_txefc::FDCAN_TXEFC_SPEC>;
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod fdcan_txefc;
#[doc = "FDCAN_TXEFS (rw) register accessor: an alias for `Reg<FDCAN_TXEFS_SPEC>`"]
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFS_SPEC>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: an alias for `Reg<FDCAN_TXEFA_SPEC>`"]
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFA_SPEC>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_TTTMC (rw) register accessor: an alias for `Reg<FDCAN_TTTMC_SPEC>`"]
pub type FDCAN_TTTMC = crate::Reg<fdcan_tttmc::FDCAN_TTTMC_SPEC>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod fdcan_tttmc;
#[doc = "FDCAN_TTRMC (rw) register accessor: an alias for `Reg<FDCAN_TTRMC_SPEC>`"]
pub type FDCAN_TTRMC = crate::Reg<fdcan_ttrmc::FDCAN_TTRMC_SPEC>;
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod fdcan_ttrmc;
#[doc = "FDCAN_TTOCF (rw) register accessor: an alias for `Reg<FDCAN_TTOCF_SPEC>`"]
pub type FDCAN_TTOCF = crate::Reg<fdcan_ttocf::FDCAN_TTOCF_SPEC>;
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod fdcan_ttocf;
#[doc = "FDCAN_TTMLM (rw) register accessor: an alias for `Reg<FDCAN_TTMLM_SPEC>`"]
pub type FDCAN_TTMLM = crate::Reg<fdcan_ttmlm::FDCAN_TTMLM_SPEC>;
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod fdcan_ttmlm;
#[doc = "FDCAN_TURCF (rw) register accessor: an alias for `Reg<FDCAN_TURCF_SPEC>`"]
pub type FDCAN_TURCF = crate::Reg<fdcan_turcf::FDCAN_TURCF_SPEC>;
#[doc = "FDCAN TUR Configuration Register"]
pub mod fdcan_turcf;
#[doc = "FDCAN_TTOCN (rw) register accessor: an alias for `Reg<FDCAN_TTOCN_SPEC>`"]
pub type FDCAN_TTOCN = crate::Reg<fdcan_ttocn::FDCAN_TTOCN_SPEC>;
#[doc = "FDCAN TT Operation Control Register"]
pub mod fdcan_ttocn;
#[doc = "CAN_TTGTP (rw) register accessor: an alias for `Reg<CAN_TTGTP_SPEC>`"]
pub type CAN_TTGTP = crate::Reg<can_ttgtp::CAN_TTGTP_SPEC>;
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod can_ttgtp;
#[doc = "FDCAN_TTTMK (rw) register accessor: an alias for `Reg<FDCAN_TTTMK_SPEC>`"]
pub type FDCAN_TTTMK = crate::Reg<fdcan_tttmk::FDCAN_TTTMK_SPEC>;
#[doc = "FDCAN TT Time Mark Register"]
pub mod fdcan_tttmk;
#[doc = "FDCAN_TTIR (rw) register accessor: an alias for `Reg<FDCAN_TTIR_SPEC>`"]
pub type FDCAN_TTIR = crate::Reg<fdcan_ttir::FDCAN_TTIR_SPEC>;
#[doc = "FDCAN TT Interrupt Register"]
pub mod fdcan_ttir;
#[doc = "FDCAN_TTIE (rw) register accessor: an alias for `Reg<FDCAN_TTIE_SPEC>`"]
pub type FDCAN_TTIE = crate::Reg<fdcan_ttie::FDCAN_TTIE_SPEC>;
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod fdcan_ttie;
#[doc = "FDCAN_TTILS (rw) register accessor: an alias for `Reg<FDCAN_TTILS_SPEC>`"]
pub type FDCAN_TTILS = crate::Reg<fdcan_ttils::FDCAN_TTILS_SPEC>;
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod fdcan_ttils;
#[doc = "FDCAN_TTOST (r) register accessor: an alias for `Reg<FDCAN_TTOST_SPEC>`"]
pub type FDCAN_TTOST = crate::Reg<fdcan_ttost::FDCAN_TTOST_SPEC>;
#[doc = "FDCAN TT Operation Status Register"]
pub mod fdcan_ttost;
#[doc = "FDCAN_TURNA (r) register accessor: an alias for `Reg<FDCAN_TURNA_SPEC>`"]
pub type FDCAN_TURNA = crate::Reg<fdcan_turna::FDCAN_TURNA_SPEC>;
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod fdcan_turna;
#[doc = "FDCAN_TTLGT (r) register accessor: an alias for `Reg<FDCAN_TTLGT_SPEC>`"]
pub type FDCAN_TTLGT = crate::Reg<fdcan_ttlgt::FDCAN_TTLGT_SPEC>;
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod fdcan_ttlgt;
#[doc = "FDCAN_TTCTC (r) register accessor: an alias for `Reg<FDCAN_TTCTC_SPEC>`"]
pub type FDCAN_TTCTC = crate::Reg<fdcan_ttctc::FDCAN_TTCTC_SPEC>;
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod fdcan_ttctc;
#[doc = "FDCAN_TTCPT (r) register accessor: an alias for `Reg<FDCAN_TTCPT_SPEC>`"]
pub type FDCAN_TTCPT = crate::Reg<fdcan_ttcpt::FDCAN_TTCPT_SPEC>;
#[doc = "FDCAN TT Capture Time Register"]
pub mod fdcan_ttcpt;
#[doc = "FDCAN_TTCSM (r) register accessor: an alias for `Reg<FDCAN_TTCSM_SPEC>`"]
pub type FDCAN_TTCSM = crate::Reg<fdcan_ttcsm::FDCAN_TTCSM_SPEC>;
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod fdcan_ttcsm;
#[doc = "FDCAN_TTTS (rw) register accessor: an alias for `Reg<FDCAN_TTTS_SPEC>`"]
pub type FDCAN_TTTS = crate::Reg<fdcan_ttts::FDCAN_TTTS_SPEC>;
#[doc = "FDCAN TT Trigger Select Register"]
pub mod fdcan_ttts;
