#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1_RMP` reader - Timer 17 input 1 connection"]
pub type TI1_RMP_R = crate::FieldReader<u8, TI1_RMP_A>;
#[doc = "Timer 17 input 1 connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP_A {
    #[doc = "0: TI1 is connected to GPIO"]
    Gpio = 0,
    #[doc = "1: TI1 is connected to LSI"]
    Lsi = 1,
    #[doc = "2: TI1 is connected to LSE"]
    Lse = 2,
    #[doc = "3: TI1 is connected to RTC wake-up interrupt"]
    Rtc = 3,
}
impl From<TI1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1_RMP_A) -> Self {
        variant as _
    }
}
impl TI1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1_RMP_A {
        match self.bits {
            0 => TI1_RMP_A::Gpio,
            1 => TI1_RMP_A::Lsi,
            2 => TI1_RMP_A::Lse,
            3 => TI1_RMP_A::Rtc,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Gpio`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP_A::Gpio
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == TI1_RMP_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == TI1_RMP_A::Lse
    }
    #[doc = "Checks if the value of the field is `Rtc`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == TI1_RMP_A::Rtc
    }
}
#[doc = "Field `TI1_RMP` writer - Timer 17 input 1 connection"]
pub type TI1_RMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OR1_SPEC, u8, TI1_RMP_A, 2, O>;
impl<'a, const O: u8> TI1_RMP_W<'a, O> {
    #[doc = "TI1 is connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Gpio)
    }
    #[doc = "TI1 is connected to LSI"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Lsi)
    }
    #[doc = "TI1 is connected to LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Lse)
    }
    #[doc = "TI1 is connected to RTC wake-up interrupt"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(TI1_RMP_A::Rtc)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer 17 input 1 connection"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer 17 input 1 connection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<0> {
        TI1_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
