#[doc = "Register `TIM4_CCR4` reader"]
pub struct R(crate::R<TIM4_CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM4_CCR4` writer"]
pub struct W(crate::W<TIM4_CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CCR4_SPEC>;
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
impl From<crate::W<TIM4_CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR4` reader - CCR4"]
pub type CCR4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR4` writer - CCR4"]
pub type CCR4_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM4_CCR4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<0> {
        CCR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM4 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr4](index.html) module"]
pub struct TIM4_CCR4_SPEC;
impl crate::RegisterSpec for TIM4_CCR4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim4_ccr4::R](R) reader structure"]
impl crate::Readable for TIM4_CCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim4_ccr4::W](W) writer structure"]
impl crate::Writable for TIM4_CCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM4_CCR4 to value 0"]
impl crate::Resettable for TIM4_CCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
