#[doc = "Register `APB3RSTR` reader"]
pub struct R(crate::R<APB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3RSTR` writer"]
pub struct W(crate::W<APB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3RSTR_SPEC>;
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
impl From<crate::W<APB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSRST` reader - SBS block reset Set and reset by software."]
pub type SBSRST_R = crate::BitReader<bool>;
#[doc = "Field `SBSRST` writer - SBS block reset Set and reset by software."]
pub type SBSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `SPI5RST` reader - SPI5 block reset Set and reset by software."]
pub type SPI5RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI5RST` writer - SPI5 block reset Set and reset by software."]
pub type SPI5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `I2C3RST` reader - I2C3 block reset Set and reset by software."]
pub type I2C3RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C3RST` writer - I2C3 block reset Set and reset by software."]
pub type I2C3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `I2C4RST` reader - I2C4 block reset Set and reset by software."]
pub type I2C4RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C4RST` writer - I2C4 block reset Set and reset by software."]
pub type I2C4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset Set and reset by software."]
pub type LPTIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset Set and reset by software."]
pub type LPTIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 block reset Set and reset by software."]
pub type LPTIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 block reset Set and reset by software."]
pub type LPTIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM5RST` reader - LPTIM5 block reset Set and reset by software."]
pub type LPTIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5RST` writer - LPTIM5 block reset Set and reset by software."]
pub type LPTIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM6RST` reader - LPTIM6 block reset Set and reset by software."]
pub type LPTIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6RST` writer - LPTIM6 block reset Set and reset by software."]
pub type LPTIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub type VREFRST_R = crate::BitReader<bool>;
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub type VREFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim6rst(&self) -> LPTIM6RST_R {
        LPTIM6RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsrst(&mut self) -> SBSRST_W<1> {
        SBSRST_W::new(self)
    }
    #[doc = "Bit 5 - SPI5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<5> {
        SPI5RST_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<6> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 7 - I2C3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<7> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 8 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<8> {
        I2C4RST_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<11> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<12> {
        LPTIM3RST_W::new(self)
    }
    #[doc = "Bit 13 - LPTIM4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<13> {
        LPTIM4RST_W::new(self)
    }
    #[doc = "Bit 14 - LPTIM5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<14> {
        LPTIM5RST_W::new(self)
    }
    #[doc = "Bit 15 - LPTIM6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6rst(&mut self) -> LPTIM6RST_W<15> {
        LPTIM6RST_W::new(self)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<20> {
        VREFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](index.html) module"]
pub struct APB3RSTR_SPEC;
impl crate::RegisterSpec for APB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3rstr::R](R) reader structure"]
impl crate::Readable for APB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](W) writer structure"]
impl crate::Writable for APB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
