#[doc = "Register `APB1LPENR` reader"]
pub struct R(crate::R<APB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LPENR` writer"]
pub struct W(crate::W<APB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LPENR_SPEC>;
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
impl From<crate::W<APB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2LPEN` reader - Timer 2 clock enable during Sleep mode"]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN_A>;
#[doc = "Timer 2 clock enable during Sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2LPEN_A {
        match self.bits {
            false => TIM2LPEN_A::Disabled,
            true => TIM2LPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN_A::Enabled
    }
}
#[doc = "Field `TIM2LPEN` writer - Timer 2 clock enable during Sleep mode"]
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, TIM2LPEN_A, O>;
impl<'a, const O: u8> TIM2LPEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::Enabled)
    }
}
#[doc = "Field `TIM3LPEN` reader - Timer 3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM3LPEN_R;
#[doc = "Field `TIM4LPEN` reader - Timer 4 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM4LPEN_R;
#[doc = "Field `TIM5LPEN` reader - Timer 5 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM5LPEN_R;
#[doc = "Field `TIM6LPEN` reader - Timer 6 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM6LPEN_R;
#[doc = "Field `TIM7LPEN` reader - Timer 7 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM7LPEN_R;
#[doc = "Field `LCDLPEN` reader - LCD clock enable during Sleep mode"]
pub use TIM2LPEN_R as LCDLPEN_R;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub use TIM2LPEN_R as WWDGLPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI 2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as SPI2LPEN_R;
#[doc = "Field `SPI3LPEN` reader - SPI 3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as SPI3LPEN_R;
#[doc = "Field `USART2LPEN` reader - USART 2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as USART2LPEN_R;
#[doc = "Field `USART3LPEN` reader - USART 3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as USART3LPEN_R;
#[doc = "Field `UART4LPEN` reader - USART 4 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART4LPEN_R;
#[doc = "Field `UART5LPEN` reader - USART 5 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART5LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C 1 clock enable during Sleep mode"]
pub use TIM2LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C 2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as I2C2LPEN_R;
#[doc = "Field `USBLPEN` reader - USB clock enable during Sleep mode"]
pub use TIM2LPEN_R as USBLPEN_R;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub use TIM2LPEN_R as PWRLPEN_R;
#[doc = "Field `DACLPEN` reader - DAC interface clock enable during Sleep mode"]
pub use TIM2LPEN_R as DACLPEN_R;
#[doc = "Field `COMPLPEN` reader - COMP interface clock enable during Sleep mode"]
pub use TIM2LPEN_R as COMPLPEN_R;
#[doc = "Field `TIM3LPEN` writer - Timer 3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM3LPEN_W;
#[doc = "Field `TIM4LPEN` writer - Timer 4 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM4LPEN_W;
#[doc = "Field `TIM5LPEN` writer - Timer 5 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM5LPEN_W;
#[doc = "Field `TIM6LPEN` writer - Timer 6 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM6LPEN_W;
#[doc = "Field `TIM7LPEN` writer - Timer 7 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM7LPEN_W;
#[doc = "Field `LCDLPEN` writer - LCD clock enable during Sleep mode"]
pub use TIM2LPEN_W as LCDLPEN_W;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub use TIM2LPEN_W as WWDGLPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI 2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as SPI2LPEN_W;
#[doc = "Field `SPI3LPEN` writer - SPI 3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as SPI3LPEN_W;
#[doc = "Field `USART2LPEN` writer - USART 2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as USART2LPEN_W;
#[doc = "Field `USART3LPEN` writer - USART 3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as USART3LPEN_W;
#[doc = "Field `UART4LPEN` writer - USART 4 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART4LPEN_W;
#[doc = "Field `UART5LPEN` writer - USART 5 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART5LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C 1 clock enable during Sleep mode"]
pub use TIM2LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C 2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as I2C2LPEN_W;
#[doc = "Field `USBLPEN` writer - USB clock enable during Sleep mode"]
pub use TIM2LPEN_W as USBLPEN_W;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub use TIM2LPEN_W as PWRLPEN_W;
#[doc = "Field `DACLPEN` writer - DAC interface clock enable during Sleep mode"]
pub use TIM2LPEN_W as DACLPEN_W;
#[doc = "Field `COMPLPEN` writer - COMP interface clock enable during Sleep mode"]
pub use TIM2LPEN_W as COMPLPEN_W;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lcdlpen(&self) -> LCDLPEN_R {
        LCDLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART 5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usblpen(&self) -> USBLPEN_R {
        USBLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<5> {
        TIM7LPEN_W::new(self)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcdlpen(&mut self) -> LCDLPEN_W<9> {
        LCDLPEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI 3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<18> {
        USART3LPEN_W::new(self)
    }
    #[doc = "Bit 19 - USART 4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<19> {
        UART4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - USART 5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<20> {
        UART5LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usblpen(&mut self) -> USBLPEN_W<23> {
        USBLPEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<28> {
        PWRLPEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn daclpen(&mut self) -> DACLPEN_W<29> {
        DACLPEN_W::new(self)
    }
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn complpen(&mut self) -> COMPLPEN_W<31> {
        COMPLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](index.html) module"]
pub struct APB1LPENR_SPEC;
impl crate::RegisterSpec for APB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lpenr::R](R) reader structure"]
impl crate::Readable for APB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](W) writer structure"]
impl crate::Writable for APB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0"]
impl crate::Resettable for APB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
