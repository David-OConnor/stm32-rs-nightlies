#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_BCR1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub btr1: BTR1,
    #[doc = "0x08 - FMC_BCR2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - FMC_BTR2"]
    pub btr2: BTR2,
    #[doc = "0x10 - >FMC_BCR3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - FMC_BTR3"]
    pub btr3: BTR3,
    #[doc = "0x18 - >FMC_BCR4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - FMC_BTR4"]
    pub btr4: BTR4,
    #[doc = "0x20 - PCSCNTR"]
    pub pcscntr: PCSCNTR,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - NAND Flash control registers"]
    pub pcr: PCR,
    #[doc = "0x84 - This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
    pub sr: SR,
    #[doc = "0x88 - The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
    pub pmem: PMEM,
    #[doc = "0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
    pub patt: PATT,
    _reserved13: [u8; 0x04],
    #[doc = "0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub eccr: ECCR,
    _reserved14: [u8; 0x6c],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr1: BWTR1,
    _reserved15: [u8; 0x04],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr2: BWTR2,
    _reserved16: [u8; 0x04],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr3: BWTR3,
    _reserved17: [u8; 0x04],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr4: BWTR4,
}
#[doc = "BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "FMC_BCR1"]
pub mod bcr1;
#[doc = "BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "FMC_BCR2"]
pub mod bcr2;
#[doc = "BCR3 (rw) register accessor: an alias for `Reg<BCR3_SPEC>`"]
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
#[doc = ">FMC_BCR3"]
pub mod bcr3;
#[doc = "BCR4 (rw) register accessor: an alias for `Reg<BCR4_SPEC>`"]
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
#[doc = ">FMC_BCR4"]
pub mod bcr4;
#[doc = "BTR1 (rw) register accessor: an alias for `Reg<BTR1_SPEC>`"]
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr1;
#[doc = "BTR2 (rw) register accessor: an alias for `Reg<BTR2_SPEC>`"]
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
#[doc = "FMC_BTR2"]
pub mod btr2;
#[doc = "BTR3 (rw) register accessor: an alias for `Reg<BTR3_SPEC>`"]
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
#[doc = "FMC_BTR3"]
pub mod btr3;
#[doc = "BTR4 (rw) register accessor: an alias for `Reg<BTR4_SPEC>`"]
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
#[doc = "FMC_BTR4"]
pub mod btr4;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "NAND Flash control registers"]
pub mod pcr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
pub mod sr;
#[doc = "PMEM (rw) register accessor: an alias for `Reg<PMEM_SPEC>`"]
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
pub mod pmem;
#[doc = "PATT (rw) register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
pub mod patt;
#[doc = "ECCR (r) register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod eccr;
#[doc = "BWTR1 (rw) register accessor: an alias for `Reg<BWTR1_SPEC>`"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr1;
#[doc = "BWTR2 (rw) register accessor: an alias for `Reg<BWTR2_SPEC>`"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr2;
#[doc = "BWTR3 (rw) register accessor: an alias for `Reg<BWTR3_SPEC>`"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr3;
#[doc = "BWTR4 (rw) register accessor: an alias for `Reg<BWTR4_SPEC>`"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr4;
#[doc = "PCSCNTR (rw) register accessor: an alias for `Reg<PCSCNTR_SPEC>`"]
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTR_SPEC>;
#[doc = "PCSCNTR"]
pub mod pcscntr;
