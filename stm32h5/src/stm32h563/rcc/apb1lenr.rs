#[doc = "Register `APB1LENR` reader"]
pub struct R(crate::R<APB1LENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LENR` writer"]
pub struct W(crate::W<APB1LENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LENR_SPEC>;
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
impl From<crate::W<APB1LENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM3EN` reader - TIM3 clock enable Set and reset by software."]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - TIM3 clock enable Set and reset by software."]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM4EN` reader - TIM4 clock enable Set and reset by software."]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - TIM4 clock enable Set and reset by software."]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM5EN` reader - TIM5 clock enable Set and reset by software."]
pub type TIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5EN` writer - TIM5 clock enable Set and reset by software."]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM6EN` reader - TIM6 clock enable Set and reset by software."]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - TIM6 clock enable Set and reset by software."]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM7EN` reader - TIM7 clock enable Set and reset by software."]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - TIM7 clock enable Set and reset by software."]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM12EN` reader - TIM12 clock enable Set and reset by software."]
pub type TIM12EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM12EN` writer - TIM12 clock enable Set and reset by software."]
pub type TIM12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM13EN` reader - TIM13 clock enable Set and reset by software."]
pub type TIM13EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM13EN` writer - TIM13 clock enable Set and reset by software."]
pub type TIM13EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `TIM14EN` reader - TIM14 clock enable Set and reset by software."]
pub type TIM14EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14EN` writer - TIM14 clock enable Set and reset by software."]
pub type TIM14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set and reset by software."]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set and reset by software."]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and reset by software."]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and reset by software."]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and reset by software."]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and reset by software."]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and reset by software."]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and reset by software."]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and reset by software."]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and reset by software."]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `UART4EN` reader - UART4 clock enable Set and reset by software."]
pub type UART4EN_R = crate::BitReader<bool>;
#[doc = "Field `UART4EN` writer - UART4 clock enable Set and reset by software."]
pub type UART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `UART5EN` reader - UART5 clock enable Set and reset by software."]
pub type UART5EN_R = crate::BitReader<bool>;
#[doc = "Field `UART5EN` writer - UART5 clock enable Set and reset by software."]
pub type UART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and reset by software."]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and reset by software."]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and reset by software."]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and reset by software."]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `I3C1EN` reader - I3C1 clock enable Set and reset by software."]
pub type I3C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I3C1EN` writer - I3C1 clock enable Set and reset by software."]
pub type I3C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `CRSEN` reader - CRS clock enable Set and reset by software."]
pub type CRSEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSEN` writer - CRS clock enable Set and reset by software."]
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `USART6EN` reader - USART6 clock enable Set and reset by software."]
pub type USART6EN_R = crate::BitReader<bool>;
#[doc = "Field `USART6EN` writer - USART6 clock enable Set and reset by software."]
pub type USART6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `USART10EN` reader - USART10 clock enable Set and reset by software."]
pub type USART10EN_R = crate::BitReader<bool>;
#[doc = "Field `USART10EN` writer - USART10 clock enable Set and reset by software."]
pub type USART10EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `USART11EN` reader - USART11 clock enable"]
pub type USART11EN_R = crate::BitReader<bool>;
#[doc = "Field `USART11EN` writer - USART11 clock enable"]
pub type USART11EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `CECEN` reader - HDMI-CEC clock enable Set and reset by software."]
pub type CECEN_R = crate::BitReader<bool>;
#[doc = "Field `CECEN` writer - HDMI-CEC clock enable Set and reset by software."]
pub type CECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `UART7EN` reader - UART7 clock enable Set and reset by software."]
pub type UART7EN_R = crate::BitReader<bool>;
#[doc = "Field `UART7EN` writer - UART7 clock enable Set and reset by software."]
pub type UART7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
#[doc = "Field `UART8EN` reader - UART8 clock enable Set and reset by software."]
pub type UART8EN_R = crate::BitReader<bool>;
#[doc = "Field `UART8EN` writer - UART8 clock enable Set and reset by software."]
pub type UART8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i3c1en(&self) -> I3C1EN_R {
        I3C1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART10 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart10en(&self) -> USART10EN_R {
        USART10EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USART11 clock enable"]
    #[inline(always)]
    pub fn usart11en(&self) -> USART11EN_R {
        USART11EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart8en(&self) -> UART8EN_R {
        UART8EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> TIM12EN_W<6> {
        TIM12EN_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> TIM13EN_W<7> {
        TIM13EN_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<8> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<19> {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<20> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1en(&mut self) -> I3C1EN_W<23> {
        I3C1EN_W::new(self)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<24> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 25 - USART6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<25> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 26 - USART10 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart10en(&mut self) -> USART10EN_W<26> {
        USART10EN_W::new(self)
    }
    #[doc = "Bit 27 - USART11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart11en(&mut self) -> USART11EN_W<27> {
        USART11EN_W::new(self)
    }
    #[doc = "Bit 28 - HDMI-CEC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<28> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 30 - UART7 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart7en(&mut self) -> UART7EN_W<30> {
        UART7EN_W::new(self)
    }
    #[doc = "Bit 31 - UART8 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart8en(&mut self) -> UART8EN_W<31> {
        UART8EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lenr](index.html) module"]
pub struct APB1LENR_SPEC;
impl crate::RegisterSpec for APB1LENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lenr::R](R) reader structure"]
impl crate::Readable for APB1LENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lenr::W](W) writer structure"]
impl crate::Writable for APB1LENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LENR to value 0"]
impl crate::Resettable for APB1LENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
