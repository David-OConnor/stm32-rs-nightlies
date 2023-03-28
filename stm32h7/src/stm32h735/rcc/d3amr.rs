#[doc = "Register `D3AMR` reader"]
pub struct R(crate::R<D3AMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3AMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3AMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3AMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3AMR` writer"]
pub struct W(crate::W<D3AMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3AMR_SPEC>;
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
impl From<crate::W<D3AMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3AMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDMAAMEN` reader - BDMA and DMAMUX Autonomous mode enable"]
pub type BDMAAMEN_R = crate::BitReader<BDMAAMEN_A>;
#[doc = "BDMA and DMAMUX Autonomous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDMAAMEN_A {
    #[doc = "0: Clock disabled in autonomous mode"]
    Disabled = 0,
    #[doc = "1: Clock enabled in autonomous mode"]
    Enabled = 1,
}
impl From<BDMAAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDMAAMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BDMAAMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDMAAMEN_A {
        match self.bits {
            false => BDMAAMEN_A::Disabled,
            true => BDMAAMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDMAAMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDMAAMEN_A::Enabled
    }
}
#[doc = "Field `BDMAAMEN` writer - BDMA and DMAMUX Autonomous mode enable"]
pub type BDMAAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, D3AMR_SPEC, BDMAAMEN_A, O>;
impl<'a, const O: u8> BDMAAMEN_W<'a, O> {
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDMAAMEN_A::Disabled)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDMAAMEN_A::Enabled)
    }
}
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable"]
pub use BDMAAMEN_R as LPUART1AMEN_R;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable"]
pub use BDMAAMEN_R as SPI6AMEN_R;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable"]
pub use BDMAAMEN_R as I2C4AMEN_R;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable"]
pub use BDMAAMEN_R as LPTIM2AMEN_R;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable"]
pub use BDMAAMEN_R as LPTIM3AMEN_R;
#[doc = "Field `LPTIM4AMEN` reader - LPTIM4 Autonomous mode enable"]
pub use BDMAAMEN_R as LPTIM4AMEN_R;
#[doc = "Field `LPTIM5AMEN` reader - LPTIM5 Autonomous mode enable"]
pub use BDMAAMEN_R as LPTIM5AMEN_R;
#[doc = "Field `COMP12AMEN` reader - COMP12 Autonomous mode enable"]
pub use BDMAAMEN_R as COMP12AMEN_R;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable"]
pub use BDMAAMEN_R as VREFAMEN_R;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable"]
pub use BDMAAMEN_R as RTCAMEN_R;
#[doc = "Field `CRCAMEN` reader - CRC Autonomous mode enable"]
pub use BDMAAMEN_R as CRCAMEN_R;
#[doc = "Field `SAI4AMEN` reader - SAI4 Autonomous mode enable"]
pub use BDMAAMEN_R as SAI4AMEN_R;
#[doc = "Field `ADC3AMEN` reader - ADC3 Autonomous mode enable"]
pub use BDMAAMEN_R as ADC3AMEN_R;
#[doc = "Field `DTSAMEN` reader - Digital temperature sensor Autonomous mode enable"]
pub use BDMAAMEN_R as DTSAMEN_R;
#[doc = "Field `BKPSRAMAMEN` reader - Backup RAM Autonomous mode enable"]
pub use BDMAAMEN_R as BKPSRAMAMEN_R;
#[doc = "Field `SRAM4AMEN` reader - SRAM4 Autonomous mode enable"]
pub use BDMAAMEN_R as SRAM4AMEN_R;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable"]
pub use BDMAAMEN_W as LPUART1AMEN_W;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable"]
pub use BDMAAMEN_W as SPI6AMEN_W;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable"]
pub use BDMAAMEN_W as I2C4AMEN_W;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable"]
pub use BDMAAMEN_W as LPTIM2AMEN_W;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable"]
pub use BDMAAMEN_W as LPTIM3AMEN_W;
#[doc = "Field `LPTIM4AMEN` writer - LPTIM4 Autonomous mode enable"]
pub use BDMAAMEN_W as LPTIM4AMEN_W;
#[doc = "Field `LPTIM5AMEN` writer - LPTIM5 Autonomous mode enable"]
pub use BDMAAMEN_W as LPTIM5AMEN_W;
#[doc = "Field `COMP12AMEN` writer - COMP12 Autonomous mode enable"]
pub use BDMAAMEN_W as COMP12AMEN_W;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable"]
pub use BDMAAMEN_W as VREFAMEN_W;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable"]
pub use BDMAAMEN_W as RTCAMEN_W;
#[doc = "Field `CRCAMEN` writer - CRC Autonomous mode enable"]
pub use BDMAAMEN_W as CRCAMEN_W;
#[doc = "Field `SAI4AMEN` writer - SAI4 Autonomous mode enable"]
pub use BDMAAMEN_W as SAI4AMEN_W;
#[doc = "Field `ADC3AMEN` writer - ADC3 Autonomous mode enable"]
pub use BDMAAMEN_W as ADC3AMEN_W;
#[doc = "Field `DTSAMEN` writer - Digital temperature sensor Autonomous mode enable"]
pub use BDMAAMEN_W as DTSAMEN_W;
#[doc = "Field `BKPSRAMAMEN` writer - Backup RAM Autonomous mode enable"]
pub use BDMAAMEN_W as BKPSRAMAMEN_W;
#[doc = "Field `SRAM4AMEN` writer - SRAM4 Autonomous mode enable"]
pub use BDMAAMEN_W as SRAM4AMEN_W;
impl R {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&self) -> BDMAAMEN_R {
        BDMAAMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&self) -> LPTIM5AMEN_R {
        LPTIM5AMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&self) -> CRCAMEN_R {
        CRCAMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&self) -> SAI4AMEN_R {
        SAI4AMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&self) -> ADC3AMEN_R {
        ADC3AMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable"]
    #[inline(always)]
    pub fn dtsamen(&self) -> DTSAMEN_R {
        DTSAMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpsramamen(&self) -> BKPSRAMAMEN_R {
        BKPSRAMAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdmaamen(&mut self) -> BDMAAMEN_W<0> {
        BDMAAMEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<3> {
        LPUART1AMEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W<5> {
        SPI6AMEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W<7> {
        I2C4AMEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W<9> {
        LPTIM2AMEN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<10> {
        LPTIM3AMEN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W<11> {
        LPTIM4AMEN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5amen(&mut self) -> LPTIM5AMEN_W<12> {
        LPTIM5AMEN_W::new(self)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W<14> {
        COMP12AMEN_W::new(self)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<15> {
        VREFAMEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcamen(&mut self) -> RTCAMEN_W<16> {
        RTCAMEN_W::new(self)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcamen(&mut self) -> CRCAMEN_W<19> {
        CRCAMEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai4amen(&mut self) -> SAI4AMEN_W<21> {
        SAI4AMEN_W::new(self)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3amen(&mut self) -> ADC3AMEN_W<24> {
        ADC3AMEN_W::new(self)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtsamen(&mut self) -> DTSAMEN_W<26> {
        DTSAMEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramamen(&mut self) -> BKPSRAMAMEN_W<28> {
        BKPSRAMAMEN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W<29> {
        SRAM4AMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC D3 Autonomous mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3amr](index.html) module"]
pub struct D3AMR_SPEC;
impl crate::RegisterSpec for D3AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3amr::R](R) reader structure"]
impl crate::Readable for D3AMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3amr::W](W) writer structure"]
impl crate::Writable for D3AMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3AMR to value 0"]
impl crate::Resettable for D3AMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
