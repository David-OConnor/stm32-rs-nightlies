#[doc = "Register `ETH_MTLRXQDR` reader"]
pub struct R(crate::R<ETH_MTLRXQDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRXQDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRXQDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRXQDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx queue."]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx queue:"]
pub type RXQSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx queue. The theoretical maximum value for this field is 256Kbyte/16bytes = 16K Packets, that is, Max_Queue_Size/Min_Packet_Size."]
pub type PRXQ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx queue."]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx queue:"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx queue. The theoretical maximum value for this field is 256Kbyte/16bytes = 16K Packets, that is, Max_Queue_Size/Min_Packet_Size."]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrxqdr](index.html) module"]
pub struct ETH_MTLRXQDR_SPEC;
impl crate::RegisterSpec for ETH_MTLRXQDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrxqdr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRXQDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLRXQDR to value 0"]
impl crate::Resettable for ETH_MTLRXQDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
