#[doc = "Register `RXCRC` reader"]
pub struct R(crate::R<RXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRC` reader - CRC register for receiver When CRC calculation is enabled, the RXCRC\\[31:0\\]
bits contain the computed CRC value of the subsequently received bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. Not used for the I2S mode. RXCRC\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case."]
pub type RXCRC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver When CRC calculation is enabled, the RXCRC\\[31:0\\]
bits contain the computed CRC value of the subsequently received bytes. CRC calculation is initialized when the CRCEN bit of SPI_CR1 is written to 1 or when a data block is transacted completely. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPOLY register. The number of bits considered at calculation depends on SPI_CRCPOLY register and CRCSIZE bits settings at SPI_CFG1 register. Note: a read to this register when the communication is ongoing could return an incorrect value. Not used for the I2S mode. RXCRC\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored. Note: The configuration of CRCSIZE bit field is not taken into account when the content of this register is read by software. No masking is applied for unused bits at this case."]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
#[doc = "SPI/I2S receiver CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](index.html) module"]
pub struct RXCRC_SPEC;
impl crate::RegisterSpec for RXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrc::R](R) reader structure"]
impl crate::Readable for RXCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
