#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - GPIOA clock enable Set and reset by software."]
pub type GPIOAEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOAEN` writer - GPIOA clock enable Set and reset by software."]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOBEN` reader - GPIOB clock enable Set and reset by software."]
pub type GPIOBEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBEN` writer - GPIOB clock enable Set and reset by software."]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOCEN` reader - GPIOC clock enable Set and reset by software."]
pub type GPIOCEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCEN` writer - GPIOC clock enable Set and reset by software."]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIODEN` reader - GPIOD clock enable Set and reset by software."]
pub type GPIODEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODEN` writer - GPIOD clock enable Set and reset by software."]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOHEN` reader - GPIOH clock enable Set and reset by software."]
pub type GPIOHEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHEN` writer - GPIOH clock enable Set and reset by software."]
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC1EN` reader - ADC1 peripherals clock enabled Set and reset by software."]
pub type ADC1EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1EN` writer - ADC1 peripherals clock enabled Set and reset by software."]
pub type ADC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `DAC12EN` reader - DAC clock enable Set and reset by software."]
pub type DAC12EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC12EN` writer - DAC clock enable Set and reset by software."]
pub type DAC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `HASHEN` reader - HASH clock enable Set and reset by software."]
pub type HASHEN_R = crate::BitReader<bool>;
#[doc = "Field `HASHEN` writer - HASH clock enable Set and reset by software."]
pub type HASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `RNGEN` reader - RNG clock enable Set and reset by software."]
pub type RNGEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGEN` writer - RNG clock enable Set and reset by software."]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software."]
pub type SRAM2EN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software."]
pub type SRAM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<10> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac12en(&mut self) -> DAC12EN_W<11> {
        DAC12EN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<30> {
        SRAM2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0xc000_0000"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
