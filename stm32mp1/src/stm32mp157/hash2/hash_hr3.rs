#[doc = "Register `HASH_HR3` reader"]
pub struct R(crate::R<HASH_HR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H3` reader - H3"]
pub type H3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H3"]
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
#[doc = "HASH digest register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr3](index.html) module"]
pub struct HASH_HR3_SPEC;
impl crate::RegisterSpec for HASH_HR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hr3::R](R) reader structure"]
impl crate::Readable for HASH_HR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HR3 to value 0"]
impl crate::Resettable for HASH_HR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
