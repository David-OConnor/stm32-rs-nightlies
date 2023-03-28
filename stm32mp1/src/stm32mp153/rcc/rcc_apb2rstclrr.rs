#[doc = "Register `RCC_APB2RSTCLRR` reader"]
pub struct R(crate::R<RCC_APB2RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2RSTCLRR` writer"]
pub struct W(crate::W<RCC_APB2RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_APB2RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1RST"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1RST"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM8RST` reader - TIM8RST"]
pub type TIM8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM8RST` writer - TIM8RST"]
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM15RST` reader - TIM15RST"]
pub type TIM15RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM15RST` writer - TIM15RST"]
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM16RST` reader - TIM16RST"]
pub type TIM16RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM16RST` writer - TIM16RST"]
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `TIM17RST` reader - TIM17RST"]
pub type TIM17RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM17RST` writer - TIM17RST"]
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPI4RST` reader - SPI4RST"]
pub type SPI4RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI4RST` writer - SPI4RST"]
pub type SPI4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SPI5RST` reader - SPI5RST"]
pub type SPI5RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI5RST` writer - SPI5RST"]
pub type SPI5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `USART6RST` reader - USART6RST"]
pub type USART6RST_R = crate::BitReader<bool>;
#[doc = "Field `USART6RST` writer - USART6RST"]
pub type USART6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SAI1RST` reader - SAI1RST"]
pub type SAI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI1RST` writer - SAI1RST"]
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SAI2RST` reader - SAI2RST"]
pub type SAI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI2RST` writer - SAI2RST"]
pub type SAI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SAI3RST` reader - SAI3RST"]
pub type SAI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI3RST` writer - SAI3RST"]
pub type SAI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `DFSDMRST` reader - DFSDMRST"]
pub type DFSDMRST_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMRST` writer - DFSDMRST"]
pub type DFSDMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
#[doc = "Field `FDCANRST` reader - FDCANRST"]
pub type FDCANRST_R = crate::BitReader<bool>;
#[doc = "Field `FDCANRST` writer - FDCANRST"]
pub type FDCANRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB2RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<0> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<1> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<2> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<3> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<4> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<8> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<9> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<10> {
        SPI5RST_W::new(self)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<13> {
        USART6RST_W::new(self)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<16> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<17> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai3rst(&mut self) -> SAI3RST_W<18> {
        SAI3RST_W::new(self)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W<20> {
        DFSDMRST_W::new(self)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<24> {
        FDCANRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2rstclrr](index.html) module"]
pub struct RCC_APB2RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_APB2RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_APB2RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_APB2RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB2RSTCLRR to value 0"]
impl crate::Resettable for RCC_APB2RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
