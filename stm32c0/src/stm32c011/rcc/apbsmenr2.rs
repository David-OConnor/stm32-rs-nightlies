#[doc = "Register `APBSMENR2` reader"]
pub struct R(crate::R<APBSMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBSMENR2` writer"]
pub struct W(crate::W<APBSMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR2_SPEC>;
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
impl From<crate::W<APBSMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software."]
pub type SYSCFGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software."]
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode Set and cleared by software."]
pub type SPI1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode Set and cleared by software."]
pub type SPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type USART1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type USART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM14SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM14SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM16SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM16SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM17SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type TIM17SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode Set and cleared by software."]
pub type ADCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode Set and cleared by software."]
pub type ADCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBSMENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim14smen(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<0> {
        SYSCFGSMEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<11> {
        TIM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W<15> {
        TIM14SMEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<17> {
        TIM16SMEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<18> {
        TIM17SMEN_W::new(self)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<20> {
        ADCSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB peripheral clock enable in Sleep/Stop mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr2](index.html) module"]
pub struct APBSMENR2_SPEC;
impl crate::RegisterSpec for APBSMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbsmenr2::R](R) reader structure"]
impl crate::Readable for APBSMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbsmenr2::W](W) writer structure"]
impl crate::Writable for APBSMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBSMENR2 to value 0x0017_d801"]
impl crate::Resettable for APBSMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0017_d801;
}
