#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c - receive FIFO 0 register"]
    pub rf0r: RF0R,
    #[doc = "0x10 - receive FIFO 1 register"]
    pub rf1r: RF1R,
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - interrupt enable register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - TX mailbox identifier register"]
    pub ti0r: TI0R,
    #[doc = "0x184 - mailbox data length control and time stamp register"]
    pub tdt0r: TDT0R,
    #[doc = "0x188 - mailbox data low register"]
    pub tdl0r: TDL0R,
    #[doc = "0x18c - mailbox data high register"]
    pub tdh0r: TDH0R,
    #[doc = "0x190 - mailbox identifier register"]
    pub ti1r: TI1R,
    #[doc = "0x194 - mailbox data length control and time stamp register"]
    pub tdt1r: TDT1R,
    #[doc = "0x198 - mailbox data low register"]
    pub tdl1r: TDL1R,
    #[doc = "0x19c - mailbox data high register"]
    pub tdh1r: TDH1R,
    #[doc = "0x1a0 - mailbox identifier register"]
    pub ti2r: TI2R,
    #[doc = "0x1a4 - mailbox data length control and time stamp register"]
    pub tdt2r: TDT2R,
    #[doc = "0x1a8 - mailbox data low register"]
    pub tdl2r: TDL2R,
    #[doc = "0x1ac - mailbox data high register"]
    pub tdh2r: TDH2R,
    #[doc = "0x1b0 - receive FIFO mailbox identifier register"]
    pub ri0r: RI0R,
    #[doc = "0x1b4 - mailbox data high register"]
    pub rdt0r: RDT0R,
    #[doc = "0x1b8 - mailbox data high register"]
    pub rdl0r: RDL0R,
    #[doc = "0x1bc - receive FIFO mailbox data high register"]
    pub rdh0r: RDH0R,
    #[doc = "0x1c0 - mailbox data high register"]
    pub ri1r: RI1R,
    #[doc = "0x1c4 - mailbox data high register"]
    pub rdt1r: RDT1R,
    #[doc = "0x1c8 - mailbox data high register"]
    pub rdl1r: RDL1R,
    #[doc = "0x1cc - mailbox data high register"]
    pub rdh1r: RDH1R,
    _reserved28: [u8; 0x3c],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: FS1R,
    _reserved29: [u8; 0x04],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: FFA1R,
    _reserved30: [u8; 0x04],
    #[doc = "0x21c - filter activation register"]
    pub fa1r: FA1R,
    _reserved31: [u8; 0x20],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: F0R1,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: F0R2,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: F1R1,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: F1R2,
    #[doc = "0x250 - Filter bank 2 register 1"]
    pub f2r1: F2R1,
    #[doc = "0x254 - Filter bank 2 register 2"]
    pub f2r2: F2R2,
    #[doc = "0x258 - Filter bank 3 register 1"]
    pub f3r1: F3R1,
    #[doc = "0x25c - Filter bank 3 register 2"]
    pub f3r2: F3R2,
    #[doc = "0x260 - Filter bank 4 register 1"]
    pub f4r1: F4R1,
    #[doc = "0x264 - Filter bank 4 register 2"]
    pub f4r2: F4R2,
    #[doc = "0x268 - Filter bank 5 register 1"]
    pub f5r1: F5R1,
    #[doc = "0x26c - Filter bank 5 register 2"]
    pub f5r2: F5R2,
    #[doc = "0x270 - Filter bank 6 register 1"]
    pub f6r1: F6R1,
    #[doc = "0x274 - Filter bank 6 register 2"]
    pub f6r2: F6R2,
    #[doc = "0x278 - Filter bank 7 register 1"]
    pub f7r1: F7R1,
    #[doc = "0x27c - Filter bank 7 register 2"]
    pub f7r2: F7R2,
    #[doc = "0x280 - Filter bank 8 register 1"]
    pub f8r1: F8R1,
    #[doc = "0x284 - Filter bank 8 register 2"]
    pub f8r2: F8R2,
    #[doc = "0x288 - Filter bank 9 register 1"]
    pub f9r1: F9R1,
    #[doc = "0x28c - Filter bank 9 register 2"]
    pub f9r2: F9R2,
    #[doc = "0x290 - Filter bank 10 register 1"]
    pub f10r1: F10R1,
    #[doc = "0x294 - Filter bank 10 register 2"]
    pub f10r2: F10R2,
    #[doc = "0x298 - Filter bank 11 register 1"]
    pub f11r1: F11R1,
    #[doc = "0x29c - Filter bank 11 register 2"]
    pub f11r2: F11R2,
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    pub f12r1: F12R1,
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    pub f12r2: F12R2,
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    pub f13r1: F13R1,
    #[doc = "0x2ac - Filter bank 13 register 2"]
    pub f13r2: F13R2,
    #[doc = "0x2b0 - Filter bank 14 register 1"]
    pub f14r1: F14R1,
    #[doc = "0x2b4 - Filter bank 14 register 2"]
    pub f14r2: F14R2,
    #[doc = "0x2b8 - Filter bank 15 register 1"]
    pub f15r1: F15R1,
    #[doc = "0x2bc - Filter bank 15 register 2"]
    pub f15r2: F15R2,
    #[doc = "0x2c0 - Filter bank 16 register 1"]
    pub f16r1: F16R1,
    #[doc = "0x2c4 - Filter bank 16 register 2"]
    pub f16r2: F16R2,
    #[doc = "0x2c8 - Filter bank 17 register 1"]
    pub f17r1: F17R1,
    #[doc = "0x2cc - Filter bank 17 register 2"]
    pub f17r2: F17R2,
    #[doc = "0x2d0 - Filter bank 18 register 1"]
    pub f18r1: F18R1,
    #[doc = "0x2d4 - Filter bank 18 register 2"]
    pub f18r2: F18R2,
    #[doc = "0x2d8 - Filter bank 19 register 1"]
    pub f19r1: F19R1,
    #[doc = "0x2dc - Filter bank 19 register 2"]
    pub f19r2: F19R2,
    #[doc = "0x2e0 - Filter bank 20 register 1"]
    pub f20r1: F20R1,
    #[doc = "0x2e4 - Filter bank 20 register 2"]
    pub f20r2: F20R2,
    #[doc = "0x2e8 - Filter bank 21 register 1"]
    pub f21r1: F21R1,
    #[doc = "0x2ec - Filter bank 21 register 2"]
    pub f21r2: F21R2,
    #[doc = "0x2f0 - Filter bank 22 register 1"]
    pub f22r1: F22R1,
    #[doc = "0x2f4 - Filter bank 22 register 2"]
    pub f22r2: F22R2,
    #[doc = "0x2f8 - Filter bank 23 register 1"]
    pub f23r1: F23R1,
    #[doc = "0x2fc - Filter bank 23 register 2"]
    pub f23r2: F23R2,
    #[doc = "0x300 - Filter bank 24 register 1"]
    pub f24r1: F24R1,
    #[doc = "0x304 - Filter bank 24 register 2"]
    pub f24r2: F24R2,
    #[doc = "0x308 - Filter bank 25 register 1"]
    pub f25r1: F25R1,
    #[doc = "0x30c - Filter bank 25 register 2"]
    pub f25r2: F25R2,
    #[doc = "0x310 - Filter bank 26 register 1"]
    pub f26r1: F26R1,
    #[doc = "0x314 - Filter bank 26 register 2"]
    pub f26r2: F26R2,
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: F27R1,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: F27R2,
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "master control register"]
pub mod mcr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "master status register"]
pub mod msr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "RF0R (rw) register accessor: an alias for `Reg<RF0R_SPEC>`"]
pub type RF0R = crate::Reg<rf0r::RF0R_SPEC>;
#[doc = "receive FIFO 0 register"]
pub mod rf0r;
#[doc = "RF1R (rw) register accessor: an alias for `Reg<RF1R_SPEC>`"]
pub type RF1R = crate::Reg<rf1r::RF1R_SPEC>;
#[doc = "receive FIFO 1 register"]
pub mod rf1r;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ESR (rw) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "bit timing register"]
pub mod btr;
#[doc = "TI0R (rw) register accessor: an alias for `Reg<TI0R_SPEC>`"]
pub type TI0R = crate::Reg<ti0r::TI0R_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod ti0r;
#[doc = "TDT0R (rw) register accessor: an alias for `Reg<TDT0R_SPEC>`"]
pub type TDT0R = crate::Reg<tdt0r::TDT0R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt0r;
#[doc = "TDL0R (rw) register accessor: an alias for `Reg<TDL0R_SPEC>`"]
pub type TDL0R = crate::Reg<tdl0r::TDL0R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl0r;
#[doc = "TDH0R (rw) register accessor: an alias for `Reg<TDH0R_SPEC>`"]
pub type TDH0R = crate::Reg<tdh0r::TDH0R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh0r;
#[doc = "TI1R (rw) register accessor: an alias for `Reg<TI1R_SPEC>`"]
pub type TI1R = crate::Reg<ti1r::TI1R_SPEC>;
#[doc = "mailbox identifier register"]
pub mod ti1r;
#[doc = "TDT1R (rw) register accessor: an alias for `Reg<TDT1R_SPEC>`"]
pub type TDT1R = crate::Reg<tdt1r::TDT1R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt1r;
#[doc = "TDL1R (rw) register accessor: an alias for `Reg<TDL1R_SPEC>`"]
pub type TDL1R = crate::Reg<tdl1r::TDL1R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl1r;
#[doc = "TDH1R (rw) register accessor: an alias for `Reg<TDH1R_SPEC>`"]
pub type TDH1R = crate::Reg<tdh1r::TDH1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh1r;
#[doc = "TI2R (rw) register accessor: an alias for `Reg<TI2R_SPEC>`"]
pub type TI2R = crate::Reg<ti2r::TI2R_SPEC>;
#[doc = "mailbox identifier register"]
pub mod ti2r;
#[doc = "TDT2R (rw) register accessor: an alias for `Reg<TDT2R_SPEC>`"]
pub type TDT2R = crate::Reg<tdt2r::TDT2R_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt2r;
#[doc = "TDL2R (rw) register accessor: an alias for `Reg<TDL2R_SPEC>`"]
pub type TDL2R = crate::Reg<tdl2r::TDL2R_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdl2r;
#[doc = "TDH2R (rw) register accessor: an alias for `Reg<TDH2R_SPEC>`"]
pub type TDH2R = crate::Reg<tdh2r::TDH2R_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdh2r;
#[doc = "RI0R (r) register accessor: an alias for `Reg<RI0R_SPEC>`"]
pub type RI0R = crate::Reg<ri0r::RI0R_SPEC>;
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri0r;
#[doc = "RDT0R (r) register accessor: an alias for `Reg<RDT0R_SPEC>`"]
pub type RDT0R = crate::Reg<rdt0r::RDT0R_SPEC>;
#[doc = "mailbox data high register"]
pub mod rdt0r;
#[doc = "RDL0R (r) register accessor: an alias for `Reg<RDL0R_SPEC>`"]
pub type RDL0R = crate::Reg<rdl0r::RDL0R_SPEC>;
#[doc = "mailbox data high register"]
pub mod rdl0r;
#[doc = "RDH0R (r) register accessor: an alias for `Reg<RDH0R_SPEC>`"]
pub type RDH0R = crate::Reg<rdh0r::RDH0R_SPEC>;
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh0r;
#[doc = "RI1R (r) register accessor: an alias for `Reg<RI1R_SPEC>`"]
pub type RI1R = crate::Reg<ri1r::RI1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod ri1r;
#[doc = "RDT1R (r) register accessor: an alias for `Reg<RDT1R_SPEC>`"]
pub type RDT1R = crate::Reg<rdt1r::RDT1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod rdt1r;
#[doc = "RDL1R (r) register accessor: an alias for `Reg<RDL1R_SPEC>`"]
pub type RDL1R = crate::Reg<rdl1r::RDL1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod rdl1r;
#[doc = "RDH1R (r) register accessor: an alias for `Reg<RDH1R_SPEC>`"]
pub type RDH1R = crate::Reg<rdh1r::RDH1R_SPEC>;
#[doc = "mailbox data high register"]
pub mod rdh1r;
#[doc = "FS1R (rw) register accessor: an alias for `Reg<FS1R_SPEC>`"]
pub type FS1R = crate::Reg<fs1r::FS1R_SPEC>;
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "FFA1R (rw) register accessor: an alias for `Reg<FFA1R_SPEC>`"]
pub type FFA1R = crate::Reg<ffa1r::FFA1R_SPEC>;
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "FA1R (rw) register accessor: an alias for `Reg<FA1R_SPEC>`"]
pub type FA1R = crate::Reg<fa1r::FA1R_SPEC>;
#[doc = "filter activation register"]
pub mod fa1r;
#[doc = "F0R1 (rw) register accessor: an alias for `Reg<F0R1_SPEC>`"]
pub type F0R1 = crate::Reg<f0r1::F0R1_SPEC>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "F0R2 (rw) register accessor: an alias for `Reg<F0R2_SPEC>`"]
pub type F0R2 = crate::Reg<f0r2::F0R2_SPEC>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "F1R1 (rw) register accessor: an alias for `Reg<F1R1_SPEC>`"]
pub type F1R1 = crate::Reg<f1r1::F1R1_SPEC>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "F1R2 (rw) register accessor: an alias for `Reg<F1R2_SPEC>`"]
pub type F1R2 = crate::Reg<f1r2::F1R2_SPEC>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "F2R1 (rw) register accessor: an alias for `Reg<F2R1_SPEC>`"]
pub type F2R1 = crate::Reg<f2r1::F2R1_SPEC>;
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "F2R2 (rw) register accessor: an alias for `Reg<F2R2_SPEC>`"]
pub type F2R2 = crate::Reg<f2r2::F2R2_SPEC>;
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "F3R1 (rw) register accessor: an alias for `Reg<F3R1_SPEC>`"]
pub type F3R1 = crate::Reg<f3r1::F3R1_SPEC>;
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "F3R2 (rw) register accessor: an alias for `Reg<F3R2_SPEC>`"]
pub type F3R2 = crate::Reg<f3r2::F3R2_SPEC>;
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "F4R1 (rw) register accessor: an alias for `Reg<F4R1_SPEC>`"]
pub type F4R1 = crate::Reg<f4r1::F4R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "F4R2 (rw) register accessor: an alias for `Reg<F4R2_SPEC>`"]
pub type F4R2 = crate::Reg<f4r2::F4R2_SPEC>;
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "F5R1 (rw) register accessor: an alias for `Reg<F5R1_SPEC>`"]
pub type F5R1 = crate::Reg<f5r1::F5R1_SPEC>;
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "F5R2 (rw) register accessor: an alias for `Reg<F5R2_SPEC>`"]
pub type F5R2 = crate::Reg<f5r2::F5R2_SPEC>;
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "F6R1 (rw) register accessor: an alias for `Reg<F6R1_SPEC>`"]
pub type F6R1 = crate::Reg<f6r1::F6R1_SPEC>;
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "F6R2 (rw) register accessor: an alias for `Reg<F6R2_SPEC>`"]
pub type F6R2 = crate::Reg<f6r2::F6R2_SPEC>;
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "F7R1 (rw) register accessor: an alias for `Reg<F7R1_SPEC>`"]
pub type F7R1 = crate::Reg<f7r1::F7R1_SPEC>;
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "F7R2 (rw) register accessor: an alias for `Reg<F7R2_SPEC>`"]
pub type F7R2 = crate::Reg<f7r2::F7R2_SPEC>;
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "F8R1 (rw) register accessor: an alias for `Reg<F8R1_SPEC>`"]
pub type F8R1 = crate::Reg<f8r1::F8R1_SPEC>;
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "F8R2 (rw) register accessor: an alias for `Reg<F8R2_SPEC>`"]
pub type F8R2 = crate::Reg<f8r2::F8R2_SPEC>;
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "F9R1 (rw) register accessor: an alias for `Reg<F9R1_SPEC>`"]
pub type F9R1 = crate::Reg<f9r1::F9R1_SPEC>;
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "F9R2 (rw) register accessor: an alias for `Reg<F9R2_SPEC>`"]
pub type F9R2 = crate::Reg<f9r2::F9R2_SPEC>;
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "F10R1 (rw) register accessor: an alias for `Reg<F10R1_SPEC>`"]
pub type F10R1 = crate::Reg<f10r1::F10R1_SPEC>;
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "F10R2 (rw) register accessor: an alias for `Reg<F10R2_SPEC>`"]
pub type F10R2 = crate::Reg<f10r2::F10R2_SPEC>;
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "F11R1 (rw) register accessor: an alias for `Reg<F11R1_SPEC>`"]
pub type F11R1 = crate::Reg<f11r1::F11R1_SPEC>;
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "F11R2 (rw) register accessor: an alias for `Reg<F11R2_SPEC>`"]
pub type F11R2 = crate::Reg<f11r2::F11R2_SPEC>;
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "F12R1 (rw) register accessor: an alias for `Reg<F12R1_SPEC>`"]
pub type F12R1 = crate::Reg<f12r1::F12R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "F12R2 (rw) register accessor: an alias for `Reg<F12R2_SPEC>`"]
pub type F12R2 = crate::Reg<f12r2::F12R2_SPEC>;
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "F13R1 (rw) register accessor: an alias for `Reg<F13R1_SPEC>`"]
pub type F13R1 = crate::Reg<f13r1::F13R1_SPEC>;
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "F13R2 (rw) register accessor: an alias for `Reg<F13R2_SPEC>`"]
pub type F13R2 = crate::Reg<f13r2::F13R2_SPEC>;
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
#[doc = "F14R1 (rw) register accessor: an alias for `Reg<F14R1_SPEC>`"]
pub type F14R1 = crate::Reg<f14r1::F14R1_SPEC>;
#[doc = "Filter bank 14 register 1"]
pub mod f14r1;
#[doc = "F14R2 (rw) register accessor: an alias for `Reg<F14R2_SPEC>`"]
pub type F14R2 = crate::Reg<f14r2::F14R2_SPEC>;
#[doc = "Filter bank 14 register 2"]
pub mod f14r2;
#[doc = "F15R1 (rw) register accessor: an alias for `Reg<F15R1_SPEC>`"]
pub type F15R1 = crate::Reg<f15r1::F15R1_SPEC>;
#[doc = "Filter bank 15 register 1"]
pub mod f15r1;
#[doc = "F15R2 (rw) register accessor: an alias for `Reg<F15R2_SPEC>`"]
pub type F15R2 = crate::Reg<f15r2::F15R2_SPEC>;
#[doc = "Filter bank 15 register 2"]
pub mod f15r2;
#[doc = "F16R1 (rw) register accessor: an alias for `Reg<F16R1_SPEC>`"]
pub type F16R1 = crate::Reg<f16r1::F16R1_SPEC>;
#[doc = "Filter bank 16 register 1"]
pub mod f16r1;
#[doc = "F16R2 (rw) register accessor: an alias for `Reg<F16R2_SPEC>`"]
pub type F16R2 = crate::Reg<f16r2::F16R2_SPEC>;
#[doc = "Filter bank 16 register 2"]
pub mod f16r2;
#[doc = "F17R1 (rw) register accessor: an alias for `Reg<F17R1_SPEC>`"]
pub type F17R1 = crate::Reg<f17r1::F17R1_SPEC>;
#[doc = "Filter bank 17 register 1"]
pub mod f17r1;
#[doc = "F17R2 (rw) register accessor: an alias for `Reg<F17R2_SPEC>`"]
pub type F17R2 = crate::Reg<f17r2::F17R2_SPEC>;
#[doc = "Filter bank 17 register 2"]
pub mod f17r2;
#[doc = "F18R1 (rw) register accessor: an alias for `Reg<F18R1_SPEC>`"]
pub type F18R1 = crate::Reg<f18r1::F18R1_SPEC>;
#[doc = "Filter bank 18 register 1"]
pub mod f18r1;
#[doc = "F18R2 (rw) register accessor: an alias for `Reg<F18R2_SPEC>`"]
pub type F18R2 = crate::Reg<f18r2::F18R2_SPEC>;
#[doc = "Filter bank 18 register 2"]
pub mod f18r2;
#[doc = "F19R1 (rw) register accessor: an alias for `Reg<F19R1_SPEC>`"]
pub type F19R1 = crate::Reg<f19r1::F19R1_SPEC>;
#[doc = "Filter bank 19 register 1"]
pub mod f19r1;
#[doc = "F19R2 (rw) register accessor: an alias for `Reg<F19R2_SPEC>`"]
pub type F19R2 = crate::Reg<f19r2::F19R2_SPEC>;
#[doc = "Filter bank 19 register 2"]
pub mod f19r2;
#[doc = "F20R1 (rw) register accessor: an alias for `Reg<F20R1_SPEC>`"]
pub type F20R1 = crate::Reg<f20r1::F20R1_SPEC>;
#[doc = "Filter bank 20 register 1"]
pub mod f20r1;
#[doc = "F20R2 (rw) register accessor: an alias for `Reg<F20R2_SPEC>`"]
pub type F20R2 = crate::Reg<f20r2::F20R2_SPEC>;
#[doc = "Filter bank 20 register 2"]
pub mod f20r2;
#[doc = "F21R1 (rw) register accessor: an alias for `Reg<F21R1_SPEC>`"]
pub type F21R1 = crate::Reg<f21r1::F21R1_SPEC>;
#[doc = "Filter bank 21 register 1"]
pub mod f21r1;
#[doc = "F21R2 (rw) register accessor: an alias for `Reg<F21R2_SPEC>`"]
pub type F21R2 = crate::Reg<f21r2::F21R2_SPEC>;
#[doc = "Filter bank 21 register 2"]
pub mod f21r2;
#[doc = "F22R1 (rw) register accessor: an alias for `Reg<F22R1_SPEC>`"]
pub type F22R1 = crate::Reg<f22r1::F22R1_SPEC>;
#[doc = "Filter bank 22 register 1"]
pub mod f22r1;
#[doc = "F22R2 (rw) register accessor: an alias for `Reg<F22R2_SPEC>`"]
pub type F22R2 = crate::Reg<f22r2::F22R2_SPEC>;
#[doc = "Filter bank 22 register 2"]
pub mod f22r2;
#[doc = "F23R1 (rw) register accessor: an alias for `Reg<F23R1_SPEC>`"]
pub type F23R1 = crate::Reg<f23r1::F23R1_SPEC>;
#[doc = "Filter bank 23 register 1"]
pub mod f23r1;
#[doc = "F23R2 (rw) register accessor: an alias for `Reg<F23R2_SPEC>`"]
pub type F23R2 = crate::Reg<f23r2::F23R2_SPEC>;
#[doc = "Filter bank 23 register 2"]
pub mod f23r2;
#[doc = "F24R1 (rw) register accessor: an alias for `Reg<F24R1_SPEC>`"]
pub type F24R1 = crate::Reg<f24r1::F24R1_SPEC>;
#[doc = "Filter bank 24 register 1"]
pub mod f24r1;
#[doc = "F24R2 (rw) register accessor: an alias for `Reg<F24R2_SPEC>`"]
pub type F24R2 = crate::Reg<f24r2::F24R2_SPEC>;
#[doc = "Filter bank 24 register 2"]
pub mod f24r2;
#[doc = "F25R1 (rw) register accessor: an alias for `Reg<F25R1_SPEC>`"]
pub type F25R1 = crate::Reg<f25r1::F25R1_SPEC>;
#[doc = "Filter bank 25 register 1"]
pub mod f25r1;
#[doc = "F25R2 (rw) register accessor: an alias for `Reg<F25R2_SPEC>`"]
pub type F25R2 = crate::Reg<f25r2::F25R2_SPEC>;
#[doc = "Filter bank 25 register 2"]
pub mod f25r2;
#[doc = "F26R1 (rw) register accessor: an alias for `Reg<F26R1_SPEC>`"]
pub type F26R1 = crate::Reg<f26r1::F26R1_SPEC>;
#[doc = "Filter bank 26 register 1"]
pub mod f26r1;
#[doc = "F26R2 (rw) register accessor: an alias for `Reg<F26R2_SPEC>`"]
pub type F26R2 = crate::Reg<f26r2::F26R2_SPEC>;
#[doc = "Filter bank 26 register 2"]
pub mod f26r2;
#[doc = "F27R1 (rw) register accessor: an alias for `Reg<F27R1_SPEC>`"]
pub type F27R1 = crate::Reg<f27r1::F27R1_SPEC>;
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "F27R2 (rw) register accessor: an alias for `Reg<F27R2_SPEC>`"]
pub type F27R2 = crate::Reg<f27r2::F27R2_SPEC>;
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
