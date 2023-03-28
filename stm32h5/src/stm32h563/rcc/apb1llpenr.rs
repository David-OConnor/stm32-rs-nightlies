#[doc = "Register `APB1LLPENR` reader"]
pub struct R(crate::R<APB1LLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LLPENR` writer"]
pub struct W(crate::W<APB1LLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LLPENR_SPEC>;
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
impl From<crate::W<APB1LLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during sleep mode Set and reset by software."]
pub type TIM3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during sleep mode Set and reset by software."]
pub type TIM3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during sleep mode Set and reset by software."]
pub type TIM4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during sleep mode Set and reset by software."]
pub type TIM4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during sleep mode Set and reset by software."]
pub type TIM5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during sleep mode Set and reset by software."]
pub type TIM5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM12LPEN` reader - TIM12 clock enable during sleep mode Set and reset by software."]
pub type TIM12LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM12LPEN` writer - TIM12 clock enable during sleep mode Set and reset by software."]
pub type TIM12LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM13LPEN` reader - TIM13 clock enable during sleep mode Set and reset by software."]
pub type TIM13LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM13LPEN` writer - TIM13 clock enable during sleep mode Set and reset by software."]
pub type TIM13LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM14LPEN` reader - TIM14 clock enable during sleep mode Set and reset by software."]
pub type TIM14LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14LPEN` writer - TIM14 clock enable during sleep mode Set and reset by software."]
pub type TIM14LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `UART4LPEN` reader - UART4 clock enable during sleep mode Set and reset by software."]
pub type UART4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART4LPEN` writer - UART4 clock enable during sleep mode Set and reset by software."]
pub type UART4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `UART5LPEN` reader - UART5 clock enable during sleep mode Set and reset by software."]
pub type UART5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART5LPEN` writer - UART5 clock enable during sleep mode Set and reset by software."]
pub type UART5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during sleep mode Set and reset by software."]
pub type I2C1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during sleep mode Set and reset by software."]
pub type I2C1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during sleep mode Set and reset by software."]
pub type I2C2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during sleep mode Set and reset by software."]
pub type I2C2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `I3C1LPEN` reader - I3C1 clock enable during sleep mode Set and reset by software."]
pub type I3C1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I3C1LPEN` writer - I3C1 clock enable during sleep mode Set and reset by software."]
pub type I3C1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `CRSLPEN` reader - CRS clock enable during sleep mode Set and reset by software."]
pub type CRSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSLPEN` writer - CRS clock enable during sleep mode Set and reset by software."]
pub type CRSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode Set and reset by software."]
pub type USART6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode Set and reset by software."]
pub type USART6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART10LPEN` reader - USART10 clock enable during sleep mode Set and reset by software."]
pub type USART10LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART10LPEN` writer - USART10 clock enable during sleep mode Set and reset by software."]
pub type USART10LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART11LPEN` reader - USART11 clock enable during sleep mode Set and reset by software."]
pub type USART11LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART11LPEN` writer - USART11 clock enable during sleep mode Set and reset by software."]
pub type USART11LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `CECLPEN` reader - HDMI-CEC clock enable during sleep mode Set and reset by software."]
pub type CECLPEN_R = crate::BitReader<bool>;
#[doc = "Field `CECLPEN` writer - HDMI-CEC clock enable during sleep mode Set and reset by software."]
pub type CECLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `UART7LPEN` reader - UART7 clock enable during sleep mode Set and reset by software."]
pub type UART7LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART7LPEN` writer - UART7 clock enable during sleep mode Set and reset by software."]
pub type UART7LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `UART8LPEN` reader - UART8 clock enable during sleep mode Set and reset by software."]
pub type UART8LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART8LPEN` writer - UART8 clock enable during sleep mode Set and reset by software."]
pub type UART8LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i3c1lpen(&self) -> I3C1LPEN_R {
        I3C1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART10 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USART11 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart11lpen(&self) -> USART11LPEN_R {
        USART11LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<5> {
        TIM7LPEN_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<6> {
        TIM12LPEN_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<7> {
        TIM13LPEN_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<8> {
        TIM14LPEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<18> {
        USART3LPEN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<19> {
        UART4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<20> {
        UART5LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1lpen(&mut self) -> I3C1LPEN_W<23> {
        I3C1LPEN_W::new(self)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crslpen(&mut self) -> CRSLPEN_W<24> {
        CRSLPEN_W::new(self)
    }
    #[doc = "Bit 25 - USART6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<25> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 26 - USART10 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W<26> {
        USART10LPEN_W::new(self)
    }
    #[doc = "Bit 27 - USART11 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart11lpen(&mut self) -> USART11LPEN_W<27> {
        USART11LPEN_W::new(self)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ceclpen(&mut self) -> CECLPEN_W<28> {
        CECLPEN_W::new(self)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<30> {
        UART7LPEN_W::new(self)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<31> {
        UART8LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1llpenr](index.html) module"]
pub struct APB1LLPENR_SPEC;
impl crate::RegisterSpec for APB1LLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1llpenr::R](R) reader structure"]
impl crate::Readable for APB1LLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1llpenr::W](W) writer structure"]
impl crate::Writable for APB1LLPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LLPENR to value 0xdffe_c9ff"]
impl crate::Resettable for APB1LLPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xdffe_c9ff;
}
