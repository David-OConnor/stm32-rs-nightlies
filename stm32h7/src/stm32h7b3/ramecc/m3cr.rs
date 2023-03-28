#[doc = "Register `M3CR` reader"]
pub struct R(crate::R<M3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3cr](index.html) module"]
pub struct M3CR_SPEC;
impl crate::RegisterSpec for M3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m3cr::R](R) reader structure"]
impl crate::Readable for M3CR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M3CR to value 0"]
impl crate::Resettable for M3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
