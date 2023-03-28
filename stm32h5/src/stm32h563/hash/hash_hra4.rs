#[doc = "Register `HASH_HRA4` reader"]
pub struct R(crate::R<HASH_HRA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HRA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HRA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HRA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hra4](index.html) module"]
pub struct HASH_HRA4_SPEC;
impl crate::RegisterSpec for HASH_HRA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hra4::R](R) reader structure"]
impl crate::Readable for HASH_HRA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HRA4 to value 0"]
impl crate::Resettable for HASH_HRA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
