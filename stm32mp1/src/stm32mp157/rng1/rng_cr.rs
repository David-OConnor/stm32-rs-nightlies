#[doc = "Register `RNG_CR` reader"]
pub struct R(crate::R<RNG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_CR` writer"]
pub struct W(crate::W<RNG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_CR_SPEC>;
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
impl From<crate::W<RNG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNGEN` reader - RNGEN"]
pub type RNGEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGEN` writer - RNGEN"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_CR_SPEC, bool, O>;
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_CR_SPEC, bool, O>;
#[doc = "Field `CED` reader - CED"]
pub type CED_R = crate::BitReader<bool>;
#[doc = "Field `CED` writer - CED"]
pub type CED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CED"]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RNGEN"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<2> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 3 - IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<3> {
        IE_W::new(self)
    }
    #[doc = "Bit 5 - CED"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<5> {
        CED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_cr](index.html) module"]
pub struct RNG_CR_SPEC;
impl crate::RegisterSpec for RNG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_cr::R](R) reader structure"]
impl crate::Readable for RNG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_cr::W](W) writer structure"]
impl crate::Writable for RNG_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNG_CR to value 0"]
impl crate::Resettable for RNG_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
