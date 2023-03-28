#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: DEACHINTMSK,
    _reserved13: [u8; 0x04],
    #[doc = "0x44 - "]
    pub diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    #[doc = "0x84 - "]
    pub doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 0x78],
    #[doc = "0x100..0x11c - Device IN endpoint 0"]
    pub diep0: DIEP0,
    _reserved16: [u8; 0x04],
    #[doc = "0x120..0x220 - Device IN endpoint X"]
    pub diep: [DIEP; 8],
    _reserved17: [u8; 0xe0],
    #[doc = "0x300..0x318 - Device OUT endpoint 0"]
    pub doep0: DOEP0,
    _reserved18: [u8; 0x08],
    #[doc = "0x320..0x420 - Device IN endpoint X"]
    pub doep: [DOEP; 8],
}
impl RegisterBlock {
    #[doc = "0x120..0x140 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep1(&self) -> &DIEP {
        &self.diep[0]
    }
    #[doc = "0x140..0x160 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep2(&self) -> &DIEP {
        &self.diep[1]
    }
    #[doc = "0x160..0x180 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep3(&self) -> &DIEP {
        &self.diep[2]
    }
    #[doc = "0x180..0x1a0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep4(&self) -> &DIEP {
        &self.diep[3]
    }
    #[doc = "0x1a0..0x1c0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep5(&self) -> &DIEP {
        &self.diep[4]
    }
    #[doc = "0x1c0..0x1e0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep6(&self) -> &DIEP {
        &self.diep[5]
    }
    #[doc = "0x1e0..0x200 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep7(&self) -> &DIEP {
        &self.diep[6]
    }
    #[doc = "0x200..0x220 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep8(&self) -> &DIEP {
        &self.diep[7]
    }
    #[doc = "0x320..0x340 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep1(&self) -> &DOEP {
        &self.doep[0]
    }
    #[doc = "0x340..0x360 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep2(&self) -> &DOEP {
        &self.doep[1]
    }
    #[doc = "0x360..0x380 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep3(&self) -> &DOEP {
        &self.doep[2]
    }
    #[doc = "0x380..0x3a0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep4(&self) -> &DOEP {
        &self.doep[3]
    }
    #[doc = "0x3a0..0x3c0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep5(&self) -> &DOEP {
        &self.doep[4]
    }
    #[doc = "0x3c0..0x3e0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep6(&self) -> &DOEP {
        &self.doep[5]
    }
    #[doc = "0x3e0..0x400 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep7(&self) -> &DOEP {
        &self.doep[6]
    }
    #[doc = "0x400..0x420 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep8(&self) -> &DOEP {
        &self.doep[7]
    }
}
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: an alias for `Reg<DEACHINT_SPEC>`"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "Device IN endpoint 0"]
pub use self::diep0::DIEP0;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint 0"]
pub mod diep0;
#[doc = "Device IN endpoint X"]
pub use self::diep::DIEP;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint X"]
pub mod diep;
#[doc = "Device OUT endpoint 0"]
pub use self::doep0::DOEP0;
#[doc = r"Cluster"]
#[doc = "Device OUT endpoint 0"]
pub mod doep0;
#[doc = "Device IN endpoint X"]
pub use self::doep::DOEP;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint X"]
pub mod doep;
#[doc = "DIEPEACHMSK1 (rw) register accessor: an alias for `Reg<DIEPEACHMSK1_SPEC>`"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = ""]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: an alias for `Reg<DOEPEACHMSK1_SPEC>`"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = ""]
pub mod doepeachmsk1;
