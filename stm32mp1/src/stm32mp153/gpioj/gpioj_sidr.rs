#[doc = "Register `GPIOJ_SIDR` reader"]
pub struct R(crate::R<GPIOJ_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOJ_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOJ_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOJ_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIDR` reader - SIDR"]
pub type SIDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SIDR"]
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new(self.bits)
    }
}
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_sidr](index.html) module"]
pub struct GPIOJ_SIDR_SPEC;
impl crate::RegisterSpec for GPIOJ_SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioj_sidr::R](R) reader structure"]
impl crate::Readable for GPIOJ_SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOJ_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for GPIOJ_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
