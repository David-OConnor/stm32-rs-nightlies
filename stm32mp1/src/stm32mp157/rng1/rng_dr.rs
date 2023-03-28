#[doc = "Register `RNG_DR` reader"]
pub struct R(crate::R<RNG_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RNDATA` reader - RNDATA"]
pub type RNDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RNDATA"]
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
#[doc = "The RNG_DR register is a read-only register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_dr](index.html) module"]
pub struct RNG_DR_SPEC;
impl crate::RegisterSpec for RNG_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_dr::R](R) reader structure"]
impl crate::Readable for RNG_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RNG_DR to value 0"]
impl crate::Resettable for RNG_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
