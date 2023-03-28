#[doc = "Register `SRDAMR` reader"]
pub struct R(crate::R<SRDAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDAMR` writer"]
pub struct W(crate::W<SRDAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDAMR_SPEC>;
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
impl From<crate::W<SRDAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDMA2AMEN` reader - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BDMA2AMEN_R = crate::BitReader<BDMA2AMEN_A>;
#[doc = "SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDMA2AMEN_A {
    #[doc = "0: Clock disabled in autonomous mode"]
    Disabled = 0,
    #[doc = "1: Clock enabled in autonomous mode"]
    Enabled = 1,
}
impl From<BDMA2AMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDMA2AMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BDMA2AMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDMA2AMEN_A {
        match self.bits {
            false => BDMA2AMEN_A::Disabled,
            true => BDMA2AMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDMA2AMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDMA2AMEN_A::Enabled
    }
}
#[doc = "Field `BDMA2AMEN` writer - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BDMA2AMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRDAMR_SPEC, BDMA2AMEN_A, O>;
impl<'a, const O: u8> BDMA2AMEN_W<'a, O> {
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDMA2AMEN_A::Disabled)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDMA2AMEN_A::Enabled)
    }
}
#[doc = "Field `GPIOAMEN` reader - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as GPIOAMEN_R;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as LPUART1AMEN_R;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as SPI6AMEN_R;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as I2C4AMEN_R;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub use BDMA2AMEN_R as LPTIM2AMEN_R;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as LPTIM3AMEN_R;
#[doc = "Field `DAC2AMEN` reader - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DAC2AMEN_R;
#[doc = "Field `COMP12AMEN` reader - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as COMP12AMEN_R;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as VREFAMEN_R;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as RTCAMEN_R;
#[doc = "Field `DTSAMEN` reader - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DTSAMEN_R;
#[doc = "Field `DFSDM2AMEN` reader - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DFSDM2AMEN_R;
#[doc = "Field `BKPRAMAMEN` reader - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as BKPRAMAMEN_R;
#[doc = "Field `SRDSRAMAMEN` reader - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as SRDSRAMAMEN_R;
#[doc = "Field `GPIOAMEN` writer - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as GPIOAMEN_W;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as LPUART1AMEN_W;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as SPI6AMEN_W;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as I2C4AMEN_W;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub use BDMA2AMEN_W as LPTIM2AMEN_W;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as LPTIM3AMEN_W;
#[doc = "Field `DAC2AMEN` writer - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DAC2AMEN_W;
#[doc = "Field `COMP12AMEN` writer - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as COMP12AMEN_W;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as VREFAMEN_W;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as RTCAMEN_W;
#[doc = "Field `DTSAMEN` writer - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DTSAMEN_W;
#[doc = "Field `DFSDM2AMEN` writer - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DFSDM2AMEN_W;
#[doc = "Field `BKPRAMAMEN` writer - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as BKPRAMAMEN_W;
#[doc = "Field `SRDSRAMAMEN` writer - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as SRDSRAMAMEN_W;
impl R {
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bdma2amen(&self) -> BDMA2AMEN_R {
        BDMA2AMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn gpioamen(&self) -> GPIOAMEN_R {
        GPIOAMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dac2amen(&self) -> DAC2AMEN_R {
        DAC2AMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dtsamen(&self) -> DTSAMEN_R {
        DTSAMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dfsdm2amen(&self) -> DFSDM2AMEN_R {
        DFSDM2AMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bkpramamen(&self) -> BKPRAMAMEN_R {
        BKPRAMAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn srdsramamen(&self) -> SRDSRAMAMEN_R {
        SRDSRAMAMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn bdma2amen(&mut self) -> BDMA2AMEN_W<0> {
        BDMA2AMEN_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn gpioamen(&mut self) -> GPIOAMEN_W<1> {
        GPIOAMEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<3> {
        LPUART1AMEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W<5> {
        SPI6AMEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W<7> {
        I2C4AMEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W<9> {
        LPTIM2AMEN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<10> {
        LPTIM3AMEN_W::new(self)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dac2amen(&mut self) -> DAC2AMEN_W<13> {
        DAC2AMEN_W::new(self)
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W<14> {
        COMP12AMEN_W::new(self)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<15> {
        VREFAMEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn rtcamen(&mut self) -> RTCAMEN_W<16> {
        RTCAMEN_W::new(self)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dtsamen(&mut self) -> DTSAMEN_W<26> {
        DTSAMEN_W::new(self)
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2amen(&mut self) -> DFSDM2AMEN_W<27> {
        DFSDM2AMEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn bkpramamen(&mut self) -> BKPRAMAMEN_W<28> {
        BKPRAMAMEN_W::new(self)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn srdsramamen(&mut self) -> SRDSRAMAMEN_W<29> {
        SRDSRAMAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC SmartRun domain Autonomous mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdamr](index.html) module"]
pub struct SRDAMR_SPEC;
impl crate::RegisterSpec for SRDAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdamr::R](R) reader structure"]
impl crate::Readable for SRDAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdamr::W](W) writer structure"]
impl crate::Writable for SRDAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRDAMR to value 0"]
impl crate::Resettable for SRDAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
