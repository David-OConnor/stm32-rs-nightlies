#[doc = "Register `DDRPHYC_RIDR` reader"]
pub struct R(crate::R<DDRPHYC_RIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_RIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_RIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_RIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PUBMNR` reader - PUBMNR"]
pub type PUBMNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUBMDR` reader - PUBMDR"]
pub type PUBMDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUBMJR` reader - PUBMJR"]
pub type PUBMJR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYMNR` reader - PHYMNR"]
pub type PHYMNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYMDR` reader - PHYMDR"]
pub type PHYMDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYMJR` reader - PHYMJR"]
pub type PHYMJR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDRID` reader - UDRID"]
pub type UDRID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PUBMNR"]
    #[inline(always)]
    pub fn pubmnr(&self) -> PUBMNR_R {
        PUBMNR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PUBMDR"]
    #[inline(always)]
    pub fn pubmdr(&self) -> PUBMDR_R {
        PUBMDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PUBMJR"]
    #[inline(always)]
    pub fn pubmjr(&self) -> PUBMJR_R {
        PUBMJR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PHYMNR"]
    #[inline(always)]
    pub fn phymnr(&self) -> PHYMNR_R {
        PHYMNR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PHYMDR"]
    #[inline(always)]
    pub fn phymdr(&self) -> PHYMDR_R {
        PHYMDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PHYMJR"]
    #[inline(always)]
    pub fn phymjr(&self) -> PHYMJR_R {
        PHYMJR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - UDRID"]
    #[inline(always)]
    pub fn udrid(&self) -> UDRID_R {
        UDRID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DDRPHYC revision ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ridr](index.html) module"]
pub struct DDRPHYC_RIDR_SPEC;
impl crate::RegisterSpec for DDRPHYC_RIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_ridr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_RIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_RIDR to value 0x0041_0010"]
impl crate::Resettable for DDRPHYC_RIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0041_0010;
}
