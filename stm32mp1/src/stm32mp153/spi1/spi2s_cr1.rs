#[doc = "Register `SPI2S_CR1` reader"]
pub struct R(crate::R<SPI2S_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2S_CR1` writer"]
pub struct W(crate::W<SPI2S_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_CR1_SPEC>;
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
impl From<crate::W<SPI2S_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPE` reader - SPE"]
pub type SPE_R = crate::BitReader<bool>;
#[doc = "Field `SPE` writer - SPE"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `MASRX` reader - MASRX"]
pub type MASRX_R = crate::BitReader<bool>;
#[doc = "Field `MASRX` writer - MASRX"]
pub type MASRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `CSTART` reader - CSTART"]
pub type CSTART_R = crate::BitReader<bool>;
#[doc = "Field `CSTART` writer - CSTART"]
pub type CSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `CSUSP` writer - CSUSP"]
pub type CSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `HDDIR` reader - HDDIR"]
pub type HDDIR_R = crate::BitReader<bool>;
#[doc = "Field `HDDIR` writer - HDDIR"]
pub type HDDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `SSI` reader - SSI"]
pub type SSI_R = crate::BitReader<bool>;
#[doc = "Field `SSI` writer - SSI"]
pub type SSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `CRC33_17` reader - CRC33_17"]
pub type CRC33_17_R = crate::BitReader<bool>;
#[doc = "Field `CRC33_17` writer - CRC33_17"]
pub type CRC33_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `RCRCINI` reader - RCRCINI"]
pub type RCRCINI_R = crate::BitReader<bool>;
#[doc = "Field `RCRCINI` writer - RCRCINI"]
pub type RCRCINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `TCRCINI` reader - TCRCINI"]
pub type TCRCINI_R = crate::BitReader<bool>;
#[doc = "Field `TCRCINI` writer - TCRCINI"]
pub type TCRCINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
#[doc = "Field `IOLOCK` reader - IOLOCK"]
pub type IOLOCK_R = crate::BitReader<bool>;
#[doc = "Field `IOLOCK` writer - IOLOCK"]
pub type IOLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<0> {
        SPE_W::new(self)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<8> {
        MASRX_W::new(self)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<9> {
        CSTART_W::new(self)
    }
    #[doc = "Bit 10 - CSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<10> {
        CSUSP_W::new(self)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<11> {
        HDDIR_W::new(self)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<12> {
        SSI_W::new(self)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<13> {
        CRC33_17_W::new(self)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    #[must_use]
    pub fn rcrcini(&mut self) -> RCRCINI_W<14> {
        RCRCINI_W::new(self)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    #[must_use]
    pub fn tcrcini(&mut self) -> TCRCINI_W<15> {
        TCRCINI_W::new(self)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn iolock(&mut self) -> IOLOCK_W<16> {
        IOLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_cr1](index.html) module"]
pub struct SPI2S_CR1_SPEC;
impl crate::RegisterSpec for SPI2S_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_cr1::R](R) reader structure"]
impl crate::Readable for SPI2S_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2s_cr1::W](W) writer structure"]
impl crate::Writable for SPI2S_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI2S_CR1 to value 0"]
impl crate::Resettable for SPI2S_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
