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
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader<ALRAF_A>;
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAF_A> {
        match self.bits {
            true => Some(ALRAF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAF_A::Match
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader<ALRBF_A>;
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBF_A> {
        match self.bits {
            true => Some(ALRBF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBF_A::Match
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader<WUTF_A>;
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTF_A> {
        match self.bits {
            true => Some(WUTF_A::Zero),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Zero`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTF_A::Zero
    }
}
#[doc = "Field `TSF` reader - Timestamp flag"]
pub type TSF_R = crate::BitReader<TSF_A>;
#[doc = "Timestamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSF_A> {
        match self.bits {
            true => Some(TSF_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSF_A::TimestampEvent
    }
}
#[doc = "Field `TSOVF` reader - Timestamp overflow flag"]
pub type TSOVF_R = crate::BitReader<TSOVF_A>;
#[doc = "Timestamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVF_A> {
        match self.bits {
            true => Some(TSOVF_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Overflow`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVF_A::Overflow
    }
}
#[doc = "Field `ITSF` reader - Internal timestamp flag"]
pub type ITSF_R = crate::BitReader<ITSF_A>;
#[doc = "Internal timestamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSF_A {
    #[doc = "1: This flag is set by hardware when a timestamp on the internal event occurs"]
    TimestampEvent = 1,
}
impl From<ITSF_A> for bool {
    #[inline(always)]
    fn from(variant: ITSF_A) -> Self {
        variant as u8 != 0
    }
}
impl ITSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITSF_A> {
        match self.bits {
            true => Some(ITSF_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == ITSF_A::TimestampEvent
    }
}
#[doc = "Field `SSRUF` reader - SSR underflow flag"]
pub type SSRUF_R = crate::BitReader<SSRUF_A>;
#[doc = "SSR underflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUF_A {
    #[doc = "1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    Underflow = 1,
}
impl From<SSRUF_A> for bool {
    #[inline(always)]
    fn from(variant: SSRUF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSRUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSRUF_A> {
        match self.bits {
            true => Some(SSRUF_A::Underflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Underflow`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == SSRUF_A::Underflow
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp flag"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow flag"]
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status register (interrupts)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
