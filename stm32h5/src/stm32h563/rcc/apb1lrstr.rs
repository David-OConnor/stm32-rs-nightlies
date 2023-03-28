#[doc = "Register `APB1LRSTR` reader"]
pub struct R(crate::R<APB1LRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LRSTR` writer"]
pub struct W(crate::W<APB1LRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LRSTR_SPEC>;
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
impl From<crate::W<APB1LRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 block reset Set and reset by software."]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - TIM2 block reset Set and reset by software."]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM3RST` reader - TIM3 block reset Set and reset by software."]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - TIM3 block reset Set and reset by software."]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM4RST` reader - TIM4 block reset Set and reset by software."]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - TIM4 block reset Set and reset by software."]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM5RST` reader - TIM5 block reset Set and reset by software."]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - TIM5 block reset Set and reset by software."]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM6RST` reader - TIM6 block reset Set and reset by software."]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - TIM6 block reset Set and reset by software."]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM7RST` reader - TIM7 block reset Set and reset by software."]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - TIM7 block reset Set and reset by software."]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM12RST` reader - TIM12 block reset Set and reset by software."]
pub type TIM12RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM12RST` writer - TIM12 block reset Set and reset by software."]
pub type TIM12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM13RST` reader - TIM13 block reset t Set and reset by software."]
pub type TIM13RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM13RST` writer - TIM13 block reset t Set and reset by software."]
pub type TIM13RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `TIM14RST` reader - TIM14 block reset Set and reset by software."]
pub type TIM14RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM14RST` writer - TIM14 block reset Set and reset by software."]
pub type TIM14RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - SPI2 block reset Set and reset by software."]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2 block reset Set and reset by software."]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `SPI3RST` reader - SPI3 block reset Set and reset by software."]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - SPI3 block reset Set and reset by software."]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - USART2 block reset Set and reset by software."]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2 block reset Set and reset by software."]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - USART3 block reset Set and reset by software."]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART3 block reset Set and reset by software."]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `UART4RST` reader - UART4 block reset Set and reset by software."]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART4 block reset Set and reset by software."]
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `UART5RST` reader - UART5 block reset Set and reset by software."]
pub type UART5RST_R = crate::BitReader<bool>;
#[doc = "Field `UART5RST` writer - UART5 block reset Set and reset by software."]
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1 block reset Set and reset by software."]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1 block reset Set and reset by software."]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - I2C2 block reset Set and reset by software."]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C2 block reset Set and reset by software."]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `I3C1RST` reader - I3C1 block reset Set and reset by software."]
pub type I3C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I3C1RST` writer - I3C1 block reset Set and reset by software."]
pub type I3C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `CRSRST` reader - CRS block reset Set and reset by software."]
pub type CRSRST_R = crate::BitReader<bool>;
#[doc = "Field `CRSRST` writer - CRS block reset Set and reset by software."]
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `USART6RST` reader - USART6 block reset Set and reset by software."]
pub type USART6RST_R = crate::BitReader<bool>;
#[doc = "Field `USART6RST` writer - USART6 block reset Set and reset by software."]
pub type USART6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `USART10RST` reader - USART10 block reset Set and reset by software."]
pub type USART10RST_R = crate::BitReader<bool>;
#[doc = "Field `USART10RST` writer - USART10 block reset Set and reset by software."]
pub type USART10RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `USART11RST` reader - USART11 block reset Set and reset by software."]
pub type USART11RST_R = crate::BitReader<bool>;
#[doc = "Field `USART11RST` writer - USART11 block reset Set and reset by software."]
pub type USART11RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `CECRST` reader - HDMI-CEC block reset Set and reset by software."]
pub type CECRST_R = crate::BitReader<bool>;
#[doc = "Field `CECRST` writer - HDMI-CEC block reset Set and reset by software."]
pub type CECRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `UART7RST` reader - UART7 block reset Set and reset by software."]
pub type UART7RST_R = crate::BitReader<bool>;
#[doc = "Field `UART7RST` writer - UART7 block reset Set and reset by software."]
pub type UART7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
#[doc = "Field `UART8RST` reader - UART8 block reset Set and reset by software."]
pub type UART8RST_R = crate::BitReader<bool>;
#[doc = "Field `UART8RST` writer - UART8 block reset Set and reset by software."]
pub type UART8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 block reset t Set and reset by software."]
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i3c1rst(&self) -> I3C1RST_R {
        I3C1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART10 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart10rst(&self) -> USART10RST_R {
        USART10RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USART11 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart11rst(&self) -> USART11RST_R {
        USART11RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HDMI-CEC block reset Set and reset by software."]
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim12rst(&mut self) -> TIM12RST_W<6> {
        TIM12RST_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 block reset t Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim13rst(&mut self) -> TIM13RST_W<7> {
        TIM13RST_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> TIM14RST_W<8> {
        TIM14RST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - UART4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 20 - UART5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1rst(&mut self) -> I3C1RST_W<23> {
        I3C1RST_W::new(self)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<24> {
        CRSRST_W::new(self)
    }
    #[doc = "Bit 25 - USART6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<25> {
        USART6RST_W::new(self)
    }
    #[doc = "Bit 26 - USART10 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart10rst(&mut self) -> USART10RST_W<26> {
        USART10RST_W::new(self)
    }
    #[doc = "Bit 27 - USART11 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart11rst(&mut self) -> USART11RST_W<27> {
        USART11RST_W::new(self)
    }
    #[doc = "Bit 28 - HDMI-CEC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cecrst(&mut self) -> CECRST_W<28> {
        CECRST_W::new(self)
    }
    #[doc = "Bit 30 - UART7 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> UART7RST_W<30> {
        UART7RST_W::new(self)
    }
    #[doc = "Bit 31 - UART8 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart8rst(&mut self) -> UART8RST_W<31> {
        UART8RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 peripheral low reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lrstr](index.html) module"]
pub struct APB1LRSTR_SPEC;
impl crate::RegisterSpec for APB1LRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lrstr::R](R) reader structure"]
impl crate::Readable for APB1LRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lrstr::W](W) writer structure"]
impl crate::Writable for APB1LRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LRSTR to value 0"]
impl crate::Resettable for APB1LRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
