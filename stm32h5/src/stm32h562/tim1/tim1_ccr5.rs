#[doc = "Register `TIM1_CCR5` reader"]
pub struct R(crate::R<TIM1_CCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_CCR5` writer"]
pub struct W(crate::W<TIM1_CCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CCR5_SPEC>;
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
impl From<crate::W<TIM1_CCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR5` reader - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCR5` writer - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_CCR5_SPEC, u32, u32, 20, O>;
#[doc = "Field `GC5C1` reader - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_R = crate::BitReader<bool>;
#[doc = "Field `GC5C1` writer - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCR5_SPEC, bool, O>;
#[doc = "Field `GC5C2` reader - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_R = crate::BitReader<bool>;
#[doc = "Field `GC5C2` writer - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCR5_SPEC, bool, O>;
#[doc = "Field `GC5C3` reader - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_R = crate::BitReader<bool>;
#[doc = "Field `GC5C3` writer - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCR5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<0> {
        CCR5_W::new(self)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<29> {
        GC5C1_W::new(self)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<30> {
        GC5C2_W::new(self)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<31> {
        GC5C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr5](index.html) module"]
pub struct TIM1_CCR5_SPEC;
impl crate::RegisterSpec for TIM1_CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_ccr5::R](R) reader structure"]
impl crate::Readable for TIM1_CCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_ccr5::W](W) writer structure"]
impl crate::Writable for TIM1_CCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1_CCR5 to value 0"]
impl crate::Resettable for TIM1_CCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
