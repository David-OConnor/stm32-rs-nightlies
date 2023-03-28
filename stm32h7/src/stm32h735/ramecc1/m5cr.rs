#[doc = "Register `M5CR` reader"]
pub struct R(crate::R<M5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5cr](index.html) module"]
pub struct M5CR_SPEC;
impl crate::RegisterSpec for M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5cr::R](R) reader structure"]
impl crate::Readable for M5CR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
