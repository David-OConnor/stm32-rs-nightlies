#[doc = "Register `APB2SMENR` reader"]
pub struct R(crate::R<APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2SMENR` writer"]
pub struct W(crate::W<APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2SMENR_SPEC>;
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
impl From<crate::W<APB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSMEN` reader - ADC clocks enable during CPU1 Csleep and CStop modes"]
pub type ADCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADCSMEN` writer - ADC clocks enable during CPU1 Csleep and CStop modes"]
pub type ADCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during CPU1 CSleep mode."]
pub type TIM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during CPU1 CSleep mode."]
pub type TIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during CPU1 CSleep mode."]
pub type SPI1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during CPU1 CSleep mode."]
pub type SPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during CPU1 Csleep and CStop modes."]
pub type USART1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during CPU1 Csleep and CStop modes."]
pub type USART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during CPU1 CSleep mode."]
pub type TIM16SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during CPU1 CSleep mode."]
pub type TIM16SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clock enable during CPU1 CSleep mode."]
pub type TIM17SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clock enable during CPU1 CSleep mode."]
pub type TIM17SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - ADC clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - ADC clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<9> {
        ADCSMEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<11> {
        TIM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<17> {
        TIM16SMEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<18> {
        TIM17SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clocks enable in Sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](index.html) module"]
pub struct APB2SMENR_SPEC;
impl crate::RegisterSpec for APB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2smenr::R](R) reader structure"]
impl crate::Readable for APB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](W) writer structure"]
impl crate::Writable for APB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0006_5a00"]
impl crate::Resettable for APB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_5a00;
}
