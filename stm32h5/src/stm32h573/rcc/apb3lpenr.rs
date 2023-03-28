#[doc = "Register `APB3LPENR` reader"]
pub struct R(crate::R<APB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3LPENR` writer"]
pub struct W(crate::W<APB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3LPENR_SPEC>;
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
impl From<crate::W<APB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSLPEN` reader - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SBSLPEN` writer - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `SPI5LPEN` reader - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
pub type SPI5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI5LPEN` writer - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
pub type SPI5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during sleep mode Set and reset by software."]
pub type I2C3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during sleep mode Set and reset by software."]
pub type I2C3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `I2C4LPEN` reader - I2C4 clock enable during sleep mode Set and reset by software."]
pub type I2C4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4LPEN` writer - I2C4 clock enable during sleep mode Set and reset by software."]
pub type I2C4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 clock enable during sleep mode Set and reset by software."]
pub type LPTIM3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 clock enable during sleep mode Set and reset by software."]
pub type LPTIM3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 clock enable during sleep mode Set and reset by software."]
pub type LPTIM4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 clock enable during sleep mode Set and reset by software."]
pub type LPTIM4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 clock enable during sleep mode Set and reset by software."]
pub type LPTIM5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 clock enable during sleep mode Set and reset by software."]
pub type LPTIM5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM6LPEN` reader - LPTIM6 clock enable during sleep mode Set and reset by software."]
pub type LPTIM6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6LPEN` writer - LPTIM6 clock enable during sleep mode Set and reset by software."]
pub type LPTIM6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `VREFLPEN` reader - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFLPEN` writer - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sbslpen(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim6lpen(&self) -> LPTIM6LPEN_R {
        LPTIM6LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbslpen(&mut self) -> SBSLPEN_W<1> {
        SBSLPEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<5> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<6> {
        LPUART1LPEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<7> {
        I2C3LPEN_W::new(self)
    }
    #[doc = "Bit 8 - I2C4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<8> {
        I2C4LPEN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<11> {
        LPTIM1LPEN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<12> {
        LPTIM3LPEN_W::new(self)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<13> {
        LPTIM4LPEN_W::new(self)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<14> {
        LPTIM5LPEN_W::new(self)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6lpen(&mut self) -> LPTIM6LPEN_W<15> {
        LPTIM6LPEN_W::new(self)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<20> {
        VREFLPEN_W::new(self)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<21> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3lpenr](index.html) module"]
pub struct APB3LPENR_SPEC;
impl crate::RegisterSpec for APB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3lpenr::R](R) reader structure"]
impl crate::Readable for APB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3lpenr::W](W) writer structure"]
impl crate::Writable for APB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3LPENR to value 0x0030_f9e2"]
impl crate::Resettable for APB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_f9e2;
}
