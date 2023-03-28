#[doc = "Register `RTCCR` reader"]
pub struct R(crate::R<RTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCR` writer"]
pub struct W(crate::W<RTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCR_SPEC>;
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
impl From<crate::W<RTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - Calibration value"]
pub type CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL` writer - Calibration value"]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CCO` reader - Calibration Clock Output"]
pub type CCO_R = crate::BitReader<bool>;
#[doc = "Field `CCO` writer - Calibration Clock Output"]
pub type CCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, bool, O>;
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type ASOE_R = crate::BitReader<ASOE_A>;
#[doc = "Alarm or second output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOE_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    Enabled = 1,
}
impl From<ASOE_A> for bool {
    #[inline(always)]
    fn from(variant: ASOE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOE_A {
        match self.bits {
            false => ASOE_A::Disabled,
            true => ASOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASOE_A::Enabled
    }
}
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type ASOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, ASOE_A, O>;
impl<'a, const O: u8> ASOE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASOE_A::Disabled)
    }
    #[doc = "Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASOE_A::Enabled)
    }
}
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type ASOS_R = crate::BitReader<ASOS_A>;
#[doc = "Alarm or second output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOS_A {
    #[doc = "0: RTC Alarm pulse output selected"]
    Alarm = 0,
    #[doc = "1: RTC Second pulse output selected"]
    Second = 1,
}
impl From<ASOS_A> for bool {
    #[inline(always)]
    fn from(variant: ASOS_A) -> Self {
        variant as u8 != 0
    }
}
impl ASOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOS_A {
        match self.bits {
            false => ASOS_A::Alarm,
            true => ASOS_A::Second,
        }
    }
    #[doc = "Checks if the value of the field is `Alarm`"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ASOS_A::Alarm
    }
    #[doc = "Checks if the value of the field is `Second`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == ASOS_A::Second
    }
}
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type ASOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, ASOS_A, O>;
impl<'a, const O: u8> ASOS_W<'a, O> {
    #[doc = "RTC Alarm pulse output selected"]
    #[inline(always)]
    pub fn alarm(self) -> &'a mut W {
        self.variant(ASOS_A::Alarm)
    }
    #[doc = "RTC Second pulse output selected"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(ASOS_A::Second)
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn cco(&mut self) -> CCO_W<7> {
        CCO_W::new(self)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    #[must_use]
    pub fn asoe(&mut self) -> ASOE_W<8> {
        ASOE_W::new(self)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    #[must_use]
    pub fn asos(&mut self) -> ASOS_W<9> {
        ASOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr](index.html) module"]
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccr::R](R) reader structure"]
impl crate::Readable for RTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccr::W](W) writer structure"]
impl crate::Writable for RTCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
