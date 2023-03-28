#[doc = "Register `APB1ENR` reader"]
pub struct R(crate::R<APB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR` writer"]
pub struct W(crate::W<APB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR_SPEC>;
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
impl From<crate::W<APB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - Timer 2 clock enable"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - Timer 2 clock enable"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM3EN` reader - Timer 3 clock enable"]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - Timer 3 clock enable"]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM4EN` reader - Timer 4 clock enable"]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - Timer 4 clock enable"]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM5EN` reader - Timer 5 clock enable"]
pub type TIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5EN` writer - Timer 5 clock enable"]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable"]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable"]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable"]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable"]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `LCDEN` reader - LCD clock enable"]
pub type LCDEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDEN` writer - LCD clock enable"]
pub type LCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - SPI 2 clock enable"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - SPI 2 clock enable"]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `SPI3EN` reader - SPI 3 clock enable"]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - SPI 3 clock enable"]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART3EN` reader - USART 3 clock enable"]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - USART 3 clock enable"]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART4EN` reader - UART 4 clock enable"]
pub type USART4EN_R = crate::BitReader<bool>;
#[doc = "Field `USART4EN` writer - UART 4 clock enable"]
pub type USART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART5EN` reader - UART 5 clock enable"]
pub type USART5EN_R = crate::BitReader<bool>;
#[doc = "Field `USART5EN` writer - UART 5 clock enable"]
pub type USART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - I2C 1 clock enable"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C 1 clock enable"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `I2C2EN` reader - I2C 2 clock enable"]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - I2C 2 clock enable"]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - USB clock enable"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USB clock enable"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub type DACEN_R = crate::BitReader<bool>;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `COMPEN` reader - COMP interface clock enable"]
pub type COMPEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPEN` writer - COMP interface clock enable"]
pub type COMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP interface clock enable"]
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 9 - LCD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<9> {
        LCDEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<19> {
        USART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<20> {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<23> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<29> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 31 - COMP interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<31> {
        COMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](index.html) module"]
pub struct APB1ENR_SPEC;
impl crate::RegisterSpec for APB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr::R](R) reader structure"]
impl crate::Readable for APB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](W) writer structure"]
impl crate::Writable for APB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for APB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
