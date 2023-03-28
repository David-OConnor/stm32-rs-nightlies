#[doc = "Register `STGENC_PIDR6` reader"]
pub struct R(crate::R<STGENC_PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR6` reader - PIDR6"]
pub type PIDR6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
#[doc = "STGENC peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr6](index.html) module"]
pub struct STGENC_PIDR6_SPEC;
impl crate::RegisterSpec for STGENC_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_pidr6::R](R) reader structure"]
impl crate::Readable for STGENC_PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_PIDR6 to value 0"]
impl crate::Resettable for STGENC_PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
