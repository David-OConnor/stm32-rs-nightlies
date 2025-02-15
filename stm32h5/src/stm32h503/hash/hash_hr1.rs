#[doc = "Register `HASH_HR1` reader"]
pub struct R(crate::R<HASH_HR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H1` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
#[doc = "HASH digest register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr1](index.html) module"]
pub struct HASH_HR1_SPEC;
impl crate::RegisterSpec for HASH_HR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hr1::R](R) reader structure"]
impl crate::Readable for HASH_HR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HR1 to value 0"]
impl crate::Resettable for HASH_HR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
