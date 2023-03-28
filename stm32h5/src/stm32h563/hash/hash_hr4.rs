#[doc = "Register `HASH_HR4` reader"]
pub struct R(crate::R<HASH_HR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR4_SPEC>) -> Self {
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
#[doc = "HASH digest register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr4](index.html) module"]
pub struct HASH_HR4_SPEC;
impl crate::RegisterSpec for HASH_HR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hr4::R](R) reader structure"]
impl crate::Readable for HASH_HR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HR4 to value 0"]
impl crate::Resettable for HASH_HR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
