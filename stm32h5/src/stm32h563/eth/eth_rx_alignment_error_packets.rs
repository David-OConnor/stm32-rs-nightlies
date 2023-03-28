#[doc = "Register `ETH_RX_ALIGNMENT_ERROR_PACKETS` reader"]
pub struct R(crate::R<ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXALGNERR` reader - Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error. It is valid only in 10/100 mode."]
pub type RXALGNERR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error. It is valid only in 10/100 mode."]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RXALGNERR_R {
        RXALGNERR_R::new(self.bits)
    }
}
#[doc = "Rx alignment error packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_rx_alignment_error_packets](index.html) module"]
pub struct ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC;
impl crate::RegisterSpec for ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_rx_alignment_error_packets::R](R) reader structure"]
impl crate::Readable for ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_RX_ALIGNMENT_ERROR_PACKETS to value 0"]
impl crate::Resettable for ETH_RX_ALIGNMENT_ERROR_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
