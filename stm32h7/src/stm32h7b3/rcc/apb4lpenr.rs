#[doc = "Register `APB4LPENR` reader"]
pub struct R(crate::R<APB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB4LPENR` writer"]
pub struct W(crate::W<APB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4LPENR_SPEC>;
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
impl From<crate::W<APB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode Set and reset by software."]
pub type SYSCFGLPEN_R = crate::BitReader<SYSCFGLPEN_A>;
#[doc = "SYSCFG peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<SYSCFGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGLPEN_A {
        match self.bits {
            false => SYSCFGLPEN_A::Disabled,
            true => SYSCFGLPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGLPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGLPEN_A::Enabled
    }
}
#[doc = "Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode Set and reset by software."]
pub type SYSCFGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4LPENR_SPEC, SYSCFGLPEN_A, O>;
impl<'a, const O: u8> SYSCFGLPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::Enabled)
    }
}
#[doc = "Field `LPUART1LPEN` reader - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_R as LPUART1LPEN_R;
#[doc = "Field `SPI6LPEN` reader - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_R as SPI6LPEN_R;
#[doc = "Field `I2C4LPEN` reader - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_R as I2C4LPEN_R;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_R as LPTIM2LPEN_R;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_R as LPTIM3LPEN_R;
#[doc = "Field `DAC2LPEN` reader - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as DAC2LPEN_R;
#[doc = "Field `COMP12LPEN` reader - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as COMP12LPEN_R;
#[doc = "Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as VREFLPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as RTCAPBLPEN_R;
#[doc = "Field `DTSLPEN` reader - temperature sensor peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as DTSLPEN_R;
#[doc = "Field `DFSDM2LPEN` reader - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_R as DFSDM2LPEN_R;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_W as LPUART1LPEN_W;
#[doc = "Field `SPI6LPEN` writer - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_W as SPI6LPEN_W;
#[doc = "Field `I2C4LPEN` writer - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_W as I2C4LPEN_W;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_W as LPTIM2LPEN_W;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
pub use SYSCFGLPEN_W as LPTIM3LPEN_W;
#[doc = "Field `DAC2LPEN` writer - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as DAC2LPEN_W;
#[doc = "Field `COMP12LPEN` writer - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as COMP12LPEN_W;
#[doc = "Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as VREFLPEN_W;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as RTCAPBLPEN_W;
#[doc = "Field `DTSLPEN` writer - temperature sensor peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as DTSLPEN_W;
#[doc = "Field `DFSDM2LPEN` writer - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software."]
pub use SYSCFGLPEN_W as DFSDM2LPEN_W;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dac2lpen(&self) -> DAC2LPEN_R {
        DAC2LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 26 - temperature sensor peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dfsdm2lpen(&self) -> DFSDM2LPEN_R {
        DFSDM2LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<1> {
        SYSCFGLPEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<3> {
        LPUART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<5> {
        SPI6LPEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<7> {
        I2C4LPEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<9> {
        LPTIM2LPEN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<10> {
        LPTIM3LPEN_W::new(self)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac2lpen(&mut self) -> DAC2LPEN_W<13> {
        DAC2LPEN_W::new(self)
    }
    #[doc = "Bit 14 - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<14> {
        COMP12LPEN_W::new(self)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<15> {
        VREFLPEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC APB clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<16> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Bit 26 - temperature sensor peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<26> {
        DTSLPEN_W::new(self)
    }
    #[doc = "Bit 27 - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2lpen(&mut self) -> DFSDM2LPEN_W<27> {
        DFSDM2LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4lpenr](index.html) module"]
pub struct APB4LPENR_SPEC;
impl crate::RegisterSpec for APB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb4lpenr::R](R) reader structure"]
impl crate::Readable for APB4LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb4lpenr::W](W) writer structure"]
impl crate::Writable for APB4LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB4LPENR to value 0x0c01_e6aa"]
impl crate::Resettable for APB4LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c01_e6aa;
}
