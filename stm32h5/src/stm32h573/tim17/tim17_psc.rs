#[doc = "Register `TIM17_PSC` reader"]
pub struct R(crate::R<TIM17_PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM17_PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM17_PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM17_PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM17_PSC` writer"]
pub struct W(crate::W<TIM17_PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM17_PSC_SPEC>;
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
impl From<crate::W<TIM17_PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM17_PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - Prescaler value The counter clock frequency (tim_cnt_ck) is equal to ftim_psc_ck / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in “reset mode”)."]
pub type PSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSC` writer - Prescaler value The counter clock frequency (tim_cnt_ck) is equal to ftim_psc_ck / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in “reset mode”)."]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM17_PSC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (tim_cnt_ck) is equal to ftim_psc_ck / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in “reset mode”)."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (tim_cnt_ck) is equal to ftim_psc_ck / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in “reset mode”)."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim17_psc](index.html) module"]
pub struct TIM17_PSC_SPEC;
impl crate::RegisterSpec for TIM17_PSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim17_psc::R](R) reader structure"]
impl crate::Readable for TIM17_PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim17_psc::W](W) writer structure"]
impl crate::Writable for TIM17_PSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM17_PSC to value 0"]
impl crate::Resettable for TIM17_PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
