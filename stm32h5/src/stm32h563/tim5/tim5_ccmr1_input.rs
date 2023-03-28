#[doc = "Register `TIM5_CCMR1_Input` reader"]
pub struct R(crate::R<TIM5_CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_CCMR1_Input` writer"]
pub struct W(crate::W<TIM5_CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_CCMR1_INPUT_SPEC>;
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
impl From<crate::W<TIM5_CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
pub type IC1PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type IC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type IC1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `CC2S` reader - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type IC2PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_CCMR1_INPUT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<10> {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<12> {
        IC2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 capture/compare mode register 1 \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccmr1_input](index.html) module"]
pub struct TIM5_CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for TIM5_CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim5_ccmr1_input::R](R) reader structure"]
impl crate::Readable for TIM5_CCMR1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_ccmr1_input::W](W) writer structure"]
impl crate::Writable for TIM5_CCMR1_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_CCMR1_Input to value 0"]
impl crate::Resettable for TIM5_CCMR1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
