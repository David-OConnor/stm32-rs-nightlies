#[doc = "Register `CSR1` reader"]
pub struct R(crate::R<CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR1` writer"]
pub struct W(crate::W<CSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR1_SPEC>;
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
impl From<crate::W<CSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
pub type LSEDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
pub type LSECSSON_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 32 kHz oscillator (LSE):"]
pub type LSECSSD_R = crate::BitReader<bool>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00."]
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00."]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTCEN` reader - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `RTCRST` reader - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
pub type RTCRST_R = crate::BitReader<bool>;
#[doc = "Field `RTCRST` writer - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
pub type RTCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_R = crate::BitReader<bool>;
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
pub type LSCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 32 kHz oscillator (LSE):"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (32 kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The RTCRST bit can be used to reset this bitfield to 00."]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrst(&mut self) -> RTCRST_W<16> {
        RTCRST_W::new(self)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC control/status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr1](index.html) module"]
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr1::R](R) reader structure"]
impl crate::Readable for CSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr1::W](W) writer structure"]
impl crate::Writable for CSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
