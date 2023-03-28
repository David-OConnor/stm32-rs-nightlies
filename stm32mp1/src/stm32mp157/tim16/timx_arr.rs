#[doc = "Register `TIMx_ARR` reader"]
pub struct R(crate::R<TIMX_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_ARR` writer"]
pub struct W(crate::W<TIMX_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_ARR_SPEC>;
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
impl From<crate::W<TIMX_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - ARR"]
pub type ARR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARR` writer - ARR"]
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIMX_ARR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_arr](index.html) module"]
pub struct TIMX_ARR_SPEC;
impl crate::RegisterSpec for TIMX_ARR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_arr::R](R) reader structure"]
impl crate::Readable for TIMX_ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_arr::W](W) writer structure"]
impl crate::Writable for TIMX_ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMx_ARR to value 0xffff"]
impl crate::Resettable for TIMX_ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
