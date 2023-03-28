#[doc = "Register `HR6` reader"]
pub struct R(crate::R<HR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H6` reader - H6"]
pub type H6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H6"]
    #[inline(always)]
    pub fn h6(&self) -> H6_R {
        H6_R::new(self.bits)
    }
}
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr6](index.html) module"]
pub struct HR6_SPEC;
impl crate::RegisterSpec for HR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr6::R](R) reader structure"]
impl crate::Readable for HR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR6 to value 0"]
impl crate::Resettable for HR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
