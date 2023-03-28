#[doc = "Register `TIM2_AF2` reader"]
pub struct R(crate::R<TIM2_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_AF2` writer"]
pub struct W(crate::W<TIM2_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_AF2_SPEC>;
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
impl From<crate::W<TIM2_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_AF2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<16> {
        OCRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_af2](index.html) module"]
pub struct TIM2_AF2_SPEC;
impl crate::RegisterSpec for TIM2_AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim2_af2::R](R) reader structure"]
impl crate::Readable for TIM2_AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_af2::W](W) writer structure"]
impl crate::Writable for TIM2_AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM2_AF2 to value 0"]
impl crate::Resettable for TIM2_AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
