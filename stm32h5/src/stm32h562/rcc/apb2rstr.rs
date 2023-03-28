#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 block reset Set and reset by software."]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1 block reset Set and reset by software."]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - SPI1 block reset Set and reset by software."]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1 block reset Set and reset by software."]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM8RST` reader - TIM8 block reset Set and reset by software."]
pub type TIM8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM8RST` writer - TIM8 block reset Set and reset by software."]
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `USART1RST` reader - USART1 block reset Set and reset by software."]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1 block reset Set and reset by software."]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM15RST` reader - TIM15 block reset Set and reset by software."]
pub type TIM15RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM15RST` writer - TIM15 block reset Set and reset by software."]
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM16RST` reader - TIM16 block reset Set and reset by software."]
pub type TIM16RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM16RST` writer - TIM16 block reset Set and reset by software."]
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM17RST` reader - TIM17 block reset Set and reset by software."]
pub type TIM17RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM17RST` writer - TIM17 block reset Set and reset by software."]
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SPI4RST` reader - SPI4 block reset Set and reset by software."]
pub type SPI4RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI4RST` writer - SPI4 block reset Set and reset by software."]
pub type SPI4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SPI6RST` reader - SPI6 block reset Set and reset by software."]
pub type SPI6RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI6RST` writer - SPI6 block reset Set and reset by software."]
pub type SPI6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SAI1RST` reader - SAI1 block reset Set and reset by software."]
pub type SAI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI1RST` writer - SAI1 block reset Set and reset by software."]
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SAI2RST` reader - SAI2 block reset Set and reset by software."]
pub type SAI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI2RST` writer - SAI2 block reset Set and reset by software."]
pub type SAI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `USBFSRST` reader - USBFS block reset Set and reset by software."]
pub type USBFSRST_R = crate::BitReader<bool>;
#[doc = "Field `USBFSRST` writer - USBFS block reset Set and reset by software."]
pub type USBFSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS block reset Set and reset by software."]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 19 - SPI4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<19> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 20 - SPI6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi6rst(&mut self) -> SPI6RST_W<20> {
        SPI6RST_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<21> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<22> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 24 - USBFS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> USBFSRST_W<24> {
        USBFSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
