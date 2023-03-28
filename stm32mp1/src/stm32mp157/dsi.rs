#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    pub vr: VR,
    #[doc = "0x04 - DSI Host control register"]
    pub cr: CR,
    #[doc = "0x08 - DSI Host clock control register"]
    pub ccr: CCR,
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    pub lvcidr: LVCIDR,
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    pub lcolcr: LCOLCR,
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    pub lpcr: LPCR,
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    pub lpmcr: LPMCR,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - DSI Host protocol configuration register"]
    pub pcr: PCR,
    #[doc = "0x30 - DSI Host generic VCID register"]
    pub gvcidr: GVCIDR,
    #[doc = "0x34 - DSI Host mode configuration register"]
    pub mcr: MCR,
    #[doc = "0x38 - DSI Host video mode configuration register"]
    pub vmcr: VMCR,
    #[doc = "0x3c - DSI Host video packet configuration register"]
    pub vpcr: VPCR,
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    pub vccr: VCCR,
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    pub vnpcr: VNPCR,
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    pub vhsacr: VHSACR,
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    pub vhbpcr: VHBPCR,
    #[doc = "0x50 - DSI Host video line configuration register"]
    pub vlcr: VLCR,
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    pub vvsacr: VVSACR,
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    pub vvbpcr: VVBPCR,
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    pub vvfpcr: VVFPCR,
    #[doc = "0x60 - DSI Host video VA configuration register"]
    pub vvacr: VVACR,
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    pub lccr: LCCR,
    #[doc = "0x68 - DSI Host command mode configuration register"]
    pub cmcr: CMCR,
    #[doc = "0x6c - DSI Host generic header configuration register"]
    pub ghcr: GHCR,
    #[doc = "0x70 - DSI Host generic payload data register"]
    pub gpdr: GPDR,
    #[doc = "0x74 - DSI Host generic packet status register"]
    pub gpsr: GPSR,
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    pub tccr0: TCCR0,
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    pub tccr1: TCCR1,
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    pub tccr2: TCCR2,
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    pub tccr3: TCCR3,
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    pub tccr4: TCCR4,
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    pub tccr5: TCCR5,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    pub clcr: CLCR,
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    pub cltcr: CLTCR,
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    pub dltcr: DLTCR,
    #[doc = "0xa0 - DSI Host PHY control register"]
    pub pctlr: PCTLR,
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    pub pconfr: PCONFR,
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    pub pucr: PUCR,
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    pub pttcr: PTTCR,
    #[doc = "0xb0 - DSI Host PHY status register"]
    pub psr: PSR,
    _reserved40: [u8; 0x08],
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    pub isr0: ISR0,
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    pub isr1: ISR1,
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    pub ier0: IER0,
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    pub ier1: IER1,
    _reserved44: [u8; 0x0c],
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    pub fir0: FIR0,
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    pub fir1: FIR1,
    _reserved46: [u8; 0x14],
    #[doc = "0xf4 - DSI Host data lane timer read configuration register"]
    pub dltrcr: DLTRCR,
    _reserved47: [u8; 0x08],
    #[doc = "0x100 - DSI Host video shadow control register"]
    pub vscr: VSCR,
    _reserved48: [u8; 0x08],
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    pub lcvcidr: LCVCIDR,
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    pub lcccr: LCCCR,
    _reserved50: [u8; 0x04],
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    pub lpmccr: LPMCCR,
    _reserved51: [u8; 0x1c],
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    pub vmccr: VMCCR,
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    pub vpccr: VPCCR,
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    pub vcccr: VCCCR,
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    pub vnpccr: VNPCCR,
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    pub vhsaccr: VHSACCR,
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    pub vhbpccr: VHBPCCR,
    #[doc = "0x150 - DSI Host video line current configuration register"]
    pub vlccr: VLCCR,
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    pub vvsaccr: VVSACCR,
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    pub vvbpccr: VVBPCCR,
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    pub vvfpccr: VVFPCCR,
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    pub vvaccr: VVACCR,
    _reserved62: [u8; 0x029c],
    #[doc = "0x400 - DSI wrapper configuration register"]
    pub wcfgr: WCFGR,
    #[doc = "0x404 - DSI wrapper control register"]
    pub wcr: WCR,
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    pub wier: WIER,
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    pub wisr: WISR,
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    pub wifcr: WIFCR,
    _reserved67: [u8; 0x04],
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    pub wpcr0: WPCR0,
    #[doc = "0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
    pub wpcr1: WPCR1,
    _reserved69: [u8; 0x10],
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    pub wrpcr: WRPCR,
    _reserved70: [u8; 0x03bc],
    #[doc = "0x7f0 - DSI Host hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x7f4 - DSI Host version register"]
    pub verr: VERR,
    #[doc = "0x7f8 - DSI Host identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x7fc - DSI Host size identification register"]
    pub sidr: SIDR,
}
#[doc = "VR (r) register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "DSI Host version register"]
pub mod vr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DSI Host control register"]
pub mod cr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DSI Host clock control register"]
pub mod ccr;
#[doc = "LVCIDR (rw) register accessor: an alias for `Reg<LVCIDR_SPEC>`"]
pub type LVCIDR = crate::Reg<lvcidr::LVCIDR_SPEC>;
#[doc = "DSI Host LTDC VCID register"]
pub mod lvcidr;
#[doc = "LCOLCR (rw) register accessor: an alias for `Reg<LCOLCR_SPEC>`"]
pub type LCOLCR = crate::Reg<lcolcr::LCOLCR_SPEC>;
#[doc = "DSI Host LTDC color coding register"]
pub mod lcolcr;
#[doc = "LPCR (rw) register accessor: an alias for `Reg<LPCR_SPEC>`"]
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod lpcr;
#[doc = "LPMCR (rw) register accessor: an alias for `Reg<LPMCR_SPEC>`"]
pub type LPMCR = crate::Reg<lpmcr::LPMCR_SPEC>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod lpmcr;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "DSI Host protocol configuration register"]
pub mod pcr;
#[doc = "GVCIDR (r) register accessor: an alias for `Reg<GVCIDR_SPEC>`"]
pub type GVCIDR = crate::Reg<gvcidr::GVCIDR_SPEC>;
#[doc = "DSI Host generic VCID register"]
pub mod gvcidr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "DSI Host mode configuration register"]
pub mod mcr;
#[doc = "VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "DSI Host video mode configuration register"]
pub mod vmcr;
#[doc = "VPCR (rw) register accessor: an alias for `Reg<VPCR_SPEC>`"]
pub type VPCR = crate::Reg<vpcr::VPCR_SPEC>;
#[doc = "DSI Host video packet configuration register"]
pub mod vpcr;
#[doc = "VCCR (rw) register accessor: an alias for `Reg<VCCR_SPEC>`"]
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
#[doc = "DSI Host video chunks configuration register"]
pub mod vccr;
#[doc = "VNPCR (rw) register accessor: an alias for `Reg<VNPCR_SPEC>`"]
pub type VNPCR = crate::Reg<vnpcr::VNPCR_SPEC>;
#[doc = "DSI Host video null packet configuration register"]
pub mod vnpcr;
#[doc = "VHSACR (rw) register accessor: an alias for `Reg<VHSACR_SPEC>`"]
pub type VHSACR = crate::Reg<vhsacr::VHSACR_SPEC>;
#[doc = "DSI Host video HSA configuration register"]
pub mod vhsacr;
#[doc = "VHBPCR (rw) register accessor: an alias for `Reg<VHBPCR_SPEC>`"]
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCR_SPEC>;
#[doc = "DSI Host video HBP configuration register"]
pub mod vhbpcr;
#[doc = "VLCR (rw) register accessor: an alias for `Reg<VLCR_SPEC>`"]
pub type VLCR = crate::Reg<vlcr::VLCR_SPEC>;
#[doc = "DSI Host video line configuration register"]
pub mod vlcr;
#[doc = "VVSACR (rw) register accessor: an alias for `Reg<VVSACR_SPEC>`"]
pub type VVSACR = crate::Reg<vvsacr::VVSACR_SPEC>;
#[doc = "DSI Host video VSA configuration register"]
pub mod vvsacr;
#[doc = "VVBPCR (rw) register accessor: an alias for `Reg<VVBPCR_SPEC>`"]
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCR_SPEC>;
#[doc = "DSI Host video VBP configuration register"]
pub mod vvbpcr;
#[doc = "VVFPCR (rw) register accessor: an alias for `Reg<VVFPCR_SPEC>`"]
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCR_SPEC>;
#[doc = "DSI Host video VFP configuration register"]
pub mod vvfpcr;
#[doc = "VVACR (rw) register accessor: an alias for `Reg<VVACR_SPEC>`"]
pub type VVACR = crate::Reg<vvacr::VVACR_SPEC>;
#[doc = "DSI Host video VA configuration register"]
pub mod vvacr;
#[doc = "LCCR (rw) register accessor: an alias for `Reg<LCCR_SPEC>`"]
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod lccr;
#[doc = "CMCR (rw) register accessor: an alias for `Reg<CMCR_SPEC>`"]
pub type CMCR = crate::Reg<cmcr::CMCR_SPEC>;
#[doc = "DSI Host command mode configuration register"]
pub mod cmcr;
#[doc = "GHCR (rw) register accessor: an alias for `Reg<GHCR_SPEC>`"]
pub type GHCR = crate::Reg<ghcr::GHCR_SPEC>;
#[doc = "DSI Host generic header configuration register"]
pub mod ghcr;
#[doc = "GPDR (rw) register accessor: an alias for `Reg<GPDR_SPEC>`"]
pub type GPDR = crate::Reg<gpdr::GPDR_SPEC>;
#[doc = "DSI Host generic payload data register"]
pub mod gpdr;
#[doc = "GPSR (r) register accessor: an alias for `Reg<GPSR_SPEC>`"]
pub type GPSR = crate::Reg<gpsr::GPSR_SPEC>;
#[doc = "DSI Host generic packet status register"]
pub mod gpsr;
#[doc = "TCCR0 (rw) register accessor: an alias for `Reg<TCCR0_SPEC>`"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod tccr0;
#[doc = "TCCR1 (rw) register accessor: an alias for `Reg<TCCR1_SPEC>`"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod tccr1;
#[doc = "TCCR2 (rw) register accessor: an alias for `Reg<TCCR2_SPEC>`"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod tccr2;
#[doc = "TCCR3 (rw) register accessor: an alias for `Reg<TCCR3_SPEC>`"]
pub type TCCR3 = crate::Reg<tccr3::TCCR3_SPEC>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod tccr3;
#[doc = "TCCR4 (rw) register accessor: an alias for `Reg<TCCR4_SPEC>`"]
pub type TCCR4 = crate::Reg<tccr4::TCCR4_SPEC>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod tccr4;
#[doc = "TCCR5 (rw) register accessor: an alias for `Reg<TCCR5_SPEC>`"]
pub type TCCR5 = crate::Reg<tccr5::TCCR5_SPEC>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod tccr5;
#[doc = "CLCR (rw) register accessor: an alias for `Reg<CLCR_SPEC>`"]
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
#[doc = "DSI Host clock lane configuration register"]
pub mod clcr;
#[doc = "CLTCR (rw) register accessor: an alias for `Reg<CLTCR_SPEC>`"]
pub type CLTCR = crate::Reg<cltcr::CLTCR_SPEC>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod cltcr;
#[doc = "DLTCR (rw) register accessor: an alias for `Reg<DLTCR_SPEC>`"]
pub type DLTCR = crate::Reg<dltcr::DLTCR_SPEC>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dltcr;
#[doc = "PCTLR (rw) register accessor: an alias for `Reg<PCTLR_SPEC>`"]
pub type PCTLR = crate::Reg<pctlr::PCTLR_SPEC>;
#[doc = "DSI Host PHY control register"]
pub mod pctlr;
#[doc = "PCONFR (rw) register accessor: an alias for `Reg<PCONFR_SPEC>`"]
pub type PCONFR = crate::Reg<pconfr::PCONFR_SPEC>;
#[doc = "DSI Host PHY configuration register"]
pub mod pconfr;
#[doc = "PUCR (rw) register accessor: an alias for `Reg<PUCR_SPEC>`"]
pub type PUCR = crate::Reg<pucr::PUCR_SPEC>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod pucr;
#[doc = "PTTCR (rw) register accessor: an alias for `Reg<PTTCR_SPEC>`"]
pub type PTTCR = crate::Reg<pttcr::PTTCR_SPEC>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod pttcr;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "DSI Host PHY status register"]
pub mod psr;
#[doc = "ISR0 (r) register accessor: an alias for `Reg<ISR0_SPEC>`"]
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod isr0;
#[doc = "ISR1 (r) register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod isr1;
#[doc = "IER0 (rw) register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod ier0;
#[doc = "IER1 (rw) register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod ier1;
#[doc = "FIR0 (w) register accessor: an alias for `Reg<FIR0_SPEC>`"]
pub type FIR0 = crate::Reg<fir0::FIR0_SPEC>;
#[doc = "DSI Host force interrupt register 0"]
pub mod fir0;
#[doc = "FIR1 (w) register accessor: an alias for `Reg<FIR1_SPEC>`"]
pub type FIR1 = crate::Reg<fir1::FIR1_SPEC>;
#[doc = "DSI Host force interrupt register 1"]
pub mod fir1;
#[doc = "DLTRCR (rw) register accessor: an alias for `Reg<DLTRCR_SPEC>`"]
pub type DLTRCR = crate::Reg<dltrcr::DLTRCR_SPEC>;
#[doc = "DSI Host data lane timer read configuration register"]
pub mod dltrcr;
#[doc = "VSCR (rw) register accessor: an alias for `Reg<VSCR_SPEC>`"]
pub type VSCR = crate::Reg<vscr::VSCR_SPEC>;
#[doc = "DSI Host video shadow control register"]
pub mod vscr;
#[doc = "LCVCIDR (rw) register accessor: an alias for `Reg<LCVCIDR_SPEC>`"]
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDR_SPEC>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod lcvcidr;
#[doc = "LCCCR (r) register accessor: an alias for `Reg<LCCCR_SPEC>`"]
pub type LCCCR = crate::Reg<lcccr::LCCCR_SPEC>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod lcccr;
#[doc = "LPMCCR (r) register accessor: an alias for `Reg<LPMCCR_SPEC>`"]
pub type LPMCCR = crate::Reg<lpmccr::LPMCCR_SPEC>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod lpmccr;
#[doc = "VMCCR (r) register accessor: an alias for `Reg<VMCCR_SPEC>`"]
pub type VMCCR = crate::Reg<vmccr::VMCCR_SPEC>;
#[doc = "DSI Host video mode current configuration register"]
pub mod vmccr;
#[doc = "VPCCR (r) register accessor: an alias for `Reg<VPCCR_SPEC>`"]
pub type VPCCR = crate::Reg<vpccr::VPCCR_SPEC>;
#[doc = "DSI Host video packet current configuration register"]
pub mod vpccr;
#[doc = "VCCCR (r) register accessor: an alias for `Reg<VCCCR_SPEC>`"]
pub type VCCCR = crate::Reg<vcccr::VCCCR_SPEC>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod vcccr;
#[doc = "VNPCCR (r) register accessor: an alias for `Reg<VNPCCR_SPEC>`"]
pub type VNPCCR = crate::Reg<vnpccr::VNPCCR_SPEC>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod vnpccr;
#[doc = "VHSACCR (r) register accessor: an alias for `Reg<VHSACCR_SPEC>`"]
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCR_SPEC>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod vhsaccr;
#[doc = "VHBPCCR (r) register accessor: an alias for `Reg<VHBPCCR_SPEC>`"]
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCR_SPEC>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod vhbpccr;
#[doc = "VLCCR (r) register accessor: an alias for `Reg<VLCCR_SPEC>`"]
pub type VLCCR = crate::Reg<vlccr::VLCCR_SPEC>;
#[doc = "DSI Host video line current configuration register"]
pub mod vlccr;
#[doc = "VVSACCR (r) register accessor: an alias for `Reg<VVSACCR_SPEC>`"]
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCR_SPEC>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod vvsaccr;
#[doc = "VVBPCCR (r) register accessor: an alias for `Reg<VVBPCCR_SPEC>`"]
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCR_SPEC>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod vvbpccr;
#[doc = "VVFPCCR (r) register accessor: an alias for `Reg<VVFPCCR_SPEC>`"]
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCR_SPEC>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod vvfpccr;
#[doc = "VVACCR (r) register accessor: an alias for `Reg<VVACCR_SPEC>`"]
pub type VVACCR = crate::Reg<vvaccr::VVACCR_SPEC>;
#[doc = "DSI Host video VA current configuration register"]
pub mod vvaccr;
#[doc = "WCFGR (rw) register accessor: an alias for `Reg<WCFGR_SPEC>`"]
pub type WCFGR = crate::Reg<wcfgr::WCFGR_SPEC>;
#[doc = "DSI wrapper configuration register"]
pub mod wcfgr;
#[doc = "WCR (rw) register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "DSI wrapper control register"]
pub mod wcr;
#[doc = "WIER (rw) register accessor: an alias for `Reg<WIER_SPEC>`"]
pub type WIER = crate::Reg<wier::WIER_SPEC>;
#[doc = "DSI wrapper interrupt enable register"]
pub mod wier;
#[doc = "WISR (r) register accessor: an alias for `Reg<WISR_SPEC>`"]
pub type WISR = crate::Reg<wisr::WISR_SPEC>;
#[doc = "DSI wrapper interrupt and status register"]
pub mod wisr;
#[doc = "WIFCR (w) register accessor: an alias for `Reg<WIFCR_SPEC>`"]
pub type WIFCR = crate::Reg<wifcr::WIFCR_SPEC>;
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod wifcr;
#[doc = "WPCR0 (rw) register accessor: an alias for `Reg<WPCR0_SPEC>`"]
pub type WPCR0 = crate::Reg<wpcr0::WPCR0_SPEC>;
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod wpcr0;
#[doc = "WPCR1 (rw) register accessor: an alias for `Reg<WPCR1_SPEC>`"]
pub type WPCR1 = crate::Reg<wpcr1::WPCR1_SPEC>;
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
pub mod wpcr1;
#[doc = "WRPCR (rw) register accessor: an alias for `Reg<WRPCR_SPEC>`"]
pub type WRPCR = crate::Reg<wrpcr::WRPCR_SPEC>;
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod wrpcr;
#[doc = "HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "DSI Host hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "DSI Host version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "DSI Host identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "DSI Host size identification register"]
pub mod sidr;
