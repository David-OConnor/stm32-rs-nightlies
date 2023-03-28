#[doc = "Register `APB3ENR` reader"]
pub struct R(crate::R<APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3ENR` writer"]
pub struct W(crate::W<APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3ENR_SPEC>;
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
impl From<crate::W<APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSEN` reader - SBS clock enable Set and reset by software."]
pub type SBSEN_R = crate::BitReader<bool>;
#[doc = "Field `SBSEN` writer - SBS clock enable Set and reset by software."]
pub type SBSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `SPI5EN` reader - SPI5 clock enable Set and reset by software."]
pub type SPI5EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable Set and reset by software."]
pub type SPI5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable Set and reset by software."]
pub type I2C3EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable Set and reset by software."]
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `I2C4EN` reader - I2C4 clock enable Set and reset by software."]
pub type I2C4EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable Set and reset by software."]
pub type I2C4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 clock enable Set and reset by software."]
pub type LPTIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 clock enable Set and reset by software."]
pub type LPTIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 clock enable Set and reset by software."]
pub type LPTIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 clock enable Set and reset by software."]
pub type LPTIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 clock enable Set and reset by software."]
pub type LPTIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 clock enable Set and reset by software."]
pub type LPTIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM6EN` reader - LPTIM6 clock enable Set and reset by software."]
pub type LPTIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6EN` writer - LPTIM6 clock enable Set and reset by software."]
pub type LPTIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `VREFEN` reader - VREF clock enable Set and reset by software."]
pub type VREFEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFEN` writer - VREF clock enable Set and reset by software."]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim6en(&self) -> LPTIM6EN_R {
        LPTIM6EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsen(&mut self) -> SBSEN_W<1> {
        SBSEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<5> {
        SPI5EN_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<6> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 7 - I2C3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<7> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 8 - I2C4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<8> {
        I2C4EN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<11> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<12> {
        LPTIM3EN_W::new(self)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<13> {
        LPTIM4EN_W::new(self)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<14> {
        LPTIM5EN_W::new(self)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6en(&mut self) -> LPTIM6EN_W<15> {
        LPTIM6EN_W::new(self)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<20> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<21> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3enr](index.html) module"]
pub struct APB3ENR_SPEC;
impl crate::RegisterSpec for APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3enr::R](R) reader structure"]
impl crate::Readable for APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3enr::W](W) writer structure"]
impl crate::Writable for APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
