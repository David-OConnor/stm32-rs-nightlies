#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWD` reader - Analog watchdog flag"]
pub type AWD_R = crate::BitReader<AWDR_A>;
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDR_A {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWDR_A> for bool {
    #[inline(always)]
    fn from(variant: AWDR_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDR_A {
        match self.bits {
            false => AWDR_A::NoEvent,
            true => AWDR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWDR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWDR_A::Event
    }
}
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDW_AW {
    #[doc = "0: Clear the analog watchdog event flag"]
    Clear = 0,
}
impl From<AWDW_AW> for bool {
    #[inline(always)]
    fn from(variant: AWDW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD` writer - Analog watchdog flag"]
pub type AWD_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, AWDW_AW, O>;
impl<'a, const O: u8> AWD_W<'a, O> {
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWDW_AW::Clear)
    }
}
#[doc = "Field `EOC` reader - Regular channel end of conversion"]
pub type EOC_R = crate::BitReader<EOCR_A>;
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
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
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    #[doc = "0: Clear End of conversion flag"]
    Clear = 0,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - Regular channel end of conversion"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, EOCW_AW, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    #[doc = "Clear End of conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCW_AW::Clear)
    }
}
#[doc = "Field `JEOC` reader - Injected channel end of conversion"]
pub type JEOC_R = crate::BitReader<JEOCR_A>;
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<JEOCR_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCR_A {
        match self.bits {
            false => JEOCR_A::NotComplete,
            true => JEOCR_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR_A::Complete
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW_AW {
    #[doc = "0: Clear Injected channel end of conversion flag"]
    Clear = 0,
}
impl From<JEOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` writer - Injected channel end of conversion"]
pub type JEOC_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, JEOCW_AW, O>;
impl<'a, const O: u8> JEOC_W<'a, O> {
    #[doc = "Clear Injected channel end of conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOCW_AW::Clear)
    }
}
#[doc = "Field `JSTRT` reader - Injected channel start flag"]
pub type JSTRT_R = crate::BitReader<JSTRTR_A>;
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTR_A {
    #[doc = "0: No injected group conversion started"]
    NotStarted = 0,
    #[doc = "1: Injected group conversion has started"]
    Started = 1,
}
impl From<JSTRTR_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl JSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRTR_A {
        match self.bits {
            false => JSTRTR_A::NotStarted,
            true => JSTRTR_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRTR_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRTR_A::Started
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTW_AW {
    #[doc = "0: Clear Injected channel Start flag"]
    Clear = 0,
}
impl From<JSTRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: JSTRTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT` writer - Injected channel start flag"]
pub type JSTRT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, JSTRTW_AW, O>;
impl<'a, const O: u8> JSTRT_W<'a, O> {
    #[doc = "Clear Injected channel Start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JSTRTW_AW::Clear)
    }
}
#[doc = "Field `STRT` reader - Regular channel start flag"]
pub type STRT_R = crate::BitReader<STRTR_A>;
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR_A {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<STRTR_A> for bool {
    #[inline(always)]
    fn from(variant: STRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRTR_A {
        match self.bits {
            false => STRTR_A::NotStarted,
            true => STRTR_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRTR_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRTR_A::Started
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW_AW {
    #[doc = "0: Clear the Regular channel Start flag"]
    Clear = 0,
}
impl From<STRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STRTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` writer - Regular channel start flag"]
pub type STRT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, STRTW_AW, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    #[doc = "Clear the Regular channel Start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STRTW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd(&mut self) -> AWD_W<0> {
        AWD_W::new(self)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<1> {
        EOC_W::new(self)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<2> {
        JEOC_W::new(self)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    #[must_use]
    pub fn jstrt(&mut self) -> JSTRT_W<3> {
        JSTRT_W::new(self)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<4> {
        STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
