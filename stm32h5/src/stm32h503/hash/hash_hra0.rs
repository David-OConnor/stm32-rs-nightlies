#[doc = "Register `HASH_HRA0` reader"]
pub struct R(crate::R<HASH_HRA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HRA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HRA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HRA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H0` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hra0](index.html) module"]
pub struct HASH_HRA0_SPEC;
impl crate::RegisterSpec for HASH_HRA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hra0::R](R) reader structure"]
impl crate::Readable for HASH_HRA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HRA0 to value 0"]
impl crate::Resettable for HASH_HRA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
