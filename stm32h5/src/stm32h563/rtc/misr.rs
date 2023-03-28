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
#[doc = "Field `ALRAMF` reader - Alarm A masked flag This flag is set by hardware when the alarm A non-secure interrupt occurs."]
pub type ALRAMF_R = crate::BitReader<bool>;
#[doc = "Field `ALRBMF` reader - Alarm B non-secure masked flag This flag is set by hardware when the alarm B non-secure interrupt occurs."]
pub type ALRBMF_R = crate::BitReader<bool>;
#[doc = "Field `WUTMF` reader - Wakeup timer non-secure masked flag This flag is set by hardware when the wakeup timer non-secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WUTMF_R = crate::BitReader<bool>;
#[doc = "Field `TSMF` reader - Timestamp non-secure masked flag This flag is set by hardware when a timestamp non-secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
pub type TSMF_R = crate::BitReader<bool>;
#[doc = "Field `TSOVMF` reader - Timestamp overflow non-secure masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TSOVMF_R = crate::BitReader<bool>;
#[doc = "Field `ITSMF` reader - Internal timestamp non-secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp non-secure interrupt is raised."]
pub type ITSMF_R = crate::BitReader<bool>;
#[doc = "Field `SSRUMF` reader - SSR underflow non-secure masked flag This flag is set by hardware when the SSR underflow non-secure interrupt occurs."]
pub type SSRUMF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Alarm A masked flag This flag is set by hardware when the alarm A non-secure interrupt occurs."]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B non-secure masked flag This flag is set by hardware when the alarm B non-secure interrupt occurs."]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer non-secure masked flag This flag is set by hardware when the wakeup timer non-secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp non-secure masked flag This flag is set by hardware when a timestamp non-secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow non-secure masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp non-secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp non-secure interrupt is raised."]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow non-secure masked flag This flag is set by hardware when the SSR underflow non-secure interrupt occurs."]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "RTC non-secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
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
