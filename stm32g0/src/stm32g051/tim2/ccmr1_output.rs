#[doc = "Register `CCMR1_Output` reader"]
pub struct R(crate::R<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR1_Output` writer"]
pub struct W(crate::W<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable"]
pub type OC1FE_R = crate::BitReader<bool>;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable"]
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_R = crate::BitReader<bool>;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1M` reader - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M_R = crate::FieldReader<u8, OC1M_A>;
#[doc = "Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M_A {
    #[doc = "0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    Frozen = 0,
    #[doc = "1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    ActiveOnMatch = 1,
    #[doc = "2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    InactiveOnMatch = 2,
    #[doc = "3: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    Toggle = 3,
    #[doc = "4: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    ForceInactive = 4,
    #[doc = "5: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    ForceActive = 5,
    #[doc = "6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    PwmMode1 = 6,
    #[doc = "7: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    PwmMode2 = 7,
}
impl From<OC1M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC1M_A) -> Self {
        variant as _
    }
}
impl OC1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1M_A {
        match self.bits {
            0 => OC1M_A::Frozen,
            1 => OC1M_A::ActiveOnMatch,
            2 => OC1M_A::InactiveOnMatch,
            3 => OC1M_A::Toggle,
            4 => OC1M_A::ForceInactive,
            5 => OC1M_A::ForceActive,
            6 => OC1M_A::PwmMode1,
            7 => OC1M_A::PwmMode2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Frozen`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC1M_A::Frozen
    }
    #[doc = "Checks if the value of the field is `ActiveOnMatch`"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC1M_A::ActiveOnMatch
    }
    #[doc = "Checks if the value of the field is `InactiveOnMatch`"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC1M_A::InactiveOnMatch
    }
    #[doc = "Checks if the value of the field is `Toggle`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC1M_A::Toggle
    }
    #[doc = "Checks if the value of the field is `ForceInactive`"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1M_A::ForceInactive
    }
    #[doc = "Checks if the value of the field is `ForceActive`"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC1M_A::ForceActive
    }
    #[doc = "Checks if the value of the field is `PwmMode1`"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC1M_A::PwmMode1
    }
    #[doc = "Checks if the value of the field is `PwmMode2`"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC1M_A::PwmMode2
    }
}
#[doc = "Field `OC1M` writer - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_OUTPUT_SPEC, u8, OC1M_A, 3, O>;
impl<'a, const O: u8> OC1M_W<'a, O> {
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC1M_A::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC1M_A::Toggle)
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC1M_A::ForceInactive)
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC1M_A::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC1M_A::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC1M_A::PwmMode2)
    }
}
#[doc = "Field `OC1CE` reader - Output compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader<bool>;
#[doc = "Field `OC1CE` writer - Output compare 1 clear enable"]
pub type OC1CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CC2S` reader - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC2FE` reader - Output compare 2 fast enable"]
pub type OC2FE_R = crate::BitReader<bool>;
#[doc = "Field `OC2FE` writer - Output compare 2 fast enable"]
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader<bool>;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC2M` reader - Output compare 2 mode"]
pub use OC1M_R as OC2M_R;
#[doc = "Field `OC2M` writer - Output compare 2 mode"]
pub use OC1M_W as OC2M_W;
#[doc = "Field `OC2CE` reader - Output compare 2 clear enable"]
pub type OC2CE_R = crate::BitReader<bool>;
#[doc = "Field `OC2CE` writer - Output compare 2 clear enable"]
pub type OC2CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC1M_3` reader - Output Compare 1 mode - bit 3"]
pub type OC1M_3_R = crate::BitReader<OC1M_3_A>;
#[doc = "Output Compare 1 mode - bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1M_3_A {
    #[doc = "0: Normal output compare mode (modes 0-7)"]
    Normal = 0,
    #[doc = "1: Extended output compare mode (modes 7-15)"]
    Extended = 1,
}
impl From<OC1M_3_A> for bool {
    #[inline(always)]
    fn from(variant: OC1M_3_A) -> Self {
        variant as u8 != 0
    }
}
impl OC1M_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OC1M_3_A {
        match self.bits {
            false => OC1M_3_A::Normal,
            true => OC1M_3_A::Extended,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC1M_3_A::Normal
    }
    #[doc = "Checks if the value of the field is `Extended`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC1M_3_A::Extended
    }
}
#[doc = "Field `OC1M_3` writer - Output Compare 1 mode - bit 3"]
pub type OC1M_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_OUTPUT_SPEC, OC1M_3_A, O>;
impl<'a, const O: u8> OC1M_3_W<'a, O> {
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OC1M_3_A::Normal)
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(OC1M_3_A::Extended)
    }
}
#[doc = "Field `OC2M_3` reader - Output Compare 2 mode - bit 3"]
pub use OC1M_3_R as OC2M_3_R;
#[doc = "Field `OC2M_3` writer - Output Compare 2 mode - bit 3"]
pub use OC1M_3_W as OC2M_3_W;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
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
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. Note: The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<4> {
        OC1M_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<7> {
        OC1CE_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
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
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<12> {
        OC2M_W::new(self)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<15> {
        OC2CE_W::new(self)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<16> {
        OC1M_3_W::new(self)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<24> {
        OC2M_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_output](index.html) module"]
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr1_output::R](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr1_output::W](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
