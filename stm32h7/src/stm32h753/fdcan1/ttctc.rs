#[doc = "Register `TTCTC` reader"]
pub struct R(crate::R<TTCTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CT` reader - Cycle Time"]
pub type CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CC` reader - Cycle Count"]
pub type CC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Cycle Time"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Cycle Count"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "FDCAN TT Cycle Time and Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttctc](index.html) module"]
pub struct TTCTC_SPEC;
impl crate::RegisterSpec for TTCTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttctc::R](R) reader structure"]
impl crate::Readable for TTCTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTCTC to value 0"]
impl crate::Resettable for TTCTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
