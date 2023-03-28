#[doc = "Register `TIM2_EGR` writer"]
pub struct W(crate::W<TIM2_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_EGR_SPEC>;
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
impl From<crate::W<TIM2_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM2_EGR_SPEC, bool, O>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM2_EGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_egr](index.html) module"]
pub struct TIM2_EGR_SPEC;
impl crate::RegisterSpec for TIM2_EGR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [tim2_egr::W](W) writer structure"]
impl crate::Writable for TIM2_EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM2_EGR to value 0"]
impl crate::Resettable for TIM2_EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
