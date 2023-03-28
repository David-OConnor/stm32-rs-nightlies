#[doc = "Register `ETH_RX_CRC_ERROR_PACKETS` reader"]
pub struct R(crate::R<ETH_RX_CRC_ERROR_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_RX_CRC_ERROR_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_RX_CRC_ERROR_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_RX_CRC_ERROR_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRCERR` reader - Rx CRC Error Packets This field indicates the number of packets received with CRC error."]
pub type RXCRCERR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx CRC Error Packets This field indicates the number of packets received with CRC error."]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new(self.bits)
    }
}
#[doc = "Rx CRC error packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_rx_crc_error_packets](index.html) module"]
pub struct ETH_RX_CRC_ERROR_PACKETS_SPEC;
impl crate::RegisterSpec for ETH_RX_CRC_ERROR_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_rx_crc_error_packets::R](R) reader structure"]
impl crate::Readable for ETH_RX_CRC_ERROR_PACKETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_RX_CRC_ERROR_PACKETS to value 0"]
impl crate::Resettable for ETH_RX_CRC_ERROR_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
