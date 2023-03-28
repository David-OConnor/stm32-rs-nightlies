#[doc = "Register `TX_SINGLE_COLLISION_GOOD_PACKETS` reader"]
pub struct R(crate::R<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSNGLCOLG` reader - TXSNGLCOLG"]
pub type TXSNGLCOLG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXSNGLCOLG"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_single_collision_good_packets](index.html) module"]
pub struct TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_single_collision_good_packets::R](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
