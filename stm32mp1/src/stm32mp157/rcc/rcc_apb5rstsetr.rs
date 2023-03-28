#[doc = "Register `RCC_APB5RSTSETR` reader"]
pub struct R(crate::R<RCC_APB5RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB5RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB5RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB5RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB5RSTSETR` writer"]
pub struct W(crate::W<RCC_APB5RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB5RSTSETR_SPEC>;
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
impl From<crate::W<RCC_APB5RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB5RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI6RST` reader - SPI6RST"]
pub type SPI6RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI6RST` writer - SPI6RST"]
pub type SPI6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB5RSTSETR_SPEC, bool, O>;
#[doc = "Field `I2C4RST` reader - I2C4RST"]
pub type I2C4RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C4RST` writer - I2C4RST"]
pub type I2C4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB5RSTSETR_SPEC, bool, O>;
#[doc = "Field `I2C6RST` reader - I2C6RST"]
pub type I2C6RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C6RST` writer - I2C6RST"]
pub type I2C6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB5RSTSETR_SPEC, bool, O>;
#[doc = "Field `USART1RST` reader - USART1RST"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1RST"]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB5RSTSETR_SPEC, bool, O>;
#[doc = "Field `STGENRST` reader - STGENRST"]
pub type STGENRST_R = crate::BitReader<bool>;
#[doc = "Field `STGENRST` writer - STGENRST"]
pub type STGENRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB5RSTSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi6rst(&mut self) -> SPI6RST_W<0> {
        SPI6RST_W::new(self)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<2> {
        I2C4RST_W::new(self)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6rst(&mut self) -> I2C6RST_W<3> {
        I2C6RST_W::new(self)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<4> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrst(&mut self) -> STGENRST_W<20> {
        STGENRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb5rstsetr](index.html) module"]
pub struct RCC_APB5RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_APB5RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb5rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_APB5RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb5rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_APB5RSTSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB5RSTSETR to value 0"]
impl crate::Resettable for RCC_APB5RSTSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
