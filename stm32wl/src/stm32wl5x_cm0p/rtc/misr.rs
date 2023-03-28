#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub type ALRAMF_R = crate::BitReader<ALRAMF_A>;
#[doc = "Alarm A masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAMF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAMF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAMF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAMF_A> {
        match self.bits {
            true => Some(ALRAMF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAMF_A::Match
    }
}
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub type ALRBMF_R = crate::BitReader<ALRBMF_A>;
#[doc = "Alarm B masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBMF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBMF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBMF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBMF_A> {
        match self.bits {
            true => Some(ALRBMF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBMF_A::Match
    }
}
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub type WUTMF_R = crate::BitReader<WUTMF_A>;
#[doc = "Wakeup timer masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTMF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTMF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTMF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTMF_A> {
        match self.bits {
            true => Some(WUTMF_A::Zero),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Zero`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTMF_A::Zero
    }
}
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub type TSMF_R = crate::BitReader<TSMF_A>;
#[doc = "Timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSMF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSMF_A> for bool {
    #[inline(always)]
    fn from(variant: TSMF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSMF_A> {
        match self.bits {
            true => Some(TSMF_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSMF_A::TimestampEvent
    }
}
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub type TSOVMF_R = crate::BitReader<TSOVMF_A>;
#[doc = "Timestamp overflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVMF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVMF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVMF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOVMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVMF_A> {
        match self.bits {
            true => Some(TSOVMF_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Overflow`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVMF_A::Overflow
    }
}
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub type ITSMF_R = crate::BitReader<ITSMF_A>;
#[doc = "Internal timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSMF_A {
    #[doc = "1: This flag is set by hardware when a timestamp on the internal event occurs"]
    TimestampEvent = 1,
}
impl From<ITSMF_A> for bool {
    #[inline(always)]
    fn from(variant: ITSMF_A) -> Self {
        variant as u8 != 0
    }
}
impl ITSMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITSMF_A> {
        match self.bits {
            true => Some(ITSMF_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == ITSMF_A::TimestampEvent
    }
}
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag"]
pub type SSRUMF_R = crate::BitReader<SSRUMF_A>;
#[doc = "SSR underflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUMF_A {
    #[doc = "1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    Underflow = 1,
}
impl From<SSRUMF_A> for bool {
    #[inline(always)]
    fn from(variant: SSRUMF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSRUMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSRUMF_A> {
        match self.bits {
            true => Some(SSRUMF_A::Underflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Underflow`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == SSRUMF_A::Underflow
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
