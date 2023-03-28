#[doc = "Register `TIM5_SR` reader"]
pub struct R(crate::R<TIM5_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_SR` writer"]
pub struct W(crate::W<TIM5_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_SR_SPEC>;
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
impl From<crate::W<TIM5_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_R = crate::BitReader<bool>;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_R = crate::BitReader<bool>;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_R = crate::BitReader<bool>;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub type CC2OF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub type CC2OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type CC3OF_R = crate::BitReader<bool>;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type CC3OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type CC4OF_R = crate::BitReader<bool>;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type CC4OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `IDXF` reader - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
pub type IDXF_R = crate::BitReader<bool>;
#[doc = "Field `IDXF` writer - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
pub type IDXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `DIRF` reader - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
pub type DIRF_R = crate::BitReader<bool>;
#[doc = "Field `DIRF` writer - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
pub type DIRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `IERRF` reader - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
pub type IERRF_R = crate::BitReader<bool>;
#[doc = "Field `IERRF` writer - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
pub type IERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
#[doc = "Field `TERRF` reader - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
pub type TERRF_R = crate::BitReader<bool>;
#[doc = "Field `TERRF` writer - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
pub type TERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM5_SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn idxf(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn ierrf(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn terrf(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<2> {
        CC2IF_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<3> {
        CC3IF_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<4> {
        CC4IF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<6> {
        TIF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<10> {
        CC2OF_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<11> {
        CC3OF_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<12> {
        CC4OF_W::new(self)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn idxf(&mut self) -> IDXF_W<20> {
        IDXF_W::new(self)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn dirf(&mut self) -> DIRF_W<21> {
        DIRF_W::new(self)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn ierrf(&mut self) -> IERRF_W<22> {
        IERRF_W::new(self)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn terrf(&mut self) -> TERRF_W<23> {
        TERRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_sr](index.html) module"]
pub struct TIM5_SR_SPEC;
impl crate::RegisterSpec for TIM5_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim5_sr::R](R) reader structure"]
impl crate::Readable for TIM5_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_sr::W](W) writer structure"]
impl crate::Writable for TIM5_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_SR to value 0"]
impl crate::Resettable for TIM5_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
