#[doc = "Register `ETH_TX_PACKET_COUNT_GOOD` reader"]
pub struct R(crate::R<ETH_TX_PACKET_COUNT_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_TX_PACKET_COUNT_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_TX_PACKET_COUNT_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_TX_PACKET_COUNT_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXPKTG` reader - Tx Packet Count Good This field indicates the number of good packets transmitted."]
pub type TXPKTG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx Packet Count Good This field indicates the number of good packets transmitted."]
    #[inline(always)]
    pub fn txpktg(&self) -> TXPKTG_R {
        TXPKTG_R::new(self.bits)
    }
}
#[doc = "Tx packet count good register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_tx_packet_count_good](index.html) module"]
pub struct ETH_TX_PACKET_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for ETH_TX_PACKET_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_tx_packet_count_good::R](R) reader structure"]
impl crate::Readable for ETH_TX_PACKET_COUNT_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_TX_PACKET_COUNT_GOOD to value 0"]
impl crate::Resettable for ETH_TX_PACKET_COUNT_GOOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
