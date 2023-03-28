#[doc = "Register `RCC_MP_APB2LPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_APB2LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB2LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB2LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB2LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB2LPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_APB2LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB2LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_APB2LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB2LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1LPEN"]
pub type TIM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1LPEN` writer - TIM1LPEN"]
pub type TIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM8LPEN` reader - TIM8LPEN"]
pub type TIM8LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8LPEN` writer - TIM8LPEN"]
pub type TIM8LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM15LPEN` reader - TIM15LPEN"]
pub type TIM15LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15LPEN` writer - TIM15LPEN"]
pub type TIM15LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM16LPEN` reader - TIM16LPEN"]
pub type TIM16LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16LPEN` writer - TIM16LPEN"]
pub type TIM16LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM17LPEN` reader - TIM17LPEN"]
pub type TIM17LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17LPEN` writer - TIM17LPEN"]
pub type TIM17LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SPI1LPEN` reader - SPI1LPEN"]
pub type SPI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1LPEN` writer - SPI1LPEN"]
pub type SPI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SPI4LPEN` reader - SPI4LPEN"]
pub type SPI4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI4LPEN` writer - SPI4LPEN"]
pub type SPI4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SPI5LPEN` reader - SPI5LPEN"]
pub type SPI5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI5LPEN` writer - SPI5LPEN"]
pub type SPI5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `USART6LPEN` reader - USART6LPEN"]
pub type USART6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART6LPEN` writer - USART6LPEN"]
pub type USART6LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SAI1LPEN` reader - SAI1LPEN"]
pub type SAI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1LPEN` writer - SAI1LPEN"]
pub type SAI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SAI2LPEN` reader - SAI2LPEN"]
pub type SAI2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2LPEN` writer - SAI2LPEN"]
pub type SAI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SAI3LPEN` reader - SAI3LPEN"]
pub type SAI3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI3LPEN` writer - SAI3LPEN"]
pub type SAI3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DFSDMLPEN` reader - DFSDMLPEN"]
pub type DFSDMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMLPEN` writer - DFSDMLPEN"]
pub type DFSDMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `ADFSDMLPEN` reader - ADFSDMLPEN"]
pub type ADFSDMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `ADFSDMLPEN` writer - ADFSDMLPEN"]
pub type ADFSDMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `FDCANLPEN` reader - FDCANLPEN"]
pub type FDCANLPEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCANLPEN` writer - FDCANLPEN"]
pub type FDCANLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB2LPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&self) -> DFSDMLPEN_R {
        DFSDMLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&self) -> ADFSDMLPEN_R {
        ADFSDMLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<0> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<1> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<2> {
        TIM15LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<3> {
        TIM16LPEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<4> {
        TIM17LPEN_W::new(self)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<8> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<9> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<10> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<13> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<16> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<17> {
        SAI2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W<18> {
        SAI3LPEN_W::new(self)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmlpen(&mut self) -> DFSDMLPEN_W<20> {
        DFSDMLPEN_W::new(self)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmlpen(&mut self) -> ADFSDMLPEN_W<21> {
        ADFSDMLPEN_W::new(self)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<24> {
        FDCANLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2lpenclrr](index.html) module"]
pub struct RCC_MP_APB2LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB2LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb2lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB2LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB2LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB2LPENCLRR to value 0x0137_271f"]
impl crate::Resettable for RCC_MP_APB2LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0137_271f;
}
