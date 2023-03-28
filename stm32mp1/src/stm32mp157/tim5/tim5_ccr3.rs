#[doc = "Register `TIM5_CCR3` reader"]
pub struct R(crate::R<TIM5_CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_CCR3` writer"]
pub struct W(crate::W<TIM5_CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_CCR3_SPEC>;
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
impl From<crate::W<TIM5_CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_CCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR3` reader - CCR3"]
pub type CCR3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR3` writer - CCR3"]
pub type CCR3_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_CCR3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> CCR3_W<0> {
        CCR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr3](index.html) module"]
pub struct TIM5_CCR3_SPEC;
impl crate::RegisterSpec for TIM5_CCR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim5_ccr3::R](R) reader structure"]
impl crate::Readable for TIM5_CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_ccr3::W](W) writer structure"]
impl crate::Writable for TIM5_CCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_CCR3 to value 0"]
impl crate::Resettable for TIM5_CCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
