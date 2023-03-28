#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPDMA secure configuration register"]
    pub seccfgr: SECCFGR,
    #[doc = "0x04 - GPDMA privileged configuration register"]
    pub privcfgr: PRIVCFGR,
    #[doc = "0x08 - GPDMA configuration lock register"]
    pub rcfglockr: RCFGLOCKR,
    #[doc = "0x0c - GPDMA non-secure masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x10 - GPDMA secure masked interrupt status register"]
    pub smisr: SMISR,
    _reserved5: [u8; 0x3c],
    #[doc = "0x50 - GPDMA channel 0 linked-list base address register"]
    pub c0lbar: C0LBAR,
    _reserved6: [u8; 0x08],
    #[doc = "0x5c - GPDMA channel 0 flag clear register"]
    pub c0fcr: C0FCR,
    #[doc = "0x60 - GPDMA channel 0 status register"]
    pub c0sr: C0SR,
    #[doc = "0x64 - GPDMA channel 0 control register"]
    pub c0cr: C0CR,
    _reserved9: [u8; 0x28],
    #[doc = "0x90 - GPDMA channel 0 transfer register 1"]
    pub c0tr1: C0TR1,
    #[doc = "0x94 - GPDMA channel 0 transfer register 2"]
    pub c0tr2: C0TR2,
    #[doc = "0x98 - GPDMA channel 0 block register 1"]
    pub c0br1: C0BR1,
    #[doc = "0x9c - GPDMA channel 0 source address register"]
    pub c0sar: C0SAR,
    #[doc = "0xa0 - GPDMA channel 0 destination address register"]
    pub c0dar: C0DAR,
    _reserved14: [u8; 0x28],
    #[doc = "0xcc - GPDMA channel 0 linked-list address register"]
    pub c0llr: C0LLR,
    #[doc = "0xd0 - GPDMA channel 1 linked-list base address register"]
    pub c1lbar: C1LBAR,
    _reserved16: [u8; 0x08],
    #[doc = "0xdc - GPDMA channel 1 flag clear register"]
    pub c1fcr: C1FCR,
    #[doc = "0xe0 - GPDMA channel 1 status register"]
    pub c1sr: C1SR,
    #[doc = "0xe4 - GPDMA channel 1 control register"]
    pub c1cr: C1CR,
    _reserved19: [u8; 0x28],
    #[doc = "0x110 - GPDMA channel 1 transfer register 1"]
    pub c1tr1: C1TR1,
    #[doc = "0x114 - GPDMA channel 1 transfer register 2"]
    pub c1tr2: C1TR2,
    #[doc = "0x118 - GPDMA channel 1 block register 1"]
    pub c1br1: C1BR1,
    #[doc = "0x11c - GPDMA channel 1 source address register"]
    pub c1sar: C1SAR,
    #[doc = "0x120 - GPDMA channel 1 destination address register"]
    pub c1dar: C1DAR,
    _reserved24: [u8; 0x28],
    #[doc = "0x14c - GPDMA channel 1 linked-list address register"]
    pub c1llr: C1LLR,
    #[doc = "0x150 - GPDMA channel 2 linked-list base address register"]
    pub c2lbar: C2LBAR,
    _reserved26: [u8; 0x08],
    #[doc = "0x15c - GPDMA channel 2 flag clear register"]
    pub c2fcr: C2FCR,
    #[doc = "0x160 - GPDMA channel 2 status register"]
    pub c2sr: C2SR,
    #[doc = "0x164 - GPDMA channel 2 control register"]
    pub c2cr: C2CR,
    _reserved29: [u8; 0x28],
    #[doc = "0x190 - GPDMA channel 2 transfer register 1"]
    pub c2tr1: C2TR1,
    #[doc = "0x194 - GPDMA channel 2 transfer register 2"]
    pub c2tr2: C2TR2,
    #[doc = "0x198 - GPDMA channel 2 block register 1"]
    pub c2br1: C2BR1,
    #[doc = "0x19c - GPDMA channel 2 source address register"]
    pub c2sar: C2SAR,
    #[doc = "0x1a0 - GPDMA channel 2 destination address register"]
    pub c2dar: C2DAR,
    _reserved34: [u8; 0x28],
    #[doc = "0x1cc - GPDMA channel 2 linked-list address register"]
    pub c2llr: C2LLR,
    #[doc = "0x1d0 - GPDMA channel 3 linked-list base address register"]
    pub c3lbar: C3LBAR,
    _reserved36: [u8; 0x08],
    #[doc = "0x1dc - GPDMA channel 3 flag clear register"]
    pub c3fcr: C3FCR,
    #[doc = "0x1e0 - GPDMA channel 3 status register"]
    pub c3sr: C3SR,
    #[doc = "0x1e4 - GPDMA channel 3 control register"]
    pub c3cr: C3CR,
    _reserved39: [u8; 0x28],
    #[doc = "0x210 - GPDMA channel 3 transfer register 1"]
    pub c3tr1: C3TR1,
    #[doc = "0x214 - GPDMA channel 3 transfer register 2"]
    pub c3tr2: C3TR2,
    #[doc = "0x218 - GPDMA channel 3 block register 1"]
    pub c3br1: C3BR1,
    #[doc = "0x21c - GPDMA channel 3 source address register"]
    pub c3sar: C3SAR,
    #[doc = "0x220 - GPDMA channel 3 destination address register"]
    pub c3dar: C3DAR,
    _reserved44: [u8; 0x28],
    #[doc = "0x24c - GPDMA channel 3 linked-list address register"]
    pub c3llr: C3LLR,
    #[doc = "0x250 - GPDMA channel 4 linked-list base address register"]
    pub c4lbar: C4LBAR,
    _reserved46: [u8; 0x08],
    #[doc = "0x25c - GPDMA channel 4 flag clear register"]
    pub c4fcr: C4FCR,
    #[doc = "0x260 - GPDMA channel 4 status register"]
    pub c4sr: C4SR,
    #[doc = "0x264 - GPDMA channel 4 control register"]
    pub c4cr: C4CR,
    _reserved49: [u8; 0x28],
    #[doc = "0x290 - GPDMA channel 4 transfer register 1"]
    pub c4tr1: C4TR1,
    #[doc = "0x294 - GPDMA channel 4 transfer register 2"]
    pub c4tr2: C4TR2,
    #[doc = "0x298 - GPDMA channel 4 block register 1"]
    pub c4br1: C4BR1,
    #[doc = "0x29c - GPDMA channel 4 source address register"]
    pub c4sar: C4SAR,
    #[doc = "0x2a0 - GPDMA channel 4 destination address register"]
    pub c4dar: C4DAR,
    _reserved54: [u8; 0x28],
    #[doc = "0x2cc - GPDMA channel 4 linked-list address register"]
    pub c4llr: C4LLR,
    #[doc = "0x2d0 - GPDMA channel 5 linked-list base address register"]
    pub c5lbar: C5LBAR,
    _reserved56: [u8; 0x08],
    #[doc = "0x2dc - GPDMA channel 5 flag clear register"]
    pub c5fcr: C5FCR,
    #[doc = "0x2e0 - GPDMA channel 5 status register"]
    pub c5sr: C5SR,
    #[doc = "0x2e4 - GPDMA channel 5 control register"]
    pub c5cr: C5CR,
    _reserved59: [u8; 0x28],
    #[doc = "0x310 - GPDMA channel 5 transfer register 1"]
    pub c5tr1: C5TR1,
    #[doc = "0x314 - GPDMA channel 5 transfer register 2"]
    pub c5tr2: C5TR2,
    #[doc = "0x318 - GPDMA channel 5 block register 1"]
    pub c5br1: C5BR1,
    #[doc = "0x31c - GPDMA channel 5 source address register"]
    pub c5sar: C5SAR,
    #[doc = "0x320 - GPDMA channel 5 destination address register"]
    pub c5dar: C5DAR,
    _reserved64: [u8; 0x28],
    #[doc = "0x34c - GPDMA channel 5 linked-list address register"]
    pub c5llr: C5LLR,
    #[doc = "0x350 - GPDMA channel 6 linked-list base address register"]
    pub c6lbar: C6LBAR,
    _reserved66: [u8; 0x08],
    #[doc = "0x35c - GPDMA channel 6 flag clear register"]
    pub c6fcr: C6FCR,
    #[doc = "0x360 - GPDMA channel 6 status register"]
    pub c6sr: C6SR,
    #[doc = "0x364 - GPDMA channel 6 control register"]
    pub c6cr: C6CR,
    _reserved69: [u8; 0x28],
    #[doc = "0x390 - GPDMA channel 6 transfer register 1"]
    pub c6tr1: C6TR1,
    #[doc = "0x394 - GPDMA channel 6 transfer register 2"]
    pub c6tr2: C6TR2,
    #[doc = "0x398 - GPDMA channel 6 alternate block register 1"]
    pub c6br1: C6BR1,
    #[doc = "0x39c - GPDMA channel 6 source address register"]
    pub c6sar: C6SAR,
    #[doc = "0x3a0 - GPDMA channel 6 destination address register"]
    pub c6dar: C6DAR,
    #[doc = "0x3a4 - GPDMA channel 6 transfer register 3"]
    pub c6tr3: C6TR3,
    #[doc = "0x3a8 - GPDMA channel 6 block register 2"]
    pub c6br2: C6BR2,
    _reserved76: [u8; 0x20],
    #[doc = "0x3cc - GPDMA channel 6 alternate linked-list address register"]
    pub c6llr: C6LLR,
    #[doc = "0x3d0 - GPDMA channel 7 linked-list base address register"]
    pub c7lbar: C7LBAR,
    _reserved78: [u8; 0x08],
    #[doc = "0x3dc - GPDMA channel 7 flag clear register"]
    pub c7fcr: C7FCR,
    #[doc = "0x3e0 - GPDMA channel 7 status register"]
    pub c7sr: C7SR,
    #[doc = "0x3e4 - GPDMA channel 7 control register"]
    pub c7cr: C7CR,
    _reserved81: [u8; 0x28],
    #[doc = "0x410 - GPDMA channel 7 transfer register 1"]
    pub c7tr1: C7TR1,
    #[doc = "0x414 - GPDMA channel 7 transfer register 2"]
    pub c7tr2: C7TR2,
    #[doc = "0x418 - GPDMA channel 7 alternate block register 1"]
    pub c7br1: C7BR1,
    #[doc = "0x41c - GPDMA channel 7 source address register"]
    pub c7sar: C7SAR,
    #[doc = "0x420 - GPDMA channel 7 destination address register"]
    pub c7dar: C7DAR,
    #[doc = "0x424 - GPDMA channel 7 transfer register 3"]
    pub c7tr3: C7TR3,
    #[doc = "0x428 - GPDMA channel 7 block register 2"]
    pub c7br2: C7BR2,
    _reserved88: [u8; 0x20],
    #[doc = "0x44c - GPDMA channel 7 alternate linked-list address register"]
    pub c7llr: C7LLR,
}
#[doc = "SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
#[doc = "GPDMA secure configuration register"]
pub mod seccfgr;
#[doc = "PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "GPDMA privileged configuration register"]
pub mod privcfgr;
#[doc = "RCFGLOCKR (rw) register accessor: an alias for `Reg<RCFGLOCKR_SPEC>`"]
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKR_SPEC>;
#[doc = "GPDMA configuration lock register"]
pub mod rcfglockr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "GPDMA non-secure masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`"]
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
#[doc = "GPDMA secure masked interrupt status register"]
pub mod smisr;
#[doc = "C0LBAR (rw) register accessor: an alias for `Reg<C0LBAR_SPEC>`"]
pub type C0LBAR = crate::Reg<c0lbar::C0LBAR_SPEC>;
#[doc = "GPDMA channel 0 linked-list base address register"]
pub mod c0lbar;
#[doc = "C0FCR (w) register accessor: an alias for `Reg<C0FCR_SPEC>`"]
pub type C0FCR = crate::Reg<c0fcr::C0FCR_SPEC>;
#[doc = "GPDMA channel 0 flag clear register"]
pub mod c0fcr;
#[doc = "C0SR (r) register accessor: an alias for `Reg<C0SR_SPEC>`"]
pub type C0SR = crate::Reg<c0sr::C0SR_SPEC>;
#[doc = "GPDMA channel 0 status register"]
pub mod c0sr;
#[doc = "C0CR (rw) register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "GPDMA channel 0 control register"]
pub mod c0cr;
#[doc = "C0TR1 (rw) register accessor: an alias for `Reg<C0TR1_SPEC>`"]
pub type C0TR1 = crate::Reg<c0tr1::C0TR1_SPEC>;
#[doc = "GPDMA channel 0 transfer register 1"]
pub mod c0tr1;
#[doc = "C0TR2 (rw) register accessor: an alias for `Reg<C0TR2_SPEC>`"]
pub type C0TR2 = crate::Reg<c0tr2::C0TR2_SPEC>;
#[doc = "GPDMA channel 0 transfer register 2"]
pub mod c0tr2;
#[doc = "C0BR1 (rw) register accessor: an alias for `Reg<C0BR1_SPEC>`"]
pub type C0BR1 = crate::Reg<c0br1::C0BR1_SPEC>;
#[doc = "GPDMA channel 0 block register 1"]
pub mod c0br1;
#[doc = "C0SAR (rw) register accessor: an alias for `Reg<C0SAR_SPEC>`"]
pub type C0SAR = crate::Reg<c0sar::C0SAR_SPEC>;
#[doc = "GPDMA channel 0 source address register"]
pub mod c0sar;
#[doc = "C0DAR (rw) register accessor: an alias for `Reg<C0DAR_SPEC>`"]
pub type C0DAR = crate::Reg<c0dar::C0DAR_SPEC>;
#[doc = "GPDMA channel 0 destination address register"]
pub mod c0dar;
#[doc = "C0LLR (rw) register accessor: an alias for `Reg<C0LLR_SPEC>`"]
pub type C0LLR = crate::Reg<c0llr::C0LLR_SPEC>;
#[doc = "GPDMA channel 0 linked-list address register"]
pub mod c0llr;
#[doc = "C1LBAR (rw) register accessor: an alias for `Reg<C1LBAR_SPEC>`"]
pub type C1LBAR = crate::Reg<c1lbar::C1LBAR_SPEC>;
#[doc = "GPDMA channel 1 linked-list base address register"]
pub mod c1lbar;
#[doc = "C1FCR (w) register accessor: an alias for `Reg<C1FCR_SPEC>`"]
pub type C1FCR = crate::Reg<c1fcr::C1FCR_SPEC>;
#[doc = "GPDMA channel 1 flag clear register"]
pub mod c1fcr;
#[doc = "C1SR (r) register accessor: an alias for `Reg<C1SR_SPEC>`"]
pub type C1SR = crate::Reg<c1sr::C1SR_SPEC>;
#[doc = "GPDMA channel 1 status register"]
pub mod c1sr;
#[doc = "C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "GPDMA channel 1 control register"]
pub mod c1cr;
#[doc = "C1TR1 (rw) register accessor: an alias for `Reg<C1TR1_SPEC>`"]
pub type C1TR1 = crate::Reg<c1tr1::C1TR1_SPEC>;
#[doc = "GPDMA channel 1 transfer register 1"]
pub mod c1tr1;
#[doc = "C1TR2 (rw) register accessor: an alias for `Reg<C1TR2_SPEC>`"]
pub type C1TR2 = crate::Reg<c1tr2::C1TR2_SPEC>;
#[doc = "GPDMA channel 1 transfer register 2"]
pub mod c1tr2;
#[doc = "C1BR1 (rw) register accessor: an alias for `Reg<C1BR1_SPEC>`"]
pub type C1BR1 = crate::Reg<c1br1::C1BR1_SPEC>;
#[doc = "GPDMA channel 1 block register 1"]
pub mod c1br1;
#[doc = "C1SAR (rw) register accessor: an alias for `Reg<C1SAR_SPEC>`"]
pub type C1SAR = crate::Reg<c1sar::C1SAR_SPEC>;
#[doc = "GPDMA channel 1 source address register"]
pub mod c1sar;
#[doc = "C1DAR (rw) register accessor: an alias for `Reg<C1DAR_SPEC>`"]
pub type C1DAR = crate::Reg<c1dar::C1DAR_SPEC>;
#[doc = "GPDMA channel 1 destination address register"]
pub mod c1dar;
#[doc = "C1LLR (rw) register accessor: an alias for `Reg<C1LLR_SPEC>`"]
pub type C1LLR = crate::Reg<c1llr::C1LLR_SPEC>;
#[doc = "GPDMA channel 1 linked-list address register"]
pub mod c1llr;
#[doc = "C2LBAR (rw) register accessor: an alias for `Reg<C2LBAR_SPEC>`"]
pub type C2LBAR = crate::Reg<c2lbar::C2LBAR_SPEC>;
#[doc = "GPDMA channel 2 linked-list base address register"]
pub mod c2lbar;
#[doc = "C2FCR (w) register accessor: an alias for `Reg<C2FCR_SPEC>`"]
pub type C2FCR = crate::Reg<c2fcr::C2FCR_SPEC>;
#[doc = "GPDMA channel 2 flag clear register"]
pub mod c2fcr;
#[doc = "C2SR (r) register accessor: an alias for `Reg<C2SR_SPEC>`"]
pub type C2SR = crate::Reg<c2sr::C2SR_SPEC>;
#[doc = "GPDMA channel 2 status register"]
pub mod c2sr;
#[doc = "C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "GPDMA channel 2 control register"]
pub mod c2cr;
#[doc = "C2TR1 (rw) register accessor: an alias for `Reg<C2TR1_SPEC>`"]
pub type C2TR1 = crate::Reg<c2tr1::C2TR1_SPEC>;
#[doc = "GPDMA channel 2 transfer register 1"]
pub mod c2tr1;
#[doc = "C2TR2 (rw) register accessor: an alias for `Reg<C2TR2_SPEC>`"]
pub type C2TR2 = crate::Reg<c2tr2::C2TR2_SPEC>;
#[doc = "GPDMA channel 2 transfer register 2"]
pub mod c2tr2;
#[doc = "C2BR1 (rw) register accessor: an alias for `Reg<C2BR1_SPEC>`"]
pub type C2BR1 = crate::Reg<c2br1::C2BR1_SPEC>;
#[doc = "GPDMA channel 2 block register 1"]
pub mod c2br1;
#[doc = "C2SAR (rw) register accessor: an alias for `Reg<C2SAR_SPEC>`"]
pub type C2SAR = crate::Reg<c2sar::C2SAR_SPEC>;
#[doc = "GPDMA channel 2 source address register"]
pub mod c2sar;
#[doc = "C2DAR (rw) register accessor: an alias for `Reg<C2DAR_SPEC>`"]
pub type C2DAR = crate::Reg<c2dar::C2DAR_SPEC>;
#[doc = "GPDMA channel 2 destination address register"]
pub mod c2dar;
#[doc = "C2LLR (rw) register accessor: an alias for `Reg<C2LLR_SPEC>`"]
pub type C2LLR = crate::Reg<c2llr::C2LLR_SPEC>;
#[doc = "GPDMA channel 2 linked-list address register"]
pub mod c2llr;
#[doc = "C3LBAR (rw) register accessor: an alias for `Reg<C3LBAR_SPEC>`"]
pub type C3LBAR = crate::Reg<c3lbar::C3LBAR_SPEC>;
#[doc = "GPDMA channel 3 linked-list base address register"]
pub mod c3lbar;
#[doc = "C3FCR (w) register accessor: an alias for `Reg<C3FCR_SPEC>`"]
pub type C3FCR = crate::Reg<c3fcr::C3FCR_SPEC>;
#[doc = "GPDMA channel 3 flag clear register"]
pub mod c3fcr;
#[doc = "C3SR (r) register accessor: an alias for `Reg<C3SR_SPEC>`"]
pub type C3SR = crate::Reg<c3sr::C3SR_SPEC>;
#[doc = "GPDMA channel 3 status register"]
pub mod c3sr;
#[doc = "C3CR (rw) register accessor: an alias for `Reg<C3CR_SPEC>`"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "GPDMA channel 3 control register"]
pub mod c3cr;
#[doc = "C3TR1 (rw) register accessor: an alias for `Reg<C3TR1_SPEC>`"]
pub type C3TR1 = crate::Reg<c3tr1::C3TR1_SPEC>;
#[doc = "GPDMA channel 3 transfer register 1"]
pub mod c3tr1;
#[doc = "C3TR2 (rw) register accessor: an alias for `Reg<C3TR2_SPEC>`"]
pub type C3TR2 = crate::Reg<c3tr2::C3TR2_SPEC>;
#[doc = "GPDMA channel 3 transfer register 2"]
pub mod c3tr2;
#[doc = "C3BR1 (rw) register accessor: an alias for `Reg<C3BR1_SPEC>`"]
pub type C3BR1 = crate::Reg<c3br1::C3BR1_SPEC>;
#[doc = "GPDMA channel 3 block register 1"]
pub mod c3br1;
#[doc = "C3SAR (rw) register accessor: an alias for `Reg<C3SAR_SPEC>`"]
pub type C3SAR = crate::Reg<c3sar::C3SAR_SPEC>;
#[doc = "GPDMA channel 3 source address register"]
pub mod c3sar;
#[doc = "C3DAR (rw) register accessor: an alias for `Reg<C3DAR_SPEC>`"]
pub type C3DAR = crate::Reg<c3dar::C3DAR_SPEC>;
#[doc = "GPDMA channel 3 destination address register"]
pub mod c3dar;
#[doc = "C3LLR (rw) register accessor: an alias for `Reg<C3LLR_SPEC>`"]
pub type C3LLR = crate::Reg<c3llr::C3LLR_SPEC>;
#[doc = "GPDMA channel 3 linked-list address register"]
pub mod c3llr;
#[doc = "C4LBAR (rw) register accessor: an alias for `Reg<C4LBAR_SPEC>`"]
pub type C4LBAR = crate::Reg<c4lbar::C4LBAR_SPEC>;
#[doc = "GPDMA channel 4 linked-list base address register"]
pub mod c4lbar;
#[doc = "C4FCR (w) register accessor: an alias for `Reg<C4FCR_SPEC>`"]
pub type C4FCR = crate::Reg<c4fcr::C4FCR_SPEC>;
#[doc = "GPDMA channel 4 flag clear register"]
pub mod c4fcr;
#[doc = "C4SR (r) register accessor: an alias for `Reg<C4SR_SPEC>`"]
pub type C4SR = crate::Reg<c4sr::C4SR_SPEC>;
#[doc = "GPDMA channel 4 status register"]
pub mod c4sr;
#[doc = "C4CR (rw) register accessor: an alias for `Reg<C4CR_SPEC>`"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "GPDMA channel 4 control register"]
pub mod c4cr;
#[doc = "C4TR1 (rw) register accessor: an alias for `Reg<C4TR1_SPEC>`"]
pub type C4TR1 = crate::Reg<c4tr1::C4TR1_SPEC>;
#[doc = "GPDMA channel 4 transfer register 1"]
pub mod c4tr1;
#[doc = "C4TR2 (rw) register accessor: an alias for `Reg<C4TR2_SPEC>`"]
pub type C4TR2 = crate::Reg<c4tr2::C4TR2_SPEC>;
#[doc = "GPDMA channel 4 transfer register 2"]
pub mod c4tr2;
#[doc = "C4BR1 (rw) register accessor: an alias for `Reg<C4BR1_SPEC>`"]
pub type C4BR1 = crate::Reg<c4br1::C4BR1_SPEC>;
#[doc = "GPDMA channel 4 block register 1"]
pub mod c4br1;
#[doc = "C4SAR (rw) register accessor: an alias for `Reg<C4SAR_SPEC>`"]
pub type C4SAR = crate::Reg<c4sar::C4SAR_SPEC>;
#[doc = "GPDMA channel 4 source address register"]
pub mod c4sar;
#[doc = "C4DAR (rw) register accessor: an alias for `Reg<C4DAR_SPEC>`"]
pub type C4DAR = crate::Reg<c4dar::C4DAR_SPEC>;
#[doc = "GPDMA channel 4 destination address register"]
pub mod c4dar;
#[doc = "C4LLR (rw) register accessor: an alias for `Reg<C4LLR_SPEC>`"]
pub type C4LLR = crate::Reg<c4llr::C4LLR_SPEC>;
#[doc = "GPDMA channel 4 linked-list address register"]
pub mod c4llr;
#[doc = "C5LBAR (rw) register accessor: an alias for `Reg<C5LBAR_SPEC>`"]
pub type C5LBAR = crate::Reg<c5lbar::C5LBAR_SPEC>;
#[doc = "GPDMA channel 5 linked-list base address register"]
pub mod c5lbar;
#[doc = "C5FCR (w) register accessor: an alias for `Reg<C5FCR_SPEC>`"]
pub type C5FCR = crate::Reg<c5fcr::C5FCR_SPEC>;
#[doc = "GPDMA channel 5 flag clear register"]
pub mod c5fcr;
#[doc = "C5SR (r) register accessor: an alias for `Reg<C5SR_SPEC>`"]
pub type C5SR = crate::Reg<c5sr::C5SR_SPEC>;
#[doc = "GPDMA channel 5 status register"]
pub mod c5sr;
#[doc = "C5CR (rw) register accessor: an alias for `Reg<C5CR_SPEC>`"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "GPDMA channel 5 control register"]
pub mod c5cr;
#[doc = "C5TR1 (rw) register accessor: an alias for `Reg<C5TR1_SPEC>`"]
pub type C5TR1 = crate::Reg<c5tr1::C5TR1_SPEC>;
#[doc = "GPDMA channel 5 transfer register 1"]
pub mod c5tr1;
#[doc = "C5TR2 (rw) register accessor: an alias for `Reg<C5TR2_SPEC>`"]
pub type C5TR2 = crate::Reg<c5tr2::C5TR2_SPEC>;
#[doc = "GPDMA channel 5 transfer register 2"]
pub mod c5tr2;
#[doc = "C5BR1 (rw) register accessor: an alias for `Reg<C5BR1_SPEC>`"]
pub type C5BR1 = crate::Reg<c5br1::C5BR1_SPEC>;
#[doc = "GPDMA channel 5 block register 1"]
pub mod c5br1;
#[doc = "C5SAR (rw) register accessor: an alias for `Reg<C5SAR_SPEC>`"]
pub type C5SAR = crate::Reg<c5sar::C5SAR_SPEC>;
#[doc = "GPDMA channel 5 source address register"]
pub mod c5sar;
#[doc = "C5DAR (rw) register accessor: an alias for `Reg<C5DAR_SPEC>`"]
pub type C5DAR = crate::Reg<c5dar::C5DAR_SPEC>;
#[doc = "GPDMA channel 5 destination address register"]
pub mod c5dar;
#[doc = "C5LLR (rw) register accessor: an alias for `Reg<C5LLR_SPEC>`"]
pub type C5LLR = crate::Reg<c5llr::C5LLR_SPEC>;
#[doc = "GPDMA channel 5 linked-list address register"]
pub mod c5llr;
#[doc = "C6LBAR (rw) register accessor: an alias for `Reg<C6LBAR_SPEC>`"]
pub type C6LBAR = crate::Reg<c6lbar::C6LBAR_SPEC>;
#[doc = "GPDMA channel 6 linked-list base address register"]
pub mod c6lbar;
#[doc = "C6FCR (w) register accessor: an alias for `Reg<C6FCR_SPEC>`"]
pub type C6FCR = crate::Reg<c6fcr::C6FCR_SPEC>;
#[doc = "GPDMA channel 6 flag clear register"]
pub mod c6fcr;
#[doc = "C6SR (r) register accessor: an alias for `Reg<C6SR_SPEC>`"]
pub type C6SR = crate::Reg<c6sr::C6SR_SPEC>;
#[doc = "GPDMA channel 6 status register"]
pub mod c6sr;
#[doc = "C6CR (rw) register accessor: an alias for `Reg<C6CR_SPEC>`"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "GPDMA channel 6 control register"]
pub mod c6cr;
#[doc = "C6TR1 (rw) register accessor: an alias for `Reg<C6TR1_SPEC>`"]
pub type C6TR1 = crate::Reg<c6tr1::C6TR1_SPEC>;
#[doc = "GPDMA channel 6 transfer register 1"]
pub mod c6tr1;
#[doc = "C6TR2 (rw) register accessor: an alias for `Reg<C6TR2_SPEC>`"]
pub type C6TR2 = crate::Reg<c6tr2::C6TR2_SPEC>;
#[doc = "GPDMA channel 6 transfer register 2"]
pub mod c6tr2;
#[doc = "C6BR1 (rw) register accessor: an alias for `Reg<C6BR1_SPEC>`"]
pub type C6BR1 = crate::Reg<c6br1::C6BR1_SPEC>;
#[doc = "GPDMA channel 6 alternate block register 1"]
pub mod c6br1;
#[doc = "C6SAR (rw) register accessor: an alias for `Reg<C6SAR_SPEC>`"]
pub type C6SAR = crate::Reg<c6sar::C6SAR_SPEC>;
#[doc = "GPDMA channel 6 source address register"]
pub mod c6sar;
#[doc = "C6DAR (rw) register accessor: an alias for `Reg<C6DAR_SPEC>`"]
pub type C6DAR = crate::Reg<c6dar::C6DAR_SPEC>;
#[doc = "GPDMA channel 6 destination address register"]
pub mod c6dar;
#[doc = "C6TR3 (rw) register accessor: an alias for `Reg<C6TR3_SPEC>`"]
pub type C6TR3 = crate::Reg<c6tr3::C6TR3_SPEC>;
#[doc = "GPDMA channel 6 transfer register 3"]
pub mod c6tr3;
#[doc = "C6BR2 (rw) register accessor: an alias for `Reg<C6BR2_SPEC>`"]
pub type C6BR2 = crate::Reg<c6br2::C6BR2_SPEC>;
#[doc = "GPDMA channel 6 block register 2"]
pub mod c6br2;
#[doc = "C6LLR (rw) register accessor: an alias for `Reg<C6LLR_SPEC>`"]
pub type C6LLR = crate::Reg<c6llr::C6LLR_SPEC>;
#[doc = "GPDMA channel 6 alternate linked-list address register"]
pub mod c6llr;
#[doc = "C7LBAR (rw) register accessor: an alias for `Reg<C7LBAR_SPEC>`"]
pub type C7LBAR = crate::Reg<c7lbar::C7LBAR_SPEC>;
#[doc = "GPDMA channel 7 linked-list base address register"]
pub mod c7lbar;
#[doc = "C7FCR (w) register accessor: an alias for `Reg<C7FCR_SPEC>`"]
pub type C7FCR = crate::Reg<c7fcr::C7FCR_SPEC>;
#[doc = "GPDMA channel 7 flag clear register"]
pub mod c7fcr;
#[doc = "C7SR (r) register accessor: an alias for `Reg<C7SR_SPEC>`"]
pub type C7SR = crate::Reg<c7sr::C7SR_SPEC>;
#[doc = "GPDMA channel 7 status register"]
pub mod c7sr;
#[doc = "C7CR (rw) register accessor: an alias for `Reg<C7CR_SPEC>`"]
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
#[doc = "GPDMA channel 7 control register"]
pub mod c7cr;
#[doc = "C7TR1 (rw) register accessor: an alias for `Reg<C7TR1_SPEC>`"]
pub type C7TR1 = crate::Reg<c7tr1::C7TR1_SPEC>;
#[doc = "GPDMA channel 7 transfer register 1"]
pub mod c7tr1;
#[doc = "C7TR2 (rw) register accessor: an alias for `Reg<C7TR2_SPEC>`"]
pub type C7TR2 = crate::Reg<c7tr2::C7TR2_SPEC>;
#[doc = "GPDMA channel 7 transfer register 2"]
pub mod c7tr2;
#[doc = "C7BR1 (rw) register accessor: an alias for `Reg<C7BR1_SPEC>`"]
pub type C7BR1 = crate::Reg<c7br1::C7BR1_SPEC>;
#[doc = "GPDMA channel 7 alternate block register 1"]
pub mod c7br1;
#[doc = "C7SAR (rw) register accessor: an alias for `Reg<C7SAR_SPEC>`"]
pub type C7SAR = crate::Reg<c7sar::C7SAR_SPEC>;
#[doc = "GPDMA channel 7 source address register"]
pub mod c7sar;
#[doc = "C7DAR (rw) register accessor: an alias for `Reg<C7DAR_SPEC>`"]
pub type C7DAR = crate::Reg<c7dar::C7DAR_SPEC>;
#[doc = "GPDMA channel 7 destination address register"]
pub mod c7dar;
#[doc = "C7TR3 (rw) register accessor: an alias for `Reg<C7TR3_SPEC>`"]
pub type C7TR3 = crate::Reg<c7tr3::C7TR3_SPEC>;
#[doc = "GPDMA channel 7 transfer register 3"]
pub mod c7tr3;
#[doc = "C7BR2 (rw) register accessor: an alias for `Reg<C7BR2_SPEC>`"]
pub type C7BR2 = crate::Reg<c7br2::C7BR2_SPEC>;
#[doc = "GPDMA channel 7 block register 2"]
pub mod c7br2;
#[doc = "C7LLR (rw) register accessor: an alias for `Reg<C7LLR_SPEC>`"]
pub type C7LLR = crate::Reg<c7llr::C7LLR_SPEC>;
#[doc = "GPDMA channel 7 alternate linked-list address register"]
pub mod c7llr;
