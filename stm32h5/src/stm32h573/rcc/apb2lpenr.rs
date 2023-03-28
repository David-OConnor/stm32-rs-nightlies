#[doc = "Register `APB2LPENR` reader"]
pub struct R(crate::R<APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2LPENR` writer"]
pub struct W(crate::W<APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2LPENR_SPEC>;
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
impl From<crate::W<APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 clock enable during sleep mode Set and reset by software."]
pub type TIM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1LPEN` writer - TIM1 clock enable during sleep mode Set and reset by software."]
pub type TIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode Set and reset by software."]
pub type SPI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode Set and reset by software."]
pub type SPI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM8LPEN` reader - TIM8 clock enable during sleep mode Set and reset by software."]
pub type TIM8LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8LPEN` writer - TIM8 clock enable during sleep mode Set and reset by software."]
pub type TIM8LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM15LPEN` reader - TIM15 clock enable during sleep mode Set and reset by software."]
pub type TIM15LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15LPEN` writer - TIM15 clock enable during sleep mode Set and reset by software."]
pub type TIM15LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM16LPEN` reader - TIM16 clock enable during sleep mode Set and reset by software."]
pub type TIM16LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16LPEN` writer - TIM16 clock enable during sleep mode Set and reset by software."]
pub type TIM16LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM17LPEN` reader - TIM17 clock enable during sleep mode Set and reset by software."]
pub type TIM17LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17LPEN` writer - TIM17 clock enable during sleep mode Set and reset by software."]
pub type TIM17LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SPI4LPEN` reader - SPI4 clock enable during sleep mode Set and reset by software."]
pub type SPI4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI4LPEN` writer - SPI4 clock enable during sleep mode Set and reset by software."]
pub type SPI4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SPI6LPEN` reader - SPI6 clock enable during sleep mode Set and reset by software."]
pub type SPI6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI6LPEN` writer - SPI6 clock enable during sleep mode Set and reset by software."]
pub type SPI6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SAI1LPEN` reader - SAI1 clock enable during sleep mode Set and reset by software."]
pub type SAI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1LPEN` writer - SAI1 clock enable during sleep mode Set and reset by software."]
pub type SAI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SAI2LPEN` reader - SAI2 clock enable during sleep mode Set and reset by software."]
pub type SAI2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2LPEN` writer - SAI2 clock enable during sleep mode Set and reset by software."]
pub type SAI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `USBFSLPEN` reader - USBFS clock enable during sleep mode Set and reset by software."]
pub type USBFSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `USBFSLPEN` writer - USBFS clock enable during sleep mode Set and reset by software."]
pub type USBFSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - TIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usbfslpen(&self) -> USBFSLPEN_R {
        USBFSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<11> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<13> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<14> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<16> {
        TIM15LPEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<17> {
        TIM16LPEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<18> {
        TIM17LPEN_W::new(self)
    }
    #[doc = "Bit 19 - SPI4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<19> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - SPI6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<20> {
        SPI6LPEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<21> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<22> {
        SAI2LPEN_W::new(self)
    }
    #[doc = "Bit 24 - USBFS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbfslpen(&mut self) -> USBFSLPEN_W<24> {
        USBFSLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB2 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](index.html) module"]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2lpenr::R](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x017f_7800"]
impl crate::Resettable for APB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x017f_7800;
}
