#[doc = "Register `TIM3_CCMR1_Output` reader"]
pub struct R(crate::R<TIM3_CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_CCMR1_Output` writer"]
pub struct W(crate::W<TIM3_CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<TIM3_CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_R = crate::BitReader<bool>;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable"]
pub type OC1PE_R = crate::BitReader<bool>;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable"]
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1M1` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC1M1` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC1CE` reader - Output compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader<bool>;
#[doc = "Field `OC1CE` writer - Output compare 1 clear enable"]
pub type OC1CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC2FE` reader - Output compare 2 fast enable"]
pub type OC2FE_R = crate::BitReader<bool>;
#[doc = "Field `OC2FE` writer - Output compare 2 fast enable"]
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader<bool>;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC2M1` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC2M1` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC2CE` reader - Output compare 2 clear enable"]
pub type OC2CE_R = crate::BitReader<bool>;
#[doc = "Field `OC2CE` writer - Output compare 2 clear enable"]
pub type OC2CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1M2` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_R = crate::BitReader<bool>;
#[doc = "Field `OC1M2` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC2M2` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_R = crate::BitReader<bool>;
#[doc = "Field `OC2M2` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_CCMR1_OUTPUT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn oc2m1(&self) -> OC2M1_R {
        OC2M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn oc2m2(&self) -> OC2M2_R {
        OC2M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> OC1M1_W<4> {
        OC1M1_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<7> {
        OC1CE_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<10> {
        OC2FE_W::new(self)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<11> {
        OC2PE_W::new(self)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m1(&mut self) -> OC2M1_W<12> {
        OC2M1_W::new(self)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<15> {
        OC2CE_W::new(self)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> OC1M2_W<16> {
        OC1M2_W::new(self)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m2(&mut self) -> OC2M2_W<24> {
        OC2M2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 capture/compare mode register 1 \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccmr1_output](index.html) module"]
pub struct TIM3_CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for TIM3_CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim3_ccmr1_output::R](R) reader structure"]
impl crate::Readable for TIM3_CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_ccmr1_output::W](W) writer structure"]
impl crate::Writable for TIM3_CCMR1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM3_CCMR1_Output to value 0"]
impl crate::Resettable for TIM3_CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
