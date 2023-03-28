#[doc = "Register `CALR` reader"]
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALR` writer"]
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALR_SPEC, u16, u16, 9, O>;
#[doc = "Field `LPCAL` reader - RTC low-power mode"]
pub type LPCAL_R = crate::BitReader<bool>;
#[doc = "Field `LPCAL` writer - RTC low-power mode"]
pub type LPCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_R = crate::BitReader<bool>;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_R = crate::BitReader<bool>;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_R = crate::BitReader<bool>;
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<0> {
        CALM_W::new(self)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpcal(&mut self) -> LPCAL_W<12> {
        LPCAL_W::new(self)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<13> {
        CALW16_W::new(self)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<14> {
        CALW8_W::new(self)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<15> {
        CALP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](index.html) module"]
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calr::R](R) reader structure"]
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calr::W](W) writer structure"]
impl crate::Writable for CALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
