#[doc = "Register `LTDC_IER` reader"]
pub struct R(crate::R<LTDC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_IER` writer"]
pub struct W(crate::W<LTDC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_IER_SPEC>;
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
impl From<crate::W<LTDC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIE` reader - LIE"]
pub type LIE_R = crate::BitReader<bool>;
#[doc = "Field `LIE` writer - LIE"]
pub type LIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_IER_SPEC, bool, O>;
#[doc = "Field `FUIE` reader - FUIE"]
pub type FUIE_R = crate::BitReader<bool>;
#[doc = "Field `FUIE` writer - FUIE"]
pub type FUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_IER_SPEC, bool, O>;
#[doc = "Field `TERRIE` reader - TERRIE"]
pub type TERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TERRIE` writer - TERRIE"]
pub type TERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_IER_SPEC, bool, O>;
#[doc = "Field `RRIE` reader - RRIE"]
pub type RRIE_R = crate::BitReader<bool>;
#[doc = "Field `RRIE` writer - RRIE"]
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<0> {
        LIE_W::new(self)
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    #[must_use]
    pub fn fuie(&mut self) -> FUIE_W<1> {
        FUIE_W::new(self)
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<2> {
        TERRIE_W::new(self)
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<3> {
        RRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_ier](index.html) module"]
pub struct LTDC_IER_SPEC;
impl crate::RegisterSpec for LTDC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_ier::R](R) reader structure"]
impl crate::Readable for LTDC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_ier::W](W) writer structure"]
impl crate::Writable for LTDC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_IER to value 0"]
impl crate::Resettable for LTDC_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
