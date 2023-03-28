#[doc = "Register `ALRMASSR` reader"]
pub struct R(crate::R<ALRMASSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMASSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMASSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMASSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRMASSR` writer"]
pub struct W(crate::W<ALRMASSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMASSR_SPEC>;
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
impl From<crate::W<ALRMASSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMASSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS` reader - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
pub type SS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SS` writer - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
pub type SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMASSR_SPEC, u16, u16, 15, O>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
pub type MASKSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
pub type MASKSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMASSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SSCLR_R = crate::BitReader<bool>;
#[doc = "Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMASSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<0> {
        SS_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<24> {
        MASKSS_W::new(self)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    #[must_use]
    pub fn ssclr(&mut self) -> SSCLR_W<31> {
        SSCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm A subsecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmassr](index.html) module"]
pub struct ALRMASSR_SPEC;
impl crate::RegisterSpec for ALRMASSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrmassr::R](R) reader structure"]
impl crate::Readable for ALRMASSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrmassr::W](W) writer structure"]
impl crate::Writable for ALRMASSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRMASSR to value 0"]
impl crate::Resettable for ALRMASSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
