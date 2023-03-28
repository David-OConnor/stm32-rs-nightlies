#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enabled Set and reset by software."]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - LSE oscillator enabled Set and reset by software."]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSERDY` writer - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSEDRV` reader - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSEDRV` writer - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LSECSSON` reader - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSON` writer - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSECSSD` reader - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSD` writer - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSEEXT` reader - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_R = crate::BitReader<bool>;
#[doc = "Field `LSEEXT` writer - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTCEN` reader - RTC clock enable Set and reset by software."]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable Set and reset by software."]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `VSWRST` reader - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_R = crate::BitReader<bool>;
#[doc = "Field `VSWRST` writer - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_R = crate::BitReader<bool>;
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software."]
pub type LSCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software."]
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software."]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software."]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDY` writer - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    pub fn lseext(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<1> {
        LSERDY_W::new(self)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn lsecssd(&mut self) -> LSECSSD_W<6> {
        LSECSSD_W::new(self)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn lseext(&mut self) -> LSEEXT_W<7> {
        LSEEXT_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vswrst(&mut self) -> VSWRST_W<16> {
        VSWRST_W::new(self)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<26> {
        LSION_W::new(self)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<27> {
        LSIRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
