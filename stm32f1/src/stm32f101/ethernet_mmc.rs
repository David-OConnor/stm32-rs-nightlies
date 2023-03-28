#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register (ETH_MMCCR)"]
    pub mmccr: MMCCR,
    #[doc = "0x04 - Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
    pub mmcrir: MMCRIR,
    #[doc = "0x08 - Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
    pub mmctir: MMCTIR,
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
    pub mmcrimr: MMCRIMR,
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
    pub mmctimr: MMCTIMR,
    _reserved5: [u8; 0x38],
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    pub mmctgfsccr: MMCTGFSCCR,
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    pub mmctgfmsccr: MMCTGFMSCCR,
    _reserved7: [u8; 0x14],
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    pub mmctgfcr: MMCTGFCR,
    _reserved8: [u8; 0x28],
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    pub mmcrfcecr: MMCRFCECR,
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    pub mmcrfaecr: MMCRFAECR,
    _reserved10: [u8; 0x28],
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    pub mmcrgufcr: MMCRGUFCR,
}
#[doc = "MMCCR (rw) register accessor: an alias for `Reg<MMCCR_SPEC>`"]
pub type MMCCR = crate::Reg<mmccr::MMCCR_SPEC>;
#[doc = "Ethernet MMC control register (ETH_MMCCR)"]
pub mod mmccr;
#[doc = "MMCRIR (rw) register accessor: an alias for `Reg<MMCRIR_SPEC>`"]
pub type MMCRIR = crate::Reg<mmcrir::MMCRIR_SPEC>;
#[doc = "Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
pub mod mmcrir;
#[doc = "MMCTIR (rw) register accessor: an alias for `Reg<MMCTIR_SPEC>`"]
pub type MMCTIR = crate::Reg<mmctir::MMCTIR_SPEC>;
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
pub mod mmctir;
#[doc = "MMCRIMR (rw) register accessor: an alias for `Reg<MMCRIMR_SPEC>`"]
pub type MMCRIMR = crate::Reg<mmcrimr::MMCRIMR_SPEC>;
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
pub mod mmcrimr;
#[doc = "MMCTIMR (rw) register accessor: an alias for `Reg<MMCTIMR_SPEC>`"]
pub type MMCTIMR = crate::Reg<mmctimr::MMCTIMR_SPEC>;
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
pub mod mmctimr;
#[doc = "MMCTGFSCCR (r) register accessor: an alias for `Reg<MMCTGFSCCR_SPEC>`"]
pub type MMCTGFSCCR = crate::Reg<mmctgfsccr::MMCTGFSCCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctgfsccr;
#[doc = "MMCTGFMSCCR (r) register accessor: an alias for `Reg<MMCTGFMSCCR_SPEC>`"]
pub type MMCTGFMSCCR = crate::Reg<mmctgfmsccr::MMCTGFMSCCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctgfmsccr;
#[doc = "MMCTGFCR (r) register accessor: an alias for `Reg<MMCTGFCR_SPEC>`"]
pub type MMCTGFCR = crate::Reg<mmctgfcr::MMCTGFCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctgfcr;
#[doc = "MMCRFCECR (r) register accessor: an alias for `Reg<MMCRFCECR_SPEC>`"]
pub type MMCRFCECR = crate::Reg<mmcrfcecr::MMCRFCECR_SPEC>;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecr;
#[doc = "MMCRFAECR (r) register accessor: an alias for `Reg<MMCRFAECR_SPEC>`"]
pub type MMCRFAECR = crate::Reg<mmcrfaecr::MMCRFAECR_SPEC>;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecr;
#[doc = "MMCRGUFCR (r) register accessor: an alias for `Reg<MMCRGUFCR_SPEC>`"]
pub type MMCRGUFCR = crate::Reg<mmcrgufcr::MMCRGUFCR_SPEC>;
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcr;
