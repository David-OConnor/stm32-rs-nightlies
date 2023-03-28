#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and reset by software."]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and reset by software."]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM8EN` reader - TIM8 clock enable Set and reset by software."]
pub type TIM8EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8EN` writer - TIM8 clock enable Set and reset by software."]
pub type TIM8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `USART1EN` reader - USART1 clock enable Set and reset by software."]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - USART1 clock enable Set and reset by software."]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM15EN` reader - TIM15 clock enable Set and reset by software."]
pub type TIM15EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15EN` writer - TIM15 clock enable Set and reset by software."]
pub type TIM15EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM16EN` reader - TIM16 clock enable Set and reset by software."]
pub type TIM16EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16EN` writer - TIM16 clock enable Set and reset by software."]
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM17EN` reader - TIM17 clock enable Set and reset by software."]
pub type TIM17EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17EN` writer - TIM17 clock enable Set and reset by software."]
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable Set and reset by software."]
pub type SPI4EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable Set and reset by software."]
pub type SPI4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI6EN` reader - SPI6 clock enable Set and reset by software."]
pub type SPI6EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI6EN` writer - SPI6 clock enable Set and reset by software."]
pub type SPI6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable Set and reset by software."]
pub type SAI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1EN` writer - SAI1 clock enable Set and reset by software."]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SAI2EN` reader - SAI2 clock enable Set and cleared by software."]
pub type SAI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable Set and cleared by software."]
pub type SAI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `USBFSEN` reader - USBFS clock enable Set and reset by software."]
pub type USBFSEN_R = crate::BitReader<bool>;
#[doc = "Field `USBFSEN` writer - USBFS clock enable Set and reset by software."]
pub type USBFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usbfsen(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<16> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 19 - SPI4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<19> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 20 - SPI6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<20> {
        SPI6EN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<22> {
        SAI2EN_W::new(self)
    }
    #[doc = "Bit 24 - USBFS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbfsen(&mut self) -> USBFSEN_W<24> {
        USBFSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB2 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
