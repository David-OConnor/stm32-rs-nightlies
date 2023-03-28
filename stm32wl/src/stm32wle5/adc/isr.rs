#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type ADRDY_R = crate::BitReader<ADRDYR_A>;
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR_A {
    #[doc = "0: ADC not yet ready to start conversion"]
    NotReady = 0,
    #[doc = "1: ADC ready to start conversion"]
    Ready = 1,
}
impl From<ADRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDYR_A {
        match self.bits {
            false => ADRDYR_A::NotReady,
            true => ADRDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR_A::Ready
    }
}
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW_AW {
    #[doc = "1: Clear the ADC ready flag"]
    Clear = 1,
}
impl From<ADRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, ADRDYW_AW, O>;
impl<'a, const O: u8> ADRDY_W<'a, O> {
    #[doc = "Clear the ADC ready flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDYW_AW::Clear)
    }
}
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader<EOSMPR_A>;
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR_A {
    #[doc = "0: Not at the end of the samplings phase"]
    NotAtEnd = 0,
    #[doc = "1: End of sampling phase reached"]
    AtEnd = 1,
}
impl From<EOSMPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMPR_A {
        match self.bits {
            false => EOSMPR_A::NotAtEnd,
            true => EOSMPR_A::AtEnd,
        }
    }
    #[doc = "Checks if the value of the field is `NotAtEnd`"]
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        *self == EOSMPR_A::NotAtEnd
    }
    #[doc = "Checks if the value of the field is `AtEnd`"]
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        *self == EOSMPR_A::AtEnd
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW_AW {
    #[doc = "1: Clear the sampling phase flag"]
    Clear = 1,
}
impl From<EOSMPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOSMPW_AW, O>;
impl<'a, const O: u8> EOSMP_W<'a, O> {
    #[doc = "Clear the sampling phase flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMPW_AW::Clear)
    }
}
#[doc = "Field `EOC` reader - EOC"]
pub type EOC_R = crate::BitReader<EOCR_A>;
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    #[doc = "0: Channel conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Channel conversion complete"]
    Complete = 1,
}
impl From<EOCR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCR_A {
        match self.bits {
            false => EOCR_A::NotComplete,
            true => EOCR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR_A::Complete
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    #[doc = "1: Clear the channel conversion flag"]
    Clear = 1,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOCW_AW, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    #[doc = "Clear the channel conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCW_AW::Clear)
    }
}
#[doc = "Field `EOS` reader - EOS"]
pub type EOS_R = crate::BitReader<EOSR_A>;
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR_A {
    #[doc = "0: Conversion sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion sequence complete"]
    Complete = 1,
}
impl From<EOSR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR_A {
        match self.bits {
            false => EOSR_A::NotComplete,
            true => EOSR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR_A::Complete
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW_AW {
    #[doc = "1: Clear the conversion sequence flag"]
    Clear = 1,
}
impl From<EOSW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOSW_AW, O>;
impl<'a, const O: u8> EOS_W<'a, O> {
    #[doc = "Clear the conversion sequence flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSW_AW::Clear)
    }
}
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader<OVRR_A>;
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NoOverrun,
            true => OVRR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::Overrun
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW_AW {
    #[doc = "1: Clear the overrun flag"]
    Clear = 1,
}
impl From<OVRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, OVRW_AW, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "Clear the overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRW_AW::Clear)
    }
}
#[doc = "Field `AWD1` reader - AWD1"]
pub type AWD1_R = crate::BitReader<AWD1R_A>;
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R_A {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1R_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1R_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1R_A {
        match self.bits {
            false => AWD1R_A::NoEvent,
            true => AWD1R_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R_A::Event
    }
}
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W_AW {
    #[doc = "1: Clear the analog watchdog event flag"]
    Clear = 1,
}
impl From<AWD1W_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, AWD1W_AW, O>;
impl<'a, const O: u8> AWD1_W<'a, O> {
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD1W_AW::Clear)
    }
}
#[doc = "Field `AWD2` reader - AWD2"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - AWD3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` writer - AWD2"]
pub use AWD1_W as AWD2_W;
#[doc = "Field `AWD3` writer - AWD3"]
pub use AWD1_W as AWD3_W;
#[doc = "Field `EOCAL` reader - EOCAL"]
pub type EOCAL_R = crate::BitReader<EOCALR_A>;
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALR_A {
    #[doc = "0: Calibration is not complete"]
    NotComplete = 0,
    #[doc = "1: Calibration complete"]
    Complete = 1,
}
impl From<EOCALR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCALR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCALR_A {
        match self.bits {
            false => EOCALR_A::NotComplete,
            true => EOCALR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCALR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCALR_A::Complete
    }
}
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALW_AW {
    #[doc = "1: Clear the calibration flag"]
    Clear = 1,
}
impl From<EOCALW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCALW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` writer - EOCAL"]
pub type EOCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOCALW_AW, O>;
impl<'a, const O: u8> EOCAL_W<'a, O> {
    #[doc = "Clear the calibration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCALW_AW::Clear)
    }
}
#[doc = "Field `CCRDY` reader - CCRDY"]
pub type CCRDY_R = crate::BitReader<CCRDYR_A>;
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYR_A {
    #[doc = "0: Channel configuration update not applied"]
    NotComplete = 0,
    #[doc = "1: Channel configuration update is applied"]
    Complete = 1,
}
impl From<CCRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRDYR_A {
        match self.bits {
            false => CCRDYR_A::NotComplete,
            true => CCRDYR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCRDYR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCRDYR_A::Complete
    }
}
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYW_AW {
    #[doc = "1: Clear the channel configuration flag"]
    Clear = 1,
}
impl From<CCRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` writer - CCRDY"]
pub type CCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, CCRDYW_AW, O>;
impl<'a, const O: u8> CCRDY_W<'a, O> {
    #[doc = "Clear the channel configuration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRDYW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<11> {
        EOCAL_W::new(self)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    #[must_use]
    pub fn ccrdy(&mut self) -> CCRDY_W<13> {
        CCRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
