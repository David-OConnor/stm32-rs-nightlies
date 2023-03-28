#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (ETH_MACCR)"]
    pub maccr: MACCR,
    #[doc = "0x04 - Ethernet MAC frame filter register (ETH_MACCFFR)"]
    pub macffr: MACFFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: MACHTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: MACHTLR,
    #[doc = "0x10 - Ethernet MAC MII address register (ETH_MACMIIAR)"]
    pub macmiiar: MACMIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register (ETH_MACMIIDR)"]
    pub macmiidr: MACMIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register (ETH_MACFCR)"]
    pub macfcr: MACFCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
    pub macvlantr: MACVLANTR,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
    pub macrwuffr: MACRWUFFR,
    #[doc = "0x2c - Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
    pub macpmtcsr: MACPMTCSR,
    _reserved10: [u8; 0x08],
    #[doc = "0x38 - Ethernet MAC interrupt status register (ETH_MACSR)"]
    pub macsr: MACSR,
    #[doc = "0x3c - Ethernet MAC interrupt mask register (ETH_MACIMR)"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register (ETH_MACA0HR)"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register (ETH_MACA1HR)"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register (ETH_MACA2HR)"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register (ETH_MACA3HR)"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: MACA3LR,
}
#[doc = "MACCR (rw) register accessor: an alias for `Reg<MACCR_SPEC>`"]
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
#[doc = "Ethernet MAC configuration register (ETH_MACCR)"]
pub mod maccr;
#[doc = "MACFFR (rw) register accessor: an alias for `Reg<MACFFR_SPEC>`"]
pub type MACFFR = crate::Reg<macffr::MACFFR_SPEC>;
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)"]
pub mod macffr;
#[doc = "MACHTHR (rw) register accessor: an alias for `Reg<MACHTHR_SPEC>`"]
pub type MACHTHR = crate::Reg<machthr::MACHTHR_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "MACHTLR (rw) register accessor: an alias for `Reg<MACHTLR_SPEC>`"]
pub type MACHTLR = crate::Reg<machtlr::MACHTLR_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "MACMIIAR (rw) register accessor: an alias for `Reg<MACMIIAR_SPEC>`"]
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIAR_SPEC>;
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)"]
pub mod macmiiar;
#[doc = "MACMIIDR (rw) register accessor: an alias for `Reg<MACMIIDR_SPEC>`"]
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDR_SPEC>;
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)"]
pub mod macmiidr;
#[doc = "MACFCR (rw) register accessor: an alias for `Reg<MACFCR_SPEC>`"]
pub type MACFCR = crate::Reg<macfcr::MACFCR_SPEC>;
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)"]
pub mod macfcr;
#[doc = "MACVLANTR (rw) register accessor: an alias for `Reg<MACVLANTR_SPEC>`"]
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
pub mod macvlantr;
#[doc = "MACRWUFFR (rw) register accessor: an alias for `Reg<MACRWUFFR_SPEC>`"]
pub type MACRWUFFR = crate::Reg<macrwuffr::MACRWUFFR_SPEC>;
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
pub mod macrwuffr;
#[doc = "MACPMTCSR (rw) register accessor: an alias for `Reg<MACPMTCSR_SPEC>`"]
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSR_SPEC>;
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
pub mod macpmtcsr;
#[doc = "MACSR (rw) register accessor: an alias for `Reg<MACSR_SPEC>`"]
pub type MACSR = crate::Reg<macsr::MACSR_SPEC>;
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)"]
pub mod macsr;
#[doc = "MACIMR (rw) register accessor: an alias for `Reg<MACIMR_SPEC>`"]
pub type MACIMR = crate::Reg<macimr::MACIMR_SPEC>;
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)"]
pub mod macimr;
#[doc = "MACA0HR (rw) register accessor: an alias for `Reg<MACA0HR_SPEC>`"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Ethernet MAC address 0 high register (ETH_MACA0HR)"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: an alias for `Reg<MACA0LR_SPEC>`"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: an alias for `Reg<MACA1HR_SPEC>`"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: an alias for `Reg<MACA1LR_SPEC>`"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: an alias for `Reg<MACA2HR_SPEC>`"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: an alias for `Reg<MACA2LR_SPEC>`"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: an alias for `Reg<MACA3HR_SPEC>`"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Ethernet MAC address 3 high register (ETH_MACA3HR)"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: an alias for `Reg<MACA3LR_SPEC>`"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
