#[doc = "Register `ETH_TX_SINGLE_COLLISION_GOOD_PACKETS` reader"]
pub struct R(crate::R<ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSNGLCOLG` reader - Tx Single Collision Good Packets This field indicates the number of successfully transmitted packets after a single collision in the Half-duplex mode."]
pub type TXSNGLCOLG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx Single Collision Good Packets This field indicates the number of successfully transmitted packets after a single collision in the Half-duplex mode."]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "Tx single collision good packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_tx_single_collision_good_packets](index.html) module"]
pub struct ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC;
impl crate::RegisterSpec for ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_tx_single_collision_good_packets::R](R) reader structure"]
impl crate::Readable for ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_TX_SINGLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for ETH_TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
