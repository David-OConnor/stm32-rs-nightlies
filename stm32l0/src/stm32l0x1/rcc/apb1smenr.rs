#[doc = "Register `APB1SMENR` reader"]
pub struct R(crate::R<APB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR` writer"]
pub struct W(crate::W<APB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR_SPEC>;
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
impl From<crate::W<APB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - Timer2 clock enable during sleep mode bit"]
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN_A>;
#[doc = "Timer2 clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::Disabled,
            true => TIM2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN_A::Enabled
    }
}
#[doc = "Field `TIM2SMEN` writer - Timer2 clock enable during sleep mode bit"]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR_SPEC, TIM2SMEN_A, O>;
impl<'a, const O: u8> TIM2SMEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::Enabled)
    }
}
#[doc = "Field `TIM3SMEN` reader - Timer 3 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as TIM3SMEN_R;
#[doc = "Field `TIM6SMEN` reader - Timer 6 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as TIM6SMEN_R;
#[doc = "Field `TIM7SMEN` reader - Timer 7 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as TIM7SMEN_R;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clock enable during sleep mode bit"]
pub use TIM2SMEN_R as WWDGSMEN_R;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as SPI2SMEN_R;
#[doc = "Field `USART2SMEN` reader - UART2 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as USART2SMEN_R;
#[doc = "Field `LPUART1SMEN` reader - LPUART1 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as LPUART1SMEN_R;
#[doc = "Field `USART4SMEN` reader - USART4 clock enabe during sleep mode bit"]
pub use TIM2SMEN_R as USART4SMEN_R;
#[doc = "Field `USART5SMEN` reader - USART5 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as USART5SMEN_R;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as I2C1SMEN_R;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as I2C2SMEN_R;
#[doc = "Field `CRSSMEN` reader - Clock recovery system clock enable during sleep mode bit"]
pub use TIM2SMEN_R as CRSSMEN_R;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during sleep mode bit"]
pub use TIM2SMEN_R as PWRSMEN_R;
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during sleep mode bit"]
pub use TIM2SMEN_R as I2C3SMEN_R;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer clock enable during sleep mode bit"]
pub use TIM2SMEN_R as LPTIM1SMEN_R;
#[doc = "Field `TIM3SMEN` writer - Timer 3 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as TIM3SMEN_W;
#[doc = "Field `TIM6SMEN` writer - Timer 6 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as TIM6SMEN_W;
#[doc = "Field `TIM7SMEN` writer - Timer 7 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as TIM7SMEN_W;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clock enable during sleep mode bit"]
pub use TIM2SMEN_W as WWDGSMEN_W;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as SPI2SMEN_W;
#[doc = "Field `USART2SMEN` writer - UART2 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as USART2SMEN_W;
#[doc = "Field `LPUART1SMEN` writer - LPUART1 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as LPUART1SMEN_W;
#[doc = "Field `USART4SMEN` writer - USART4 clock enabe during sleep mode bit"]
pub use TIM2SMEN_W as USART4SMEN_W;
#[doc = "Field `USART5SMEN` writer - USART5 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as USART5SMEN_W;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as I2C1SMEN_W;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as I2C2SMEN_W;
#[doc = "Field `CRSSMEN` writer - Clock recovery system clock enable during sleep mode bit"]
pub use TIM2SMEN_W as CRSSMEN_W;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during sleep mode bit"]
pub use TIM2SMEN_W as PWRSMEN_W;
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during sleep mode bit"]
pub use TIM2SMEN_W as I2C3SMEN_W;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer clock enable during sleep mode bit"]
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPUART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enabe during sleep mode bit"]
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Clock recovery system clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    #[doc = "Bit 17 - UART2 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - LPUART1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<18> {
        LPUART1SMEN_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enabe during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<19> {
        USART4SMEN_W::new(self)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart5smen(&mut self) -> USART5SMEN_W<20> {
        USART5SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    #[doc = "Bit 27 - Clock recovery system clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<27> {
        CRSSMEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<30> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr](index.html) module"]
pub struct APB1SMENR_SPEC;
impl crate::RegisterSpec for APB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr::R](R) reader structure"]
impl crate::Readable for APB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr::W](W) writer structure"]
impl crate::Writable for APB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1SMENR to value 0xb8e6_4a11"]
impl crate::Resettable for APB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb8e6_4a11;
}
