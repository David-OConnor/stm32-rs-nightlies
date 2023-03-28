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
#[doc = "Field `TIM2RST` reader - Timer2 reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RSTW_A>;
#[doc = "Timer2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RSTW_A {
    #[doc = "1: Reset the module"]
    Reset = 1,
}
impl From<TIM2RSTW_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM2RSTW_A> {
        match self.bits {
            true => Some(TIM2RSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RSTW_A::Reset
    }
}
#[doc = "Field `TIM2RST` writer - Timer2 reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, TIM2RSTW_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW_A::Reset)
    }
}
#[doc = "Field `TIM3RST` reader - Timer3 reset"]
pub use TIM2RST_R as TIM3RST_R;
#[doc = "Field `TIM6RST` reader - Timer 6 reset"]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `TIM7RST` reader - Timer 7 reset"]
pub use TIM2RST_R as TIM7RST_R;
#[doc = "Field `WWDRST` reader - Window watchdog reset"]
pub use TIM2RST_R as WWDRST_R;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use TIM2RST_R as SPI2RST_R;
#[doc = "Field `LPUART12RST` reader - UART2 reset"]
pub use TIM2RST_R as LPUART12RST_R;
#[doc = "Field `LPUART1RST` reader - LPUART1 reset"]
pub use TIM2RST_R as LPUART1RST_R;
#[doc = "Field `USART4RST` reader - USART4 reset"]
pub use TIM2RST_R as USART4RST_R;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub use TIM2RST_R as USART5RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `USBRST` reader - USB reset"]
pub use TIM2RST_R as USBRST_R;
#[doc = "Field `CRSRST` reader - Clock recovery system reset"]
pub use TIM2RST_R as CRSRST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM2RST_R as PWRRST_R;
#[doc = "Field `DACRST` reader - DAC interface reset"]
pub use TIM2RST_R as DACRST_R;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub use TIM2RST_R as I2C3RST_R;
#[doc = "Field `LPTIM1RST` reader - Low power timer reset"]
pub use TIM2RST_R as LPTIM1RST_R;
#[doc = "Field `TIM3RST` writer - Timer3 reset"]
pub use TIM2RST_W as TIM3RST_W;
#[doc = "Field `TIM6RST` writer - Timer 6 reset"]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `TIM7RST` writer - Timer 7 reset"]
pub use TIM2RST_W as TIM7RST_W;
#[doc = "Field `WWDRST` writer - Window watchdog reset"]
pub use TIM2RST_W as WWDRST_W;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use TIM2RST_W as SPI2RST_W;
#[doc = "Field `LPUART12RST` writer - UART2 reset"]
pub use TIM2RST_W as LPUART12RST_W;
#[doc = "Field `LPUART1RST` writer - LPUART1 reset"]
pub use TIM2RST_W as LPUART1RST_W;
#[doc = "Field `USART4RST` writer - USART4 reset"]
pub use TIM2RST_W as USART4RST_W;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub use TIM2RST_W as USART5RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `USBRST` writer - USB reset"]
pub use TIM2RST_W as USBRST_W;
#[doc = "Field `CRSRST` writer - Clock recovery system reset"]
pub use TIM2RST_W as CRSRST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM2RST_W as PWRRST_W;
#[doc = "Field `DACRST` writer - DAC interface reset"]
pub use TIM2RST_W as DACRST_W;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub use TIM2RST_W as I2C3RST_W;
#[doc = "Field `LPTIM1RST` writer - Low power timer reset"]
pub use TIM2RST_W as LPTIM1RST_W;
impl R {
    #[doc = "Bit 0 - Timer2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdrst(&self) -> WWDRST_R {
        WWDRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn lpuart12rst(&self) -> LPUART12RST_R {
        LPUART12RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPUART1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Clock recovery system reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - Timer3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdrst(&mut self) -> WWDRST_W<11> {
        WWDRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart12rst(&mut self) -> LPUART12RST_W<17> {
        LPUART12RST_W::new(self)
    }
    #[doc = "Bit 18 - LPUART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<18> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4rst(&mut self) -> USART4RST_W<19> {
        USART4RST_W::new(self)
    }
    #[doc = "Bit 20 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> USART5RST_W<20> {
        USART5RST_W::new(self)
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
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<23> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 27 - Clock recovery system reset"]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<27> {
        CRSRST_W::new(self)
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
    pub fn dacrst(&mut self) -> DACRST_W<29> {
        DACRST_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<30> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<31> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](index.html) module"]
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
