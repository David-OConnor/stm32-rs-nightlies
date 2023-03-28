#[doc = "Register `UR13` reader"]
pub struct R(crate::R<UR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDRS` reader - Secured DTCM RAM Size"]
pub type SDRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D1SBRST` reader - D1 Standby reset"]
pub type D1SBRST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Secured DTCM RAM Size"]
    #[inline(always)]
    pub fn sdrs(&self) -> SDRS_R {
        SDRS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - D1 Standby reset"]
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1SBRST_R {
        D1SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur13](index.html) module"]
pub struct UR13_SPEC;
impl crate::RegisterSpec for UR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur13::R](R) reader structure"]
impl crate::Readable for UR13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR13 to value 0"]
impl crate::Resettable for UR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
