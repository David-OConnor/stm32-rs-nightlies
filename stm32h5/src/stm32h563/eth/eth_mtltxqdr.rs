#[doc = "Register `ETH_MTLTXQDR` reader"]
pub struct R(crate::R<ETH_MTLTXQDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx queue is in the Pause condition (in the Full-duplex only mode) because of the following: Reception of the PFC packet for the priorities assigned to the Tx queue when PFC is enabled Reception of 802.3x Pause packet when PFC is disabled"]
pub type TXQPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx queue Write Controller is active, and it is transferring the data to the Tx queue."]
pub type TWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx queue is not empty and some data is left for transmission."]
pub type TXQSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full. Therefore, the MTL cannot accept any more packets for transmission."]
pub type TXSTSFSTS_R = crate::BitReader<bool>;
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx queue. When the DTXSTS bit of Operating mode Register (ETH_MTLOMR) register is set to 1, this field does not reflect the number of packets in the Transmit queue."]
pub type PTXQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue. When the DTXSTS bit of ETH_MTLOMR register is set to 1, this field does not reflect the number of status words in Tx Status FIFO."]
pub type STXSTSF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx queue is in the Pause condition (in the Full-duplex only mode) because of the following: Reception of the PFC packet for the priorities assigned to the Tx queue when PFC is enabled Reception of 802.3x Pause packet when PFC is disabled"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx queue Write Controller is active, and it is transferring the data to the Tx queue."]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx queue is not empty and some data is left for transmission."]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full. Therefore, the MTL cannot accept any more packets for transmission."]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx queue. When the DTXSTS bit of Operating mode Register (ETH_MTLOMR) register is set to 1, this field does not reflect the number of packets in the Transmit queue."]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue. When the DTXSTS bit of ETH_MTLOMR register is set to 1, this field does not reflect the number of status words in Tx Status FIFO."]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Tx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltxqdr](index.html) module"]
pub struct ETH_MTLTXQDR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltxqdr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLTXQDR to value 0"]
impl crate::Resettable for ETH_MTLTXQDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
