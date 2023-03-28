#[doc = "Register `TIM16_CCR1` reader"]
pub struct R(crate::R<TIM16_CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM16_CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM16_CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM16_CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM16_CCR1` writer"]
pub struct W(crate::W<TIM16_CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM16_CCR1_SPEC>;
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
impl From<crate::W<TIM16_CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM16_CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1` reader - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (tim_ic1). Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
pub type CCR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCR1` writer - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (tim_ic1). Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
pub type CCR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM16_CCR1_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (tim_ic1). Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (tim_ic1). Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<0> {
        CCR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim16_ccr1](index.html) module"]
pub struct TIM16_CCR1_SPEC;
impl crate::RegisterSpec for TIM16_CCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim16_ccr1::R](R) reader structure"]
impl crate::Readable for TIM16_CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim16_ccr1::W](W) writer structure"]
impl crate::Writable for TIM16_CCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM16_CCR1 to value 0"]
impl crate::Resettable for TIM16_CCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
