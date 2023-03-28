#[doc = "Register `HRA0` reader"]
pub struct R(crate::R<HRA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H0` reader - H0"]
pub type H0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hra0](index.html) module"]
pub struct HRA0_SPEC;
impl crate::RegisterSpec for HRA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hra0::R](R) reader structure"]
impl crate::Readable for HRA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRA0 to value 0"]
impl crate::Resettable for HRA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
