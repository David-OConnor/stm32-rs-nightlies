#[doc = "Register `OTG_DSTS` reader"]
pub struct R(crate::R<OTG_DSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPSTS` reader - SUSPSTS"]
pub type SUSPSTS_R = crate::BitReader<bool>;
#[doc = "Field `ENUMSPD` reader - ENUMSPD"]
pub type ENUMSPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EERR` reader - EERR"]
pub type EERR_R = crate::BitReader<bool>;
#[doc = "Field `FNSOF` reader - FNSOF"]
pub type FNSOF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVLNSTS` reader - DEVLNSTS"]
pub type DEVLNSTS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - SUSPSTS"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ENUMSPD"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - EERR"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - FNSOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - DEVLNSTS"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dsts](index.html) module"]
pub struct OTG_DSTS_SPEC;
impl crate::RegisterSpec for OTG_DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dsts::R](R) reader structure"]
impl crate::Readable for OTG_DSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_DSTS to value 0x10"]
impl crate::Resettable for OTG_DSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
