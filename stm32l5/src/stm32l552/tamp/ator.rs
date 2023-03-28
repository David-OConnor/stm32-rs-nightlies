#[doc = "Register `ATOR` reader"]
pub struct R(crate::R<ATOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRNG` reader - Pseudo-random generator value"]
pub type PRNG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEEDF` reader - Seed running flag"]
pub type SEEDF_R = crate::BitReader<bool>;
#[doc = "Field `INITS` reader - Active tamper initialization status"]
pub type INITS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Pseudo-random generator value"]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - Seed running flag"]
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Active tamper initialization status"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "TAMP active tamper output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ator](index.html) module"]
pub struct ATOR_SPEC;
impl crate::RegisterSpec for ATOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ator::R](R) reader structure"]
impl crate::Readable for ATOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ATOR to value 0"]
impl crate::Resettable for ATOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
