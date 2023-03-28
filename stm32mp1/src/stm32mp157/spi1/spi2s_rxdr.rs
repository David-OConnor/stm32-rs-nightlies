#[doc = "Register `SPI2S_RXDR` reader"]
pub struct R(crate::R<SPI2S_RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_RXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDR` reader - RXDR"]
pub type RXDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
#[doc = "SPI/I2S receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_rxdr](index.html) module"]
pub struct SPI2S_RXDR_SPEC;
impl crate::RegisterSpec for SPI2S_RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_rxdr::R](R) reader structure"]
impl crate::Readable for SPI2S_RXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI2S_RXDR to value 0"]
impl crate::Resettable for SPI2S_RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
