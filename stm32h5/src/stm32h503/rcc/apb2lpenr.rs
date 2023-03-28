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
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
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
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<14> {
        USART1LPEN_W::new(self)
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
