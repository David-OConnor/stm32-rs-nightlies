#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub grxfsiz: GRXFSIZ,
    _reserved_10_gnptxfsiz: [u8; 0x04],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - core ID register"]
    pub cid: CID,
    _reserved14: [u8; 0x14],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub glpmcfg: GLPMCFG,
    #[doc = "0x58 - OTG power down register"]
    pub gpwrdn: GPWRDN,
    _reserved16: [u8; 0x04],
    #[doc = "0x60 - OTG ADP timer, control and status register"]
    pub gadpctl: GADPCTL,
    _reserved17: [u8; 0x9c],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)"]
    pub dieptxf5: DIEPTXF5,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - OTG status read and pop (host mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - OTG status read and pop (device mode)"]
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz_host(&self) -> &GNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz_device(&self) -> &GNPTXFSIZ_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod gintmsk;
#[doc = "GRXSTSR_Device (r) register accessor: an alias for `Reg<GRXSTSR_DEVICE_SPEC>`"]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod grxstsr_device;
#[doc = "GRXSTSR_Host (r) register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`"]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod grxstsr_host;
#[doc = "GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_Device (rw) register accessor: an alias for `Reg<GNPTXFSIZ_DEVICE_SPEC>`"]
pub type GNPTXFSIZ_DEVICE = crate::Reg<gnptxfsiz_device::GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod gnptxfsiz_device;
#[doc = "GNPTXFSIZ_Host (rw) register accessor: an alias for `Reg<GNPTXFSIZ_HOST_SPEC>`"]
pub type GNPTXFSIZ_HOST = crate::Reg<gnptxfsiz_host::GNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod gnptxfsiz_host;
#[doc = "GNPTXSTS (r) register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod gnptxsts;
#[doc = "GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod dieptxf3;
#[doc = "GRXSTSP_Device (r) register accessor: an alias for `Reg<GRXSTSP_DEVICE_SPEC>`"]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICE_SPEC>;
#[doc = "OTG status read and pop (device mode)"]
pub mod grxstsp_device;
#[doc = "GRXSTSP_Host (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`"]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
#[doc = "OTG status read and pop (host mode)"]
pub mod grxstsp_host;
#[doc = "DIEPTXF4 (rw) register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)"]
pub mod dieptxf5;
#[doc = "GLPMCFG (rw) register accessor: an alias for `Reg<GLPMCFG_SPEC>`"]
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFG_SPEC>;
#[doc = "OTG core LPM configuration register"]
pub mod glpmcfg;
#[doc = "GPWRDN (rw) register accessor: an alias for `Reg<GPWRDN_SPEC>`"]
pub type GPWRDN = crate::Reg<gpwrdn::GPWRDN_SPEC>;
#[doc = "OTG power down register"]
pub mod gpwrdn;
#[doc = "GADPCTL (rw) register accessor: an alias for `Reg<GADPCTL_SPEC>`"]
pub type GADPCTL = crate::Reg<gadpctl::GADPCTL_SPEC>;
#[doc = "OTG ADP timer, control and status register"]
pub mod gadpctl;
