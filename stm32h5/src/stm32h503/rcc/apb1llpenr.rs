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
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `OPAMPLPEN` reader - OPAMP clock enable during sleep mode Set and reset by software."]
pub type OPAMPLPEN_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPLPEN` writer - OPAMP clock enable during sleep mode Set and reset by software."]
pub type OPAMPLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `COMPLPEN` reader - COMP clock enable during sleep mode Set and reset by software."]
pub type COMPLPEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPLPEN` writer - COMP clock enable during sleep mode Set and reset by software."]
pub type COMPLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LLPENR_SPEC, bool, O>;
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
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<13> {
        OPAMPLPEN_W::new(self)
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
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn complpen(&mut self) -> COMPLPEN_W<16> {
        COMPLPEN_W::new(self)
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
