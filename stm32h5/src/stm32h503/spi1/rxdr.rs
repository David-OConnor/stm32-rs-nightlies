#[doc = "Register `RXDR` reader"]
pub struct R(crate::R<RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDR` reader - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden."]
pub type RXDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - receive data register The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are read as zero when the register is read. Writing to the register is ignored. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by single access halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be read by single access word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be read by single access. Read access of this register less than the configured data size is forbidden."]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
#[doc = "SPI/I2S receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](index.html) module"]
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdr::R](R) reader structure"]
impl crate::Readable for RXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
