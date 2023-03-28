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
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - Timer 2 reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM3RST` reader - Timer 3 reset"]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - Timer 3 reset"]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM4RST` reader - Timer 4 reset"]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - Timer 4 reset"]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM5RST` reader - Timer 5 reset"]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - Timer 5 reset"]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM6RST` reader - Timer 6reset"]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - Timer 6reset"]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM7RST` reader - Timer 7 reset"]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - Timer 7 reset"]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `LCDRST` reader - LCD reset"]
pub type LCDRST_R = crate::BitReader<bool>;
#[doc = "Field `LCDRST` writer - LCD reset"]
pub type LCDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `WWDRST` reader - Window watchdog reset"]
pub type WWDRST_R = crate::BitReader<bool>;
#[doc = "Field `WWDRST` writer - Window watchdog reset"]
pub type WWDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI3RST` reader - SPI 3 reset"]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - SPI 3 reset"]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - USART 3 reset"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART 3 reset"]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART4RST` reader - UART 4 reset"]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART 4 reset"]
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART5RST` reader - UART 5 reset"]
pub type UART5RST_R = crate::BitReader<bool>;
#[doc = "Field `UART5RST` writer - UART 5 reset"]
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USB reset"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `DACRST` reader - DAC interface reset"]
pub type DACRST_R = crate::BitReader<bool>;
#[doc = "Field `DACRST` writer - DAC interface reset"]
pub type DACRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `COMPRST` reader - COMP interface reset"]
pub type COMPRST_R = crate::BitReader<bool>;
#[doc = "Field `COMPRST` writer - COMP interface reset"]
pub type COMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD reset"]
    #[inline(always)]
    pub fn lcdrst(&self) -> LCDRST_R {
        LCDRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdrst(&self) -> WWDRST_R {
        WWDRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 31 - COMP interface reset"]
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6reset"]
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
    #[doc = "Bit 9 - LCD reset"]
    #[inline(always)]
    #[must_use]
    pub fn lcdrst(&mut self) -> LCDRST_W<9> {
        LCDRST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdrst(&mut self) -> WWDRST_W<11> {
        WWDRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
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
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
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
    #[doc = "Bit 31 - COMP interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<31> {
        COMPRST_W::new(self)
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
