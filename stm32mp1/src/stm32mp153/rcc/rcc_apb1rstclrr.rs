#[doc = "Register `RCC_APB1RSTCLRR` reader"]
pub struct R(crate::R<RCC_APB1RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1RSTCLRR` writer"]
pub struct W(crate::W<RCC_APB1RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_APB1RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - TIM2RST"]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - TIM2RST"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM3RST` reader - TIM3RST"]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - TIM3RST"]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM4RST` reader - TIM4RST"]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - TIM4RST"]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM5RST` reader - TIM5RST"]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - TIM5RST"]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM6RST` reader - TIM6RST"]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - TIM6RST"]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM7RST` reader - TIM7RST"]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - TIM7RST"]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM12RST` reader - TIM12RST"]
pub type TIM12RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM12RST` writer - TIM12RST"]
pub type TIM12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM13RST` reader - TIM13RST"]
pub type TIM13RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM13RST` writer - TIM13RST"]
pub type TIM13RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM14RST` reader - TIM14RST"]
pub type TIM14RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM14RST` writer - TIM14RST"]
pub type TIM14RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM1RST` reader - LPTIM1RST"]
pub type LPTIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1RST` writer - LPTIM1RST"]
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - SPI2RST"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2RST"]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPI3RST` reader - SPI3RST"]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - SPI3RST"]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - USART2RST"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2RST"]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - USART3RST"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART3RST"]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `UART4RST` reader - UART4RST"]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART4RST"]
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `UART5RST` reader - UART5RST"]
pub type UART5RST_R = crate::BitReader<bool>;
#[doc = "Field `UART5RST` writer - UART5RST"]
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `UART7RST` reader - UART7RST"]
pub type UART7RST_R = crate::BitReader<bool>;
#[doc = "Field `UART7RST` writer - UART7RST"]
pub type UART7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `UART8RST` reader - UART8RST"]
pub type UART8RST_R = crate::BitReader<bool>;
#[doc = "Field `UART8RST` writer - UART8RST"]
pub type UART8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1RST"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1RST"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - I2C2RST"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C2RST"]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `I2C3RST` reader - I2C3RST"]
pub type I2C3RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C3RST` writer - I2C3RST"]
pub type I2C3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `I2C5RST` reader - I2C5RST"]
pub type I2C5RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C5RST` writer - I2C5RST"]
pub type I2C5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPDIFRST` reader - SPDIFRST"]
pub type SPDIFRST_R = crate::BitReader<bool>;
#[doc = "Field `SPDIFRST` writer - SPDIFRST"]
pub type SPDIFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `CECRST` reader - CECRST"]
pub type CECRST_R = crate::BitReader<bool>;
#[doc = "Field `CECRST` writer - CECRST"]
pub type CECRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `DAC12RST` reader - DAC12RST"]
pub type DAC12RST_R = crate::BitReader<bool>;
#[doc = "Field `DAC12RST` writer - DAC12RST"]
pub type DAC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
#[doc = "Field `MDIOSRST` reader - MDIOSRST"]
pub type MDIOSRST_R = crate::BitReader<bool>;
#[doc = "Field `MDIOSRST` writer - MDIOSRST"]
pub type MDIOSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB1RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2RST"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3RST"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4RST"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5RST"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6RST"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7RST"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12RST"]
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13RST"]
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14RST"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1RST"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI2RST"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI3RST"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART2RST"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USART3RST"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - UART4RST"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART5RST"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART7RST"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART8RST"]
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2RST"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3RST"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C5RST"]
    #[inline(always)]
    pub fn i2c5rst(&self) -> I2C5RST_R {
        I2C5RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - SPDIFRST"]
    #[inline(always)]
    pub fn spdifrst(&self) -> SPDIFRST_R {
        SPDIFRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CECRST"]
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC12RST"]
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - MDIOSRST"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM4RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 6 - TIM12RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim12rst(&mut self) -> TIM12RST_W<6> {
        TIM12RST_W::new(self)
    }
    #[doc = "Bit 7 - TIM13RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim13rst(&mut self) -> TIM13RST_W<7> {
        TIM13RST_W::new(self)
    }
    #[doc = "Bit 8 - TIM14RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> TIM14RST_W<8> {
        TIM14RST_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<9> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Bit 11 - SPI2RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<11> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI3RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<12> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 14 - USART2RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<14> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 15 - USART3RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<15> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 16 - UART4RST"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<16> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 17 - UART5RST"]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<17> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 18 - UART7RST"]
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> UART7RST_W<18> {
        UART7RST_W::new(self)
    }
    #[doc = "Bit 19 - UART8RST"]
    #[inline(always)]
    #[must_use]
    pub fn uart8rst(&mut self) -> UART8RST_W<19> {
        UART8RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - I2C3RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<23> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 24 - I2C5RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5rst(&mut self) -> I2C5RST_W<24> {
        I2C5RST_W::new(self)
    }
    #[doc = "Bit 26 - SPDIFRST"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrst(&mut self) -> SPDIFRST_W<26> {
        SPDIFRST_W::new(self)
    }
    #[doc = "Bit 27 - CECRST"]
    #[inline(always)]
    #[must_use]
    pub fn cecrst(&mut self) -> CECRST_W<27> {
        CECRST_W::new(self)
    }
    #[doc = "Bit 29 - DAC12RST"]
    #[inline(always)]
    #[must_use]
    pub fn dac12rst(&mut self) -> DAC12RST_W<29> {
        DAC12RST_W::new(self)
    }
    #[doc = "Bit 31 - MDIOSRST"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<31> {
        MDIOSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1rstclrr](index.html) module"]
pub struct RCC_APB1RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_APB1RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_APB1RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_APB1RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1RSTCLRR to value 0"]
impl crate::Resettable for RCC_APB1RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
