#[doc = "Register `TAMP_ATSEEDR` writer"]
pub struct W(crate::W<TAMP_ATSEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ATSEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TAMP_ATSEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ATSEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEED` writer - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
pub type SEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATSEEDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
    #[inline(always)]
    #[must_use]
    pub fn seed(&mut self) -> SEED_W<0> {
        SEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper seed register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atseedr](index.html) module"]
pub struct TAMP_ATSEEDR_SPEC;
impl crate::RegisterSpec for TAMP_ATSEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tamp_atseedr::W](W) writer structure"]
impl crate::Writable for TAMP_ATSEEDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_ATSEEDR to value 0"]
impl crate::Resettable for TAMP_ATSEEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
