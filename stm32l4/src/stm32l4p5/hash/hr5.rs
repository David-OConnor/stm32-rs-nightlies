#[doc = "Register `HR5` reader"]
pub struct R(crate::R<HR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H5` reader - H5"]
pub type H5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H5"]
    #[inline(always)]
    pub fn h5(&self) -> H5_R {
        H5_R::new(self.bits)
    }
}
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr5](index.html) module"]
pub struct HR5_SPEC;
impl crate::RegisterSpec for HR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr5::R](R) reader structure"]
impl crate::Readable for HR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR5 to value 0"]
impl crate::Resettable for HR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
