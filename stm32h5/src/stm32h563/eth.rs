#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode configuration register"]
    pub eth_maccr: ETH_MACCR,
    #[doc = "0x04 - Extended operating mode configuration register"]
    pub eth_macecr: ETH_MACECR,
    #[doc = "0x08 - Packet filtering control register"]
    pub eth_macpfr: ETH_MACPFR,
    #[doc = "0x0c - Watchdog timeout register"]
    pub eth_macwtr: ETH_MACWTR,
    #[doc = "0x10 - Hash Table 0 register"]
    pub eth_macht0r: ETH_MACHT0R,
    #[doc = "0x14 - Hash Table 1 register"]
    pub eth_macht1r: ETH_MACHT1R,
    _reserved6: [u8; 0x38],
    #[doc = "0x50 - VLAN tag register"]
    pub eth_macvtr: ETH_MACVTR,
    _reserved7: [u8; 0x04],
    #[doc = "0x58 - VLAN Hash table register"]
    pub eth_macvhtr: ETH_MACVHTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x60 - VLAN inclusion register"]
    pub eth_macvir: ETH_MACVIR,
    #[doc = "0x64 - Inner VLAN inclusion register"]
    pub eth_macivir: ETH_MACIVIR,
    _reserved10: [u8; 0x08],
    #[doc = "0x70 - Tx Queue flow control register"]
    pub eth_macqtxfcr: ETH_MACQTXFCR,
    _reserved11: [u8; 0x1c],
    #[doc = "0x90 - Rx flow control register"]
    pub eth_macrxfcr: ETH_MACRXFCR,
    _reserved12: [u8; 0x1c],
    #[doc = "0xb0 - Interrupt status register"]
    pub eth_macisr: ETH_MACISR,
    #[doc = "0xb4 - Interrupt enable register"]
    pub eth_macier: ETH_MACIER,
    #[doc = "0xb8 - Rx Tx status register"]
    pub eth_macrxtxsr: ETH_MACRXTXSR,
    _reserved15: [u8; 0x04],
    #[doc = "0xc0 - PMT control status register"]
    pub eth_macpcsr: ETH_MACPCSR,
    #[doc = "0xc4 - Remote wakeup packet filter register"]
    pub eth_macrwkpfr: ETH_MACRWKPFR,
    _reserved17: [u8; 0x08],
    #[doc = "0xd0 - LPI control and status register"]
    pub eth_maclcsr: ETH_MACLCSR,
    #[doc = "0xd4 - LPI timers control register"]
    pub eth_macltcr: ETH_MACLTCR,
    #[doc = "0xd8 - LPI entry timer register"]
    pub eth_macletr: ETH_MACLETR,
    #[doc = "0xdc - One-microsecond-tick counter register"]
    pub eth_mac1ustcr: ETH_MAC1USTCR,
    _reserved21: [u8; 0x30],
    #[doc = "0x110 - Version register"]
    pub eth_macvr: ETH_MACVR,
    #[doc = "0x114 - Debug register"]
    pub eth_macdr: ETH_MACDR,
    _reserved23: [u8; 0x04],
    #[doc = "0x11c - HW feature 0 register"]
    pub eth_machwf0r: ETH_MACHWF0R,
    #[doc = "0x120 - HW feature 1 register"]
    pub eth_machwf1r: ETH_MACHWF1R,
    #[doc = "0x124 - HW feature 2 register"]
    pub eth_machwf2r: ETH_MACHWF2R,
    #[doc = "0x128 - HW feature 3 register"]
    pub eth_machwf3r: ETH_MACHWF3R,
    _reserved27: [u8; 0xd4],
    #[doc = "0x200 - MDIO address register"]
    pub eth_macmdioar: ETH_MACMDIOAR,
    #[doc = "0x204 - MDIO data register"]
    pub eth_macmdiodr: ETH_MACMDIODR,
    _reserved29: [u8; 0x08],
    #[doc = "0x210 - ARP address register"]
    pub eth_macarpar: ETH_MACARPAR,
    _reserved30: [u8; 0x1c],
    #[doc = "0x230 - CSR software control register"]
    pub eth_maccsrswcr: ETH_MACCSRSWCR,
    _reserved31: [u8; 0xcc],
    #[doc = "0x300 - MAC Address 0 high register"]
    pub eth_maca0hr: ETH_MACA0HR,
    #[doc = "0x304 - MAC Address 0 low register"]
    pub eth_maca0lr: ETH_MACA0LR,
    #[doc = "0x308 - MAC Address 1 high register"]
    pub eth_maca1hr: ETH_MACA1HR,
    #[doc = "0x30c - MAC Address 1 low register"]
    pub eth_maca1lr: ETH_MACA1LR,
    #[doc = "0x310 - MAC Address 2 high register"]
    pub eth_maca2hr: ETH_MACA2HR,
    #[doc = "0x314 - MAC Address 2 low register"]
    pub eth_maca2lr: ETH_MACA2LR,
    #[doc = "0x318 - MAC Address 3 high register"]
    pub eth_maca3hr: ETH_MACA3HR,
    #[doc = "0x31c - MAC Address 3 low register"]
    pub eth_maca3lr: ETH_MACA3LR,
    _reserved39: [u8; 0x03e0],
    #[doc = "0x700 - MMC control register"]
    pub eth_mmc_control: ETH_MMC_CONTROL,
    #[doc = "0x704 - MMC Rx interrupt register"]
    pub eth_mmc_rx_interrupt: ETH_MMC_RX_INTERRUPT,
    #[doc = "0x708 - MMC Tx interrupt register"]
    pub eth_mmc_tx_interrupt: ETH_MMC_TX_INTERRUPT,
    #[doc = "0x70c - MMC Rx interrupt mask register"]
    pub eth_mmc_rx_interrupt_mask: ETH_MMC_RX_INTERRUPT_MASK,
    #[doc = "0x710 - MMC Tx interrupt mask register"]
    pub eth_mmc_tx_interrupt_mask: ETH_MMC_TX_INTERRUPT_MASK,
    _reserved44: [u8; 0x38],
    #[doc = "0x74c - Tx single collision good packets register"]
    pub eth_tx_single_collision_good_packets: ETH_TX_SINGLE_COLLISION_GOOD_PACKETS,
    #[doc = "0x750 - Tx multiple collision good packets register"]
    pub eth_tx_multiple_collision_good_packets: ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved46: [u8; 0x14],
    #[doc = "0x768 - Tx packet count good register"]
    pub eth_tx_packet_count_good: ETH_TX_PACKET_COUNT_GOOD,
    _reserved47: [u8; 0x28],
    #[doc = "0x794 - Rx CRC error packets register"]
    pub eth_rx_crc_error_packets: ETH_RX_CRC_ERROR_PACKETS,
    #[doc = "0x798 - Rx alignment error packets register"]
    pub eth_rx_alignment_error_packets: ETH_RX_ALIGNMENT_ERROR_PACKETS,
    _reserved49: [u8; 0x28],
    #[doc = "0x7c4 - Rx unicast packets good register"]
    pub eth_rx_unicast_packets_good: ETH_RX_UNICAST_PACKETS_GOOD,
    _reserved50: [u8; 0x24],
    #[doc = "0x7ec - Tx LPI microsecond timer register"]
    pub eth_tx_lpi_usec_cntr: ETH_TX_LPI_USEC_CNTR,
    #[doc = "0x7f0 - Tx LPI transition counter register"]
    pub eth_tx_lpi_tran_cntr: ETH_TX_LPI_TRAN_CNTR,
    #[doc = "0x7f4 - Rx LPI microsecond counter register"]
    pub eth_rx_lpi_usec_cntr: ETH_RX_LPI_USEC_CNTR,
    #[doc = "0x7f8 - Rx LPI transition counter register"]
    pub eth_rx_lpi_tran_cntr: ETH_RX_LPI_TRAN_CNTR,
    _reserved54: [u8; 0x0104],
    #[doc = "0x900 - L3 and L4 control 0 register"]
    pub eth_macl3l4c0r: ETH_MACL3L4C0R,
    #[doc = "0x904 - Layer4 Address filter 0 register"]
    pub eth_macl4a0r: ETH_MACL4A0R,
    _reserved56: [u8; 0x08],
    #[doc = "0x910 - Layer3 Address 0 filter 0 register"]
    pub eth_macl3a00r: ETH_MACL3A00R,
    #[doc = "0x914 - Layer3 Address 1 filter 0 register"]
    pub eth_macl3a10r: ETH_MACL3A10R,
    #[doc = "0x918 - Layer3 Address 2 filter 0 register"]
    pub eth_macl3a20r: ETH_MACL3A20R,
    #[doc = "0x91c - Layer3 Address 3 filter 0 register"]
    pub eth_macl3a30r: ETH_MACL3A30R,
    _reserved60: [u8; 0x10],
    #[doc = "0x930 - L3 and L4 control 1 register"]
    pub eth_macl3l4c1r: ETH_MACL3L4C1R,
    #[doc = "0x934 - Layer 4 address filter 1 register"]
    pub eth_macl4a1r: ETH_MACL4A1R,
    _reserved62: [u8; 0x08],
    #[doc = "0x940 - Layer3 address 0 filter 1 Register"]
    pub eth_macl3a01r: ETH_MACL3A01R,
    #[doc = "0x944 - Layer3 address 1 filter 1 register"]
    pub eth_macl3a11r: ETH_MACL3A11R,
    #[doc = "0x948 - Layer3 address 2 filter 1 Register"]
    pub eth_macl3a21r: ETH_MACL3A21R,
    #[doc = "0x94c - Layer3 address 3 filter 1 register"]
    pub eth_macl3a31r: ETH_MACL3A31R,
    _reserved66: [u8; 0x01b0],
    #[doc = "0xb00 - Timestamp control Register"]
    pub eth_mactscr: ETH_MACTSCR,
    #[doc = "0xb04 - Subsecond increment register"]
    pub eth_macssir: ETH_MACSSIR,
    #[doc = "0xb08 - System time seconds register"]
    pub eth_macstsr: ETH_MACSTSR,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub eth_macstnr: ETH_MACSTNR,
    #[doc = "0xb10 - System time seconds update register"]
    pub eth_macstsur: ETH_MACSTSUR,
    #[doc = "0xb14 - System time nanoseconds update register"]
    pub eth_macstnur: ETH_MACSTNUR,
    #[doc = "0xb18 - Timestamp addend register"]
    pub eth_mactsar: ETH_MACTSAR,
    _reserved73: [u8; 0x04],
    #[doc = "0xb20 - Timestamp status register"]
    pub eth_mactssr: ETH_MACTSSR,
    _reserved74: [u8; 0x0c],
    #[doc = "0xb30 - Tx timestamp status nanoseconds register"]
    pub eth_mactxtssnr: ETH_MACTXTSSNR,
    #[doc = "0xb34 - Tx timestamp status seconds register"]
    pub eth_mactxtsssr: ETH_MACTXTSSSR,
    _reserved76: [u8; 0x08],
    #[doc = "0xb40 - Auxiliary control register"]
    pub eth_macacr: ETH_MACACR,
    _reserved77: [u8; 0x04],
    #[doc = "0xb48 - Auxiliary timestamp nanoseconds register"]
    pub eth_macatsnr: ETH_MACATSNR,
    #[doc = "0xb4c - Auxiliary timestamp seconds register"]
    pub eth_macatssr: ETH_MACATSSR,
    #[doc = "0xb50 - Timestamp Ingress asymmetric correction register"]
    pub eth_mactsiacr: ETH_MACTSIACR,
    #[doc = "0xb54 - Timestamp Egress asymmetric correction register"]
    pub eth_mactseacr: ETH_MACTSEACR,
    #[doc = "0xb58 - Timestamp Ingress correction nanosecond register"]
    pub eth_mactsicnr: ETH_MACTSICNR,
    #[doc = "0xb5c - Timestamp Egress correction nanosecond register"]
    pub eth_mactsecnr: ETH_MACTSECNR,
    _reserved83: [u8; 0x10],
    _reserved_83_eth_: [u8; 0x04],
    _reserved84: [u8; 0x0c],
    #[doc = "0xb80 - PPS target time seconds register"]
    pub eth_macppsttsr: ETH_MACPPSTTSR,
    #[doc = "0xb84 - PPS target time nanoseconds register"]
    pub eth_macppsttnr: ETH_MACPPSTTNR,
    #[doc = "0xb88 - PPS interval register"]
    pub eth_macppsir: ETH_MACPPSIR,
    #[doc = "0xb8c - PPS width register"]
    pub eth_macppswr: ETH_MACPPSWR,
    _reserved88: [u8; 0x30],
    #[doc = "0xbc0 - PTP Offload control register"]
    pub eth_macpocr: ETH_MACPOCR,
    #[doc = "0xbc4 - PTP Source Port Identity 0 Register"]
    pub eth_macspi0r: ETH_MACSPI0R,
    #[doc = "0xbc8 - PTP Source port identity 1 register"]
    pub eth_macspi1r: ETH_MACSPI1R,
    #[doc = "0xbcc - PTP Source port identity 2 register"]
    pub eth_macspi2r: ETH_MACSPI2R,
    #[doc = "0xbd0 - Log message interval register"]
    pub eth_maclmir: ETH_MACLMIR,
    _reserved93: [u8; 0x2c],
    #[doc = "0xc00 - Operating mode Register"]
    pub eth_mtlomr: ETH_MTLOMR,
    _reserved94: [u8; 0x1c],
    #[doc = "0xc20 - Interrupt status Register"]
    pub eth_mtlisr: ETH_MTLISR,
    _reserved95: [u8; 0xdc],
    #[doc = "0xd00 - Tx queue operating mode Register"]
    pub eth_mtltxqomr: ETH_MTLTXQOMR,
    #[doc = "0xd04 - Tx queue underflow register"]
    pub eth_mtltxqur: ETH_MTLTXQUR,
    #[doc = "0xd08 - Tx queue debug register"]
    pub eth_mtltxqdr: ETH_MTLTXQDR,
    _reserved98: [u8; 0x20],
    #[doc = "0xd2c - Queue interrupt control status Register"]
    pub eth_mtlqicsr: ETH_MTLQICSR,
    #[doc = "0xd30 - Rx queue operating mode register"]
    pub eth_mtlrxqomr: ETH_MTLRXQOMR,
    #[doc = "0xd34 - Rx queue missed packet and overflow counter register"]
    pub eth_mtlrxqmpocr: ETH_MTLRXQMPOCR,
    #[doc = "0xd38 - Rx queue debug register"]
    pub eth_mtlrxqdr: ETH_MTLRXQDR,
    _reserved102: [u8; 0x02c4],
    #[doc = "0x1000 - DMA mode register"]
    pub eth_dmamr: ETH_DMAMR,
    #[doc = "0x1004 - System bus mode register"]
    pub eth_dmasbmr: ETH_DMASBMR,
    #[doc = "0x1008 - Interrupt status register"]
    pub eth_dmaisr: ETH_DMAISR,
    #[doc = "0x100c - Debug status register"]
    pub eth_dmadsr: ETH_DMADSR,
    _reserved106: [u8; 0xf0],
    #[doc = "0x1100 - Channel control register"]
    pub eth_dmaccr: ETH_DMACCR,
    #[doc = "0x1104 - Channel transmit control register"]
    pub eth_dmactxcr: ETH_DMACTXCR,
    #[doc = "0x1108 - Channel receive control register"]
    pub eth_dmacrxcr: ETH_DMACRXCR,
    _reserved109: [u8; 0x08],
    #[doc = "0x1114 - Channel Tx descriptor list address register"]
    pub eth_dmactxdlar: ETH_DMACTXDLAR,
    _reserved110: [u8; 0x04],
    #[doc = "0x111c - Channel Rx descriptor list address register"]
    pub eth_dmacrxdlar: ETH_DMACRXDLAR,
    #[doc = "0x1120 - Channel Tx descriptor tail pointer register"]
    pub eth_dmactxdtpr: ETH_DMACTXDTPR,
    _reserved112: [u8; 0x04],
    #[doc = "0x1128 - Channel Rx descriptor tail pointer register"]
    pub eth_dmacrxdtpr: ETH_DMACRXDTPR,
    #[doc = "0x112c - Channel Tx descriptor ring length register"]
    pub eth_dmactxrlr: ETH_DMACTXRLR,
    #[doc = "0x1130 - Channel Rx descriptor ring length register"]
    pub eth_dmacrxrlr: ETH_DMACRXRLR,
    #[doc = "0x1134 - Channel interrupt enable register"]
    pub eth_dmacier: ETH_DMACIER,
    #[doc = "0x1138 - Channel Rx interrupt watchdog timer register"]
    pub eth_dmacrxiwtr: ETH_DMACRXIWTR,
    _reserved117: [u8; 0x08],
    #[doc = "0x1144 - Channel current application transmit descriptor register"]
    pub eth_dmaccatxdr: ETH_DMACCATXDR,
    _reserved118: [u8; 0x04],
    #[doc = "0x114c - Channel current application receive descriptor register"]
    pub eth_dmaccarxdr: ETH_DMACCARXDR,
    _reserved119: [u8; 0x04],
    #[doc = "0x1154 - Channel current application transmit buffer register"]
    pub eth_dmaccatxbr: ETH_DMACCATXBR,
    _reserved120: [u8; 0x04],
    #[doc = "0x115c - Channel current application receive buffer register"]
    pub eth_dmaccarxbr: ETH_DMACCARXBR,
    #[doc = "0x1160 - Channel status register"]
    pub eth_dmacsr: ETH_DMACSR,
    _reserved122: [u8; 0x08],
    #[doc = "0x116c - Channel missed frame count register"]
    pub eth_dmacmfcr: ETH_DMACMFCR,
}
impl RegisterBlock {
    #[doc = "0xb70 - PPS control register"]
    #[inline(always)]
    pub const fn eth_macppscr_alternate(&self) -> &ETH_MACPPSCR_ALTERNATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(2928usize).cast() }
    }
    #[doc = "0xb70 - PPS control register"]
    #[inline(always)]
    pub const fn eth_macppscr(&self) -> &ETH_MACPPSCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2928usize).cast() }
    }
}
#[doc = "ETH_MACCR (rw) register accessor: an alias for `Reg<ETH_MACCR_SPEC>`"]
pub type ETH_MACCR = crate::Reg<eth_maccr::ETH_MACCR_SPEC>;
#[doc = "Operating mode configuration register"]
pub mod eth_maccr;
#[doc = "ETH_MACECR (rw) register accessor: an alias for `Reg<ETH_MACECR_SPEC>`"]
pub type ETH_MACECR = crate::Reg<eth_macecr::ETH_MACECR_SPEC>;
#[doc = "Extended operating mode configuration register"]
pub mod eth_macecr;
#[doc = "ETH_MACPFR (rw) register accessor: an alias for `Reg<ETH_MACPFR_SPEC>`"]
pub type ETH_MACPFR = crate::Reg<eth_macpfr::ETH_MACPFR_SPEC>;
#[doc = "Packet filtering control register"]
pub mod eth_macpfr;
#[doc = "ETH_MACWTR (rw) register accessor: an alias for `Reg<ETH_MACWTR_SPEC>`"]
pub type ETH_MACWTR = crate::Reg<eth_macwtr::ETH_MACWTR_SPEC>;
#[doc = "Watchdog timeout register"]
pub mod eth_macwtr;
#[doc = "ETH_MACHT0R (rw) register accessor: an alias for `Reg<ETH_MACHT0R_SPEC>`"]
pub type ETH_MACHT0R = crate::Reg<eth_macht0r::ETH_MACHT0R_SPEC>;
#[doc = "Hash Table 0 register"]
pub mod eth_macht0r;
#[doc = "ETH_MACHT1R (rw) register accessor: an alias for `Reg<ETH_MACHT1R_SPEC>`"]
pub type ETH_MACHT1R = crate::Reg<eth_macht1r::ETH_MACHT1R_SPEC>;
#[doc = "Hash Table 1 register"]
pub mod eth_macht1r;
#[doc = "ETH_MACVTR (rw) register accessor: an alias for `Reg<ETH_MACVTR_SPEC>`"]
pub type ETH_MACVTR = crate::Reg<eth_macvtr::ETH_MACVTR_SPEC>;
#[doc = "VLAN tag register"]
pub mod eth_macvtr;
#[doc = "ETH_MACVHTR (rw) register accessor: an alias for `Reg<ETH_MACVHTR_SPEC>`"]
pub type ETH_MACVHTR = crate::Reg<eth_macvhtr::ETH_MACVHTR_SPEC>;
#[doc = "VLAN Hash table register"]
pub mod eth_macvhtr;
#[doc = "ETH_MACVIR (rw) register accessor: an alias for `Reg<ETH_MACVIR_SPEC>`"]
pub type ETH_MACVIR = crate::Reg<eth_macvir::ETH_MACVIR_SPEC>;
#[doc = "VLAN inclusion register"]
pub mod eth_macvir;
#[doc = "ETH_MACIVIR (rw) register accessor: an alias for `Reg<ETH_MACIVIR_SPEC>`"]
pub type ETH_MACIVIR = crate::Reg<eth_macivir::ETH_MACIVIR_SPEC>;
#[doc = "Inner VLAN inclusion register"]
pub mod eth_macivir;
#[doc = "ETH_MACQTXFCR (rw) register accessor: an alias for `Reg<ETH_MACQTXFCR_SPEC>`"]
pub type ETH_MACQTXFCR = crate::Reg<eth_macqtxfcr::ETH_MACQTXFCR_SPEC>;
#[doc = "Tx Queue flow control register"]
pub mod eth_macqtxfcr;
#[doc = "ETH_MACRXFCR (rw) register accessor: an alias for `Reg<ETH_MACRXFCR_SPEC>`"]
pub type ETH_MACRXFCR = crate::Reg<eth_macrxfcr::ETH_MACRXFCR_SPEC>;
#[doc = "Rx flow control register"]
pub mod eth_macrxfcr;
#[doc = "ETH_MACISR (rw) register accessor: an alias for `Reg<ETH_MACISR_SPEC>`"]
pub type ETH_MACISR = crate::Reg<eth_macisr::ETH_MACISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod eth_macisr;
#[doc = "ETH_MACIER (rw) register accessor: an alias for `Reg<ETH_MACIER_SPEC>`"]
pub type ETH_MACIER = crate::Reg<eth_macier::ETH_MACIER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod eth_macier;
#[doc = "ETH_MACRXTXSR (rw) register accessor: an alias for `Reg<ETH_MACRXTXSR_SPEC>`"]
pub type ETH_MACRXTXSR = crate::Reg<eth_macrxtxsr::ETH_MACRXTXSR_SPEC>;
#[doc = "Rx Tx status register"]
pub mod eth_macrxtxsr;
#[doc = "ETH_MACPCSR (rw) register accessor: an alias for `Reg<ETH_MACPCSR_SPEC>`"]
pub type ETH_MACPCSR = crate::Reg<eth_macpcsr::ETH_MACPCSR_SPEC>;
#[doc = "PMT control status register"]
pub mod eth_macpcsr;
#[doc = "ETH_MACRWKPFR (rw) register accessor: an alias for `Reg<ETH_MACRWKPFR_SPEC>`"]
pub type ETH_MACRWKPFR = crate::Reg<eth_macrwkpfr::ETH_MACRWKPFR_SPEC>;
#[doc = "Remote wakeup packet filter register"]
pub mod eth_macrwkpfr;
#[doc = "ETH_MACLCSR (rw) register accessor: an alias for `Reg<ETH_MACLCSR_SPEC>`"]
pub type ETH_MACLCSR = crate::Reg<eth_maclcsr::ETH_MACLCSR_SPEC>;
#[doc = "LPI control and status register"]
pub mod eth_maclcsr;
#[doc = "ETH_MACLTCR (rw) register accessor: an alias for `Reg<ETH_MACLTCR_SPEC>`"]
pub type ETH_MACLTCR = crate::Reg<eth_macltcr::ETH_MACLTCR_SPEC>;
#[doc = "LPI timers control register"]
pub mod eth_macltcr;
#[doc = "ETH_MACLETR (rw) register accessor: an alias for `Reg<ETH_MACLETR_SPEC>`"]
pub type ETH_MACLETR = crate::Reg<eth_macletr::ETH_MACLETR_SPEC>;
#[doc = "LPI entry timer register"]
pub mod eth_macletr;
#[doc = "ETH_MAC1USTCR (rw) register accessor: an alias for `Reg<ETH_MAC1USTCR_SPEC>`"]
pub type ETH_MAC1USTCR = crate::Reg<eth_mac1ustcr::ETH_MAC1USTCR_SPEC>;
#[doc = "One-microsecond-tick counter register"]
pub mod eth_mac1ustcr;
#[doc = "ETH_MACVR (r) register accessor: an alias for `Reg<ETH_MACVR_SPEC>`"]
pub type ETH_MACVR = crate::Reg<eth_macvr::ETH_MACVR_SPEC>;
#[doc = "Version register"]
pub mod eth_macvr;
#[doc = "ETH_MACDR (r) register accessor: an alias for `Reg<ETH_MACDR_SPEC>`"]
pub type ETH_MACDR = crate::Reg<eth_macdr::ETH_MACDR_SPEC>;
#[doc = "Debug register"]
pub mod eth_macdr;
#[doc = "ETH_MACHWF0R (r) register accessor: an alias for `Reg<ETH_MACHWF0R_SPEC>`"]
pub type ETH_MACHWF0R = crate::Reg<eth_machwf0r::ETH_MACHWF0R_SPEC>;
#[doc = "HW feature 0 register"]
pub mod eth_machwf0r;
#[doc = "ETH_MACHWF1R (r) register accessor: an alias for `Reg<ETH_MACHWF1R_SPEC>`"]
pub type ETH_MACHWF1R = crate::Reg<eth_machwf1r::ETH_MACHWF1R_SPEC>;
#[doc = "HW feature 1 register"]
pub mod eth_machwf1r;
#[doc = "ETH_MACHWF2R (r) register accessor: an alias for `Reg<ETH_MACHWF2R_SPEC>`"]
pub type ETH_MACHWF2R = crate::Reg<eth_machwf2r::ETH_MACHWF2R_SPEC>;
#[doc = "HW feature 2 register"]
pub mod eth_machwf2r;
#[doc = "ETH_MACHWF3R (r) register accessor: an alias for `Reg<ETH_MACHWF3R_SPEC>`"]
pub type ETH_MACHWF3R = crate::Reg<eth_machwf3r::ETH_MACHWF3R_SPEC>;
#[doc = "HW feature 3 register"]
pub mod eth_machwf3r;
#[doc = "ETH_MACMDIOAR (rw) register accessor: an alias for `Reg<ETH_MACMDIOAR_SPEC>`"]
pub type ETH_MACMDIOAR = crate::Reg<eth_macmdioar::ETH_MACMDIOAR_SPEC>;
#[doc = "MDIO address register"]
pub mod eth_macmdioar;
#[doc = "ETH_MACMDIODR (rw) register accessor: an alias for `Reg<ETH_MACMDIODR_SPEC>`"]
pub type ETH_MACMDIODR = crate::Reg<eth_macmdiodr::ETH_MACMDIODR_SPEC>;
#[doc = "MDIO data register"]
pub mod eth_macmdiodr;
#[doc = "ETH_MACARPAR (rw) register accessor: an alias for `Reg<ETH_MACARPAR_SPEC>`"]
pub type ETH_MACARPAR = crate::Reg<eth_macarpar::ETH_MACARPAR_SPEC>;
#[doc = "ARP address register"]
pub mod eth_macarpar;
#[doc = "ETH_MACCSRSWCR (rw) register accessor: an alias for `Reg<ETH_MACCSRSWCR_SPEC>`"]
pub type ETH_MACCSRSWCR = crate::Reg<eth_maccsrswcr::ETH_MACCSRSWCR_SPEC>;
#[doc = "CSR software control register"]
pub mod eth_maccsrswcr;
#[doc = "ETH_MACA0HR (rw) register accessor: an alias for `Reg<ETH_MACA0HR_SPEC>`"]
pub type ETH_MACA0HR = crate::Reg<eth_maca0hr::ETH_MACA0HR_SPEC>;
#[doc = "MAC Address 0 high register"]
pub mod eth_maca0hr;
#[doc = "ETH_MACA0LR (rw) register accessor: an alias for `Reg<ETH_MACA0LR_SPEC>`"]
pub type ETH_MACA0LR = crate::Reg<eth_maca0lr::ETH_MACA0LR_SPEC>;
#[doc = "MAC Address 0 low register"]
pub mod eth_maca0lr;
#[doc = "ETH_MACA1HR (rw) register accessor: an alias for `Reg<ETH_MACA1HR_SPEC>`"]
pub type ETH_MACA1HR = crate::Reg<eth_maca1hr::ETH_MACA1HR_SPEC>;
#[doc = "MAC Address 1 high register"]
pub mod eth_maca1hr;
#[doc = "ETH_MACA1LR (rw) register accessor: an alias for `Reg<ETH_MACA1LR_SPEC>`"]
pub type ETH_MACA1LR = crate::Reg<eth_maca1lr::ETH_MACA1LR_SPEC>;
#[doc = "MAC Address 1 low register"]
pub mod eth_maca1lr;
#[doc = "ETH_MACA2HR (rw) register accessor: an alias for `Reg<ETH_MACA2HR_SPEC>`"]
pub type ETH_MACA2HR = crate::Reg<eth_maca2hr::ETH_MACA2HR_SPEC>;
#[doc = "MAC Address 2 high register"]
pub mod eth_maca2hr;
#[doc = "ETH_MACA2LR (rw) register accessor: an alias for `Reg<ETH_MACA2LR_SPEC>`"]
pub type ETH_MACA2LR = crate::Reg<eth_maca2lr::ETH_MACA2LR_SPEC>;
#[doc = "MAC Address 2 low register"]
pub mod eth_maca2lr;
#[doc = "ETH_MACA3HR (rw) register accessor: an alias for `Reg<ETH_MACA3HR_SPEC>`"]
pub type ETH_MACA3HR = crate::Reg<eth_maca3hr::ETH_MACA3HR_SPEC>;
#[doc = "MAC Address 3 high register"]
pub mod eth_maca3hr;
#[doc = "ETH_MACA3LR (rw) register accessor: an alias for `Reg<ETH_MACA3LR_SPEC>`"]
pub type ETH_MACA3LR = crate::Reg<eth_maca3lr::ETH_MACA3LR_SPEC>;
#[doc = "MAC Address 3 low register"]
pub mod eth_maca3lr;
#[doc = "ETH_MMC_CONTROL (rw) register accessor: an alias for `Reg<ETH_MMC_CONTROL_SPEC>`"]
pub type ETH_MMC_CONTROL = crate::Reg<eth_mmc_control::ETH_MMC_CONTROL_SPEC>;
#[doc = "MMC control register"]
pub mod eth_mmc_control;
#[doc = "ETH_MMC_RX_INTERRUPT (rw) register accessor: an alias for `Reg<ETH_MMC_RX_INTERRUPT_SPEC>`"]
pub type ETH_MMC_RX_INTERRUPT = crate::Reg<eth_mmc_rx_interrupt::ETH_MMC_RX_INTERRUPT_SPEC>;
#[doc = "MMC Rx interrupt register"]
pub mod eth_mmc_rx_interrupt;
#[doc = "ETH_MMC_TX_INTERRUPT (rw) register accessor: an alias for `Reg<ETH_MMC_TX_INTERRUPT_SPEC>`"]
pub type ETH_MMC_TX_INTERRUPT = crate::Reg<eth_mmc_tx_interrupt::ETH_MMC_TX_INTERRUPT_SPEC>;
#[doc = "MMC Tx interrupt register"]
pub mod eth_mmc_tx_interrupt;
#[doc = "ETH_MMC_RX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<ETH_MMC_RX_INTERRUPT_MASK_SPEC>`"]
pub type ETH_MMC_RX_INTERRUPT_MASK =
    crate::Reg<eth_mmc_rx_interrupt_mask::ETH_MMC_RX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Rx interrupt mask register"]
pub mod eth_mmc_rx_interrupt_mask;
#[doc = "ETH_MMC_TX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<ETH_MMC_TX_INTERRUPT_MASK_SPEC>`"]
pub type ETH_MMC_TX_INTERRUPT_MASK =
    crate::Reg<eth_mmc_tx_interrupt_mask::ETH_MMC_TX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Tx interrupt mask register"]
pub mod eth_mmc_tx_interrupt_mask;
#[doc = "ETH_TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>`"]
pub type ETH_TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<eth_tx_single_collision_good_packets::ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx single collision good packets register"]
pub mod eth_tx_single_collision_good_packets;
#[doc = "ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>`"]
pub type ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<eth_tx_multiple_collision_good_packets::ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx multiple collision good packets register"]
pub mod eth_tx_multiple_collision_good_packets;
#[doc = "ETH_TX_PACKET_COUNT_GOOD (r) register accessor: an alias for `Reg<ETH_TX_PACKET_COUNT_GOOD_SPEC>`"]
pub type ETH_TX_PACKET_COUNT_GOOD =
    crate::Reg<eth_tx_packet_count_good::ETH_TX_PACKET_COUNT_GOOD_SPEC>;
#[doc = "Tx packet count good register"]
pub mod eth_tx_packet_count_good;
#[doc = "ETH_RX_CRC_ERROR_PACKETS (r) register accessor: an alias for `Reg<ETH_RX_CRC_ERROR_PACKETS_SPEC>`"]
pub type ETH_RX_CRC_ERROR_PACKETS =
    crate::Reg<eth_rx_crc_error_packets::ETH_RX_CRC_ERROR_PACKETS_SPEC>;
#[doc = "Rx CRC error packets register"]
pub mod eth_rx_crc_error_packets;
#[doc = "ETH_RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: an alias for `Reg<ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>`"]
pub type ETH_RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<eth_rx_alignment_error_packets::ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
#[doc = "Rx alignment error packets register"]
pub mod eth_rx_alignment_error_packets;
#[doc = "ETH_RX_UNICAST_PACKETS_GOOD (r) register accessor: an alias for `Reg<ETH_RX_UNICAST_PACKETS_GOOD_SPEC>`"]
pub type ETH_RX_UNICAST_PACKETS_GOOD =
    crate::Reg<eth_rx_unicast_packets_good::ETH_RX_UNICAST_PACKETS_GOOD_SPEC>;
#[doc = "Rx unicast packets good register"]
pub mod eth_rx_unicast_packets_good;
#[doc = "ETH_TX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<ETH_TX_LPI_USEC_CNTR_SPEC>`"]
pub type ETH_TX_LPI_USEC_CNTR = crate::Reg<eth_tx_lpi_usec_cntr::ETH_TX_LPI_USEC_CNTR_SPEC>;
#[doc = "Tx LPI microsecond timer register"]
pub mod eth_tx_lpi_usec_cntr;
#[doc = "ETH_TX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<ETH_TX_LPI_TRAN_CNTR_SPEC>`"]
pub type ETH_TX_LPI_TRAN_CNTR = crate::Reg<eth_tx_lpi_tran_cntr::ETH_TX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Tx LPI transition counter register"]
pub mod eth_tx_lpi_tran_cntr;
#[doc = "ETH_RX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<ETH_RX_LPI_USEC_CNTR_SPEC>`"]
pub type ETH_RX_LPI_USEC_CNTR = crate::Reg<eth_rx_lpi_usec_cntr::ETH_RX_LPI_USEC_CNTR_SPEC>;
#[doc = "Rx LPI microsecond counter register"]
pub mod eth_rx_lpi_usec_cntr;
#[doc = "ETH_RX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<ETH_RX_LPI_TRAN_CNTR_SPEC>`"]
pub type ETH_RX_LPI_TRAN_CNTR = crate::Reg<eth_rx_lpi_tran_cntr::ETH_RX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Rx LPI transition counter register"]
pub mod eth_rx_lpi_tran_cntr;
#[doc = "ETH_MACL3L4C0R (rw) register accessor: an alias for `Reg<ETH_MACL3L4C0R_SPEC>`"]
pub type ETH_MACL3L4C0R = crate::Reg<eth_macl3l4c0r::ETH_MACL3L4C0R_SPEC>;
#[doc = "L3 and L4 control 0 register"]
pub mod eth_macl3l4c0r;
#[doc = "ETH_MACL4A0R (rw) register accessor: an alias for `Reg<ETH_MACL4A0R_SPEC>`"]
pub type ETH_MACL4A0R = crate::Reg<eth_macl4a0r::ETH_MACL4A0R_SPEC>;
#[doc = "Layer4 Address filter 0 register"]
pub mod eth_macl4a0r;
#[doc = "ETH_MACL3A00R (rw) register accessor: an alias for `Reg<ETH_MACL3A00R_SPEC>`"]
pub type ETH_MACL3A00R = crate::Reg<eth_macl3a00r::ETH_MACL3A00R_SPEC>;
#[doc = "Layer3 Address 0 filter 0 register"]
pub mod eth_macl3a00r;
#[doc = "ETH_MACL3A10R (rw) register accessor: an alias for `Reg<ETH_MACL3A10R_SPEC>`"]
pub type ETH_MACL3A10R = crate::Reg<eth_macl3a10r::ETH_MACL3A10R_SPEC>;
#[doc = "Layer3 Address 1 filter 0 register"]
pub mod eth_macl3a10r;
#[doc = "ETH_MACL3A20R (rw) register accessor: an alias for `Reg<ETH_MACL3A20R_SPEC>`"]
pub type ETH_MACL3A20R = crate::Reg<eth_macl3a20r::ETH_MACL3A20R_SPEC>;
#[doc = "Layer3 Address 2 filter 0 register"]
pub mod eth_macl3a20r;
#[doc = "ETH_MACL3A30R (rw) register accessor: an alias for `Reg<ETH_MACL3A30R_SPEC>`"]
pub type ETH_MACL3A30R = crate::Reg<eth_macl3a30r::ETH_MACL3A30R_SPEC>;
#[doc = "Layer3 Address 3 filter 0 register"]
pub mod eth_macl3a30r;
#[doc = "ETH_MACL3L4C1R (rw) register accessor: an alias for `Reg<ETH_MACL3L4C1R_SPEC>`"]
pub type ETH_MACL3L4C1R = crate::Reg<eth_macl3l4c1r::ETH_MACL3L4C1R_SPEC>;
#[doc = "L3 and L4 control 1 register"]
pub mod eth_macl3l4c1r;
#[doc = "ETH_MACL4A1R (rw) register accessor: an alias for `Reg<ETH_MACL4A1R_SPEC>`"]
pub type ETH_MACL4A1R = crate::Reg<eth_macl4a1r::ETH_MACL4A1R_SPEC>;
#[doc = "Layer 4 address filter 1 register"]
pub mod eth_macl4a1r;
#[doc = "ETH_MACL3A01R (rw) register accessor: an alias for `Reg<ETH_MACL3A01R_SPEC>`"]
pub type ETH_MACL3A01R = crate::Reg<eth_macl3a01r::ETH_MACL3A01R_SPEC>;
#[doc = "Layer3 address 0 filter 1 Register"]
pub mod eth_macl3a01r;
#[doc = "ETH_MACL3A11R (rw) register accessor: an alias for `Reg<ETH_MACL3A11R_SPEC>`"]
pub type ETH_MACL3A11R = crate::Reg<eth_macl3a11r::ETH_MACL3A11R_SPEC>;
#[doc = "Layer3 address 1 filter 1 register"]
pub mod eth_macl3a11r;
#[doc = "ETH_MACL3A21R (rw) register accessor: an alias for `Reg<ETH_MACL3A21R_SPEC>`"]
pub type ETH_MACL3A21R = crate::Reg<eth_macl3a21r::ETH_MACL3A21R_SPEC>;
#[doc = "Layer3 address 2 filter 1 Register"]
pub mod eth_macl3a21r;
#[doc = "ETH_MACL3A31R (rw) register accessor: an alias for `Reg<ETH_MACL3A31R_SPEC>`"]
pub type ETH_MACL3A31R = crate::Reg<eth_macl3a31r::ETH_MACL3A31R_SPEC>;
#[doc = "Layer3 address 3 filter 1 register"]
pub mod eth_macl3a31r;
#[doc = "ETH_MACTSCR (rw) register accessor: an alias for `Reg<ETH_MACTSCR_SPEC>`"]
pub type ETH_MACTSCR = crate::Reg<eth_mactscr::ETH_MACTSCR_SPEC>;
#[doc = "Timestamp control Register"]
pub mod eth_mactscr;
#[doc = "ETH_MACSSIR (rw) register accessor: an alias for `Reg<ETH_MACSSIR_SPEC>`"]
pub type ETH_MACSSIR = crate::Reg<eth_macssir::ETH_MACSSIR_SPEC>;
#[doc = "Subsecond increment register"]
pub mod eth_macssir;
#[doc = "ETH_MACSTSR (r) register accessor: an alias for `Reg<ETH_MACSTSR_SPEC>`"]
pub type ETH_MACSTSR = crate::Reg<eth_macstsr::ETH_MACSTSR_SPEC>;
#[doc = "System time seconds register"]
pub mod eth_macstsr;
#[doc = "ETH_MACSTNR (r) register accessor: an alias for `Reg<ETH_MACSTNR_SPEC>`"]
pub type ETH_MACSTNR = crate::Reg<eth_macstnr::ETH_MACSTNR_SPEC>;
#[doc = "System time nanoseconds register"]
pub mod eth_macstnr;
#[doc = "ETH_MACSTSUR (rw) register accessor: an alias for `Reg<ETH_MACSTSUR_SPEC>`"]
pub type ETH_MACSTSUR = crate::Reg<eth_macstsur::ETH_MACSTSUR_SPEC>;
#[doc = "System time seconds update register"]
pub mod eth_macstsur;
#[doc = "ETH_MACSTNUR (rw) register accessor: an alias for `Reg<ETH_MACSTNUR_SPEC>`"]
pub type ETH_MACSTNUR = crate::Reg<eth_macstnur::ETH_MACSTNUR_SPEC>;
#[doc = "System time nanoseconds update register"]
pub mod eth_macstnur;
#[doc = "ETH_MACTSAR (rw) register accessor: an alias for `Reg<ETH_MACTSAR_SPEC>`"]
pub type ETH_MACTSAR = crate::Reg<eth_mactsar::ETH_MACTSAR_SPEC>;
#[doc = "Timestamp addend register"]
pub mod eth_mactsar;
#[doc = "ETH_MACTSSR (rw) register accessor: an alias for `Reg<ETH_MACTSSR_SPEC>`"]
pub type ETH_MACTSSR = crate::Reg<eth_mactssr::ETH_MACTSSR_SPEC>;
#[doc = "Timestamp status register"]
pub mod eth_mactssr;
#[doc = "ETH_MACTXTSSNR (rw) register accessor: an alias for `Reg<ETH_MACTXTSSNR_SPEC>`"]
pub type ETH_MACTXTSSNR = crate::Reg<eth_mactxtssnr::ETH_MACTXTSSNR_SPEC>;
#[doc = "Tx timestamp status nanoseconds register"]
pub mod eth_mactxtssnr;
#[doc = "ETH_MACTXTSSSR (r) register accessor: an alias for `Reg<ETH_MACTXTSSSR_SPEC>`"]
pub type ETH_MACTXTSSSR = crate::Reg<eth_mactxtsssr::ETH_MACTXTSSSR_SPEC>;
#[doc = "Tx timestamp status seconds register"]
pub mod eth_mactxtsssr;
#[doc = "ETH_MACACR (rw) register accessor: an alias for `Reg<ETH_MACACR_SPEC>`"]
pub type ETH_MACACR = crate::Reg<eth_macacr::ETH_MACACR_SPEC>;
#[doc = "Auxiliary control register"]
pub mod eth_macacr;
#[doc = "ETH_MACATSNR (r) register accessor: an alias for `Reg<ETH_MACATSNR_SPEC>`"]
pub type ETH_MACATSNR = crate::Reg<eth_macatsnr::ETH_MACATSNR_SPEC>;
#[doc = "Auxiliary timestamp nanoseconds register"]
pub mod eth_macatsnr;
#[doc = "ETH_MACATSSR (r) register accessor: an alias for `Reg<ETH_MACATSSR_SPEC>`"]
pub type ETH_MACATSSR = crate::Reg<eth_macatssr::ETH_MACATSSR_SPEC>;
#[doc = "Auxiliary timestamp seconds register"]
pub mod eth_macatssr;
#[doc = "ETH_MACTSIACR (rw) register accessor: an alias for `Reg<ETH_MACTSIACR_SPEC>`"]
pub type ETH_MACTSIACR = crate::Reg<eth_mactsiacr::ETH_MACTSIACR_SPEC>;
#[doc = "Timestamp Ingress asymmetric correction register"]
pub mod eth_mactsiacr;
#[doc = "ETH_MACTSEACR (rw) register accessor: an alias for `Reg<ETH_MACTSEACR_SPEC>`"]
pub type ETH_MACTSEACR = crate::Reg<eth_mactseacr::ETH_MACTSEACR_SPEC>;
#[doc = "Timestamp Egress asymmetric correction register"]
pub mod eth_mactseacr;
#[doc = "ETH_MACTSICNR (rw) register accessor: an alias for `Reg<ETH_MACTSICNR_SPEC>`"]
pub type ETH_MACTSICNR = crate::Reg<eth_mactsicnr::ETH_MACTSICNR_SPEC>;
#[doc = "Timestamp Ingress correction nanosecond register"]
pub mod eth_mactsicnr;
#[doc = "ETH_MACTSECNR (rw) register accessor: an alias for `Reg<ETH_MACTSECNR_SPEC>`"]
pub type ETH_MACTSECNR = crate::Reg<eth_mactsecnr::ETH_MACTSECNR_SPEC>;
#[doc = "Timestamp Egress correction nanosecond register"]
pub mod eth_mactsecnr;
#[doc = "ETH_MACPPSCR (rw) register accessor: an alias for `Reg<ETH_MACPPSCR_SPEC>`"]
pub type ETH_MACPPSCR = crate::Reg<eth_macppscr::ETH_MACPPSCR_SPEC>;
#[doc = "PPS control register"]
pub mod eth_macppscr;
#[doc = "ETH_MACPPSCR_ALTERNATE (rw) register accessor: an alias for `Reg<ETH_MACPPSCR_ALTERNATE_SPEC>`"]
pub type ETH_MACPPSCR_ALTERNATE = crate::Reg<eth_macppscr_alternate::ETH_MACPPSCR_ALTERNATE_SPEC>;
#[doc = "PPS control register"]
pub mod eth_macppscr_alternate;
#[doc = "ETH_MACPPSTTSR (rw) register accessor: an alias for `Reg<ETH_MACPPSTTSR_SPEC>`"]
pub type ETH_MACPPSTTSR = crate::Reg<eth_macppsttsr::ETH_MACPPSTTSR_SPEC>;
#[doc = "PPS target time seconds register"]
pub mod eth_macppsttsr;
#[doc = "ETH_MACPPSTTNR (rw) register accessor: an alias for `Reg<ETH_MACPPSTTNR_SPEC>`"]
pub type ETH_MACPPSTTNR = crate::Reg<eth_macppsttnr::ETH_MACPPSTTNR_SPEC>;
#[doc = "PPS target time nanoseconds register"]
pub mod eth_macppsttnr;
#[doc = "ETH_MACPPSIR (rw) register accessor: an alias for `Reg<ETH_MACPPSIR_SPEC>`"]
pub type ETH_MACPPSIR = crate::Reg<eth_macppsir::ETH_MACPPSIR_SPEC>;
#[doc = "PPS interval register"]
pub mod eth_macppsir;
#[doc = "ETH_MACPPSWR (rw) register accessor: an alias for `Reg<ETH_MACPPSWR_SPEC>`"]
pub type ETH_MACPPSWR = crate::Reg<eth_macppswr::ETH_MACPPSWR_SPEC>;
#[doc = "PPS width register"]
pub mod eth_macppswr;
#[doc = "ETH_MACPOCR (rw) register accessor: an alias for `Reg<ETH_MACPOCR_SPEC>`"]
pub type ETH_MACPOCR = crate::Reg<eth_macpocr::ETH_MACPOCR_SPEC>;
#[doc = "PTP Offload control register"]
pub mod eth_macpocr;
#[doc = "ETH_MACSPI0R (rw) register accessor: an alias for `Reg<ETH_MACSPI0R_SPEC>`"]
pub type ETH_MACSPI0R = crate::Reg<eth_macspi0r::ETH_MACSPI0R_SPEC>;
#[doc = "PTP Source Port Identity 0 Register"]
pub mod eth_macspi0r;
#[doc = "ETH_MACSPI1R (rw) register accessor: an alias for `Reg<ETH_MACSPI1R_SPEC>`"]
pub type ETH_MACSPI1R = crate::Reg<eth_macspi1r::ETH_MACSPI1R_SPEC>;
#[doc = "PTP Source port identity 1 register"]
pub mod eth_macspi1r;
#[doc = "ETH_MACSPI2R (rw) register accessor: an alias for `Reg<ETH_MACSPI2R_SPEC>`"]
pub type ETH_MACSPI2R = crate::Reg<eth_macspi2r::ETH_MACSPI2R_SPEC>;
#[doc = "PTP Source port identity 2 register"]
pub mod eth_macspi2r;
#[doc = "ETH_MACLMIR (rw) register accessor: an alias for `Reg<ETH_MACLMIR_SPEC>`"]
pub type ETH_MACLMIR = crate::Reg<eth_maclmir::ETH_MACLMIR_SPEC>;
#[doc = "Log message interval register"]
pub mod eth_maclmir;
#[doc = "ETH_MTLOMR (rw) register accessor: an alias for `Reg<ETH_MTLOMR_SPEC>`"]
pub type ETH_MTLOMR = crate::Reg<eth_mtlomr::ETH_MTLOMR_SPEC>;
#[doc = "Operating mode Register"]
pub mod eth_mtlomr;
#[doc = "ETH_MTLISR (r) register accessor: an alias for `Reg<ETH_MTLISR_SPEC>`"]
pub type ETH_MTLISR = crate::Reg<eth_mtlisr::ETH_MTLISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod eth_mtlisr;
#[doc = "ETH_MTLTXQOMR (rw) register accessor: an alias for `Reg<ETH_MTLTXQOMR_SPEC>`"]
pub type ETH_MTLTXQOMR = crate::Reg<eth_mtltxqomr::ETH_MTLTXQOMR_SPEC>;
#[doc = "Tx queue operating mode Register"]
pub mod eth_mtltxqomr;
#[doc = "ETH_MTLTXQUR (rw) register accessor: an alias for `Reg<ETH_MTLTXQUR_SPEC>`"]
pub type ETH_MTLTXQUR = crate::Reg<eth_mtltxqur::ETH_MTLTXQUR_SPEC>;
#[doc = "Tx queue underflow register"]
pub mod eth_mtltxqur;
#[doc = "ETH_MTLTXQDR (r) register accessor: an alias for `Reg<ETH_MTLTXQDR_SPEC>`"]
pub type ETH_MTLTXQDR = crate::Reg<eth_mtltxqdr::ETH_MTLTXQDR_SPEC>;
#[doc = "Tx queue debug register"]
pub mod eth_mtltxqdr;
#[doc = "ETH_MTLQICSR (rw) register accessor: an alias for `Reg<ETH_MTLQICSR_SPEC>`"]
pub type ETH_MTLQICSR = crate::Reg<eth_mtlqicsr::ETH_MTLQICSR_SPEC>;
#[doc = "Queue interrupt control status Register"]
pub mod eth_mtlqicsr;
#[doc = "ETH_MTLRXQOMR (rw) register accessor: an alias for `Reg<ETH_MTLRXQOMR_SPEC>`"]
pub type ETH_MTLRXQOMR = crate::Reg<eth_mtlrxqomr::ETH_MTLRXQOMR_SPEC>;
#[doc = "Rx queue operating mode register"]
pub mod eth_mtlrxqomr;
#[doc = "ETH_MTLRXQMPOCR (rw) register accessor: an alias for `Reg<ETH_MTLRXQMPOCR_SPEC>`"]
pub type ETH_MTLRXQMPOCR = crate::Reg<eth_mtlrxqmpocr::ETH_MTLRXQMPOCR_SPEC>;
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod eth_mtlrxqmpocr;
#[doc = "ETH_MTLRXQDR (r) register accessor: an alias for `Reg<ETH_MTLRXQDR_SPEC>`"]
pub type ETH_MTLRXQDR = crate::Reg<eth_mtlrxqdr::ETH_MTLRXQDR_SPEC>;
#[doc = "Rx queue debug register"]
pub mod eth_mtlrxqdr;
#[doc = "ETH_DMAMR (rw) register accessor: an alias for `Reg<ETH_DMAMR_SPEC>`"]
pub type ETH_DMAMR = crate::Reg<eth_dmamr::ETH_DMAMR_SPEC>;
#[doc = "DMA mode register"]
pub mod eth_dmamr;
#[doc = "ETH_DMASBMR (rw) register accessor: an alias for `Reg<ETH_DMASBMR_SPEC>`"]
pub type ETH_DMASBMR = crate::Reg<eth_dmasbmr::ETH_DMASBMR_SPEC>;
#[doc = "System bus mode register"]
pub mod eth_dmasbmr;
#[doc = "ETH_DMAISR (r) register accessor: an alias for `Reg<ETH_DMAISR_SPEC>`"]
pub type ETH_DMAISR = crate::Reg<eth_dmaisr::ETH_DMAISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod eth_dmaisr;
#[doc = "ETH_DMADSR (r) register accessor: an alias for `Reg<ETH_DMADSR_SPEC>`"]
pub type ETH_DMADSR = crate::Reg<eth_dmadsr::ETH_DMADSR_SPEC>;
#[doc = "Debug status register"]
pub mod eth_dmadsr;
#[doc = "ETH_DMACCR (rw) register accessor: an alias for `Reg<ETH_DMACCR_SPEC>`"]
pub type ETH_DMACCR = crate::Reg<eth_dmaccr::ETH_DMACCR_SPEC>;
#[doc = "Channel control register"]
pub mod eth_dmaccr;
#[doc = "ETH_DMACTXCR (rw) register accessor: an alias for `Reg<ETH_DMACTXCR_SPEC>`"]
pub type ETH_DMACTXCR = crate::Reg<eth_dmactxcr::ETH_DMACTXCR_SPEC>;
#[doc = "Channel transmit control register"]
pub mod eth_dmactxcr;
#[doc = "ETH_DMACRXCR (rw) register accessor: an alias for `Reg<ETH_DMACRXCR_SPEC>`"]
pub type ETH_DMACRXCR = crate::Reg<eth_dmacrxcr::ETH_DMACRXCR_SPEC>;
#[doc = "Channel receive control register"]
pub mod eth_dmacrxcr;
#[doc = "ETH_DMACTXDLAR (rw) register accessor: an alias for `Reg<ETH_DMACTXDLAR_SPEC>`"]
pub type ETH_DMACTXDLAR = crate::Reg<eth_dmactxdlar::ETH_DMACTXDLAR_SPEC>;
#[doc = "Channel Tx descriptor list address register"]
pub mod eth_dmactxdlar;
#[doc = "ETH_DMACRXDLAR (rw) register accessor: an alias for `Reg<ETH_DMACRXDLAR_SPEC>`"]
pub type ETH_DMACRXDLAR = crate::Reg<eth_dmacrxdlar::ETH_DMACRXDLAR_SPEC>;
#[doc = "Channel Rx descriptor list address register"]
pub mod eth_dmacrxdlar;
#[doc = "ETH_DMACTXDTPR (rw) register accessor: an alias for `Reg<ETH_DMACTXDTPR_SPEC>`"]
pub type ETH_DMACTXDTPR = crate::Reg<eth_dmactxdtpr::ETH_DMACTXDTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod eth_dmactxdtpr;
#[doc = "ETH_DMACRXDTPR (rw) register accessor: an alias for `Reg<ETH_DMACRXDTPR_SPEC>`"]
pub type ETH_DMACRXDTPR = crate::Reg<eth_dmacrxdtpr::ETH_DMACRXDTPR_SPEC>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod eth_dmacrxdtpr;
#[doc = "ETH_DMACTXRLR (rw) register accessor: an alias for `Reg<ETH_DMACTXRLR_SPEC>`"]
pub type ETH_DMACTXRLR = crate::Reg<eth_dmactxrlr::ETH_DMACTXRLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod eth_dmactxrlr;
#[doc = "ETH_DMACRXRLR (rw) register accessor: an alias for `Reg<ETH_DMACRXRLR_SPEC>`"]
pub type ETH_DMACRXRLR = crate::Reg<eth_dmacrxrlr::ETH_DMACRXRLR_SPEC>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod eth_dmacrxrlr;
#[doc = "ETH_DMACIER (rw) register accessor: an alias for `Reg<ETH_DMACIER_SPEC>`"]
pub type ETH_DMACIER = crate::Reg<eth_dmacier::ETH_DMACIER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod eth_dmacier;
#[doc = "ETH_DMACRXIWTR (rw) register accessor: an alias for `Reg<ETH_DMACRXIWTR_SPEC>`"]
pub type ETH_DMACRXIWTR = crate::Reg<eth_dmacrxiwtr::ETH_DMACRXIWTR_SPEC>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod eth_dmacrxiwtr;
#[doc = "ETH_DMACCATXDR (r) register accessor: an alias for `Reg<ETH_DMACCATXDR_SPEC>`"]
pub type ETH_DMACCATXDR = crate::Reg<eth_dmaccatxdr::ETH_DMACCATXDR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod eth_dmaccatxdr;
#[doc = "ETH_DMACCARXDR (r) register accessor: an alias for `Reg<ETH_DMACCARXDR_SPEC>`"]
pub type ETH_DMACCARXDR = crate::Reg<eth_dmaccarxdr::ETH_DMACCARXDR_SPEC>;
#[doc = "Channel current application receive descriptor register"]
pub mod eth_dmaccarxdr;
#[doc = "ETH_DMACCATXBR (r) register accessor: an alias for `Reg<ETH_DMACCATXBR_SPEC>`"]
pub type ETH_DMACCATXBR = crate::Reg<eth_dmaccatxbr::ETH_DMACCATXBR_SPEC>;
#[doc = "Channel current application transmit buffer register"]
pub mod eth_dmaccatxbr;
#[doc = "ETH_DMACCARXBR (r) register accessor: an alias for `Reg<ETH_DMACCARXBR_SPEC>`"]
pub type ETH_DMACCARXBR = crate::Reg<eth_dmaccarxbr::ETH_DMACCARXBR_SPEC>;
#[doc = "Channel current application receive buffer register"]
pub mod eth_dmaccarxbr;
#[doc = "ETH_DMACSR (rw) register accessor: an alias for `Reg<ETH_DMACSR_SPEC>`"]
pub type ETH_DMACSR = crate::Reg<eth_dmacsr::ETH_DMACSR_SPEC>;
#[doc = "Channel status register"]
pub mod eth_dmacsr;
#[doc = "ETH_DMACMFCR (rw) register accessor: an alias for `Reg<ETH_DMACMFCR_SPEC>`"]
pub type ETH_DMACMFCR = crate::Reg<eth_dmacmfcr::ETH_DMACMFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod eth_dmacmfcr;
