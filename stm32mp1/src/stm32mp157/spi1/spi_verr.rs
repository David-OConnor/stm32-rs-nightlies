#[doc = "Register `SPI_VERR` reader"]
pub struct R(crate::R<SPI_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_VERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MINREV` reader - MINREV"]
pub type MINREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJREV` reader - MAJREV"]
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - MINREV"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MAJREV"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SPI/I2S version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_verr](index.html) module"]
pub struct SPI_VERR_SPEC;
impl crate::RegisterSpec for SPI_VERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_verr::R](R) reader structure"]
impl crate::Readable for SPI_VERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_VERR to value 0x11"]
impl crate::Resettable for SPI_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
