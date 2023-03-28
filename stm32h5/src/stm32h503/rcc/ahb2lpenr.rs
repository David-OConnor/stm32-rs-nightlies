#[doc = "Register `AHB2LPENR` reader"]
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2LPENR` writer"]
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOALPEN` writer - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOBLPEN` reader - GPIOB clock enable during sleep mode Set and reset by software."]
pub type GPIOBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBLPEN` writer - GPIOB clock enable during sleep mode Set and reset by software."]
pub type GPIOBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOCLPEN` reader - GPIOC clock enable during sleep mode Set and reset by software."]
pub type GPIOCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCLPEN` writer - GPIOC clock enable during sleep mode Set and reset by software."]
pub type GPIOCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `GPIODLPEN` reader - GPIOD clock enable during sleep mode Set and reset by software."]
pub type GPIODLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODLPEN` writer - GPIOD clock enable during sleep mode Set and reset by software."]
pub type GPIODLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOHLPEN` reader - GPIOH clock enable during sleep mode Set and reset by software."]
pub type GPIOHLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHLPEN` writer - GPIOH clock enable during sleep mode Set and reset by software."]
pub type GPIOHLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `ADC1LPEN` reader - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
pub type ADC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1LPEN` writer - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
pub type ADC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `DAC12LPEN` reader - DAC clock enable during sleep mode Set and reset by software."]
pub type DAC12LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC12LPEN` writer - DAC clock enable during sleep mode Set and reset by software."]
pub type DAC12LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `HASHLPEN` reader - HASH clock enable during sleep mode Set and reset by software."]
pub type HASHLPEN_R = crate::BitReader<bool>;
#[doc = "Field `HASHLPEN` writer - HASH clock enable during sleep mode Set and reset by software."]
pub type HASHLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `RNGLPEN` reader - RNG clock enable during sleep mode Set and reset by software."]
pub type RNGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGLPEN` writer - RNG clock enable during sleep mode Set and reset by software."]
pub type RNGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 clock enable during sleep mode Set and reset by software."]
pub type SRAM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 clock enable during sleep mode Set and reset by software."]
pub type SRAM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<10> {
        ADC1LPEN_W::new(self)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W<11> {
        DAC12LPEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<17> {
        HASHLPEN_W::new(self)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<18> {
        RNGLPEN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<30> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](index.html) module"]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2lpenr::R](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xc01f_1dff"]
impl crate::Resettable for AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc01f_1dff;
}
