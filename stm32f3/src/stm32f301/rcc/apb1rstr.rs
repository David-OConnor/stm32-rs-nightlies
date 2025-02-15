#[doc = "Register `APB1RSTR` reader"]
pub struct R(crate::R<APB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR` writer"]
pub struct W(crate::W<APB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR_SPEC>;
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
impl From<crate::W<APB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - Timer 2 reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
#[doc = "Timer 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM2RST_A> {
        match self.bits {
            true => Some(TIM2RST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST_A::Reset
    }
}
#[doc = "Field `TIM2RST` writer - Timer 2 reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, TIM2RST_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::Reset)
    }
}
#[doc = "Field `TIM6RST` reader - Timer 6 reset"]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub use TIM2RST_R as WWDGRST_R;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use TIM2RST_R as SPI2RST_R;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub use TIM2RST_R as SPI3RST_R;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub use TIM2RST_R as USART3RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM2RST_R as PWRRST_R;
#[doc = "Field `DAC1RST` reader - DAC interface reset"]
pub use TIM2RST_R as DAC1RST_R;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub use TIM2RST_R as I2C3RST_R;
#[doc = "Field `TIM6RST` writer - Timer 6 reset"]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub use TIM2RST_W as WWDGRST_W;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use TIM2RST_W as SPI2RST_W;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub use TIM2RST_W as SPI3RST_W;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub use TIM2RST_W as USART3RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM2RST_W as PWRRST_W;
#[doc = "Field `DAC1RST` writer - DAC interface reset"]
pub use TIM2RST_W as DAC1RST_W;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub use TIM2RST_W as I2C3RST_W;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<11> {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<29> {
        DAC1RST_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<30> {
        I2C3RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](index.html) module"]
pub struct APB1RSTR_SPEC;
impl crate::RegisterSpec for APB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr::R](R) reader structure"]
impl crate::Readable for APB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](W) writer structure"]
impl crate::Writable for APB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
