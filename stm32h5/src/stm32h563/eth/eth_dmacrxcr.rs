#[doc = "Register `ETH_DMACRXCR` reader"]
pub struct R(crate::R<ETH_DMACRXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACRXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACRXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACRXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACRXCR` writer"]
pub struct W(crate::W<ETH_DMACRXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACRXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH_DMACRXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACRXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the address set by the Channel Rx descriptor list address register (ETH_DMACRXDLAR). The position at which the Rx process was previously stopped If the DMA does not own the current descriptor, the reception is suspended and the RBU bit of the ETH_DMACSR is set. The Start Receive command is effective only when the reception is stopped. If the command is issued before setting the Channel Rx descriptor list address register (ETH_DMACRXDLAR), the DMA behavior is unpredictable. When this bit is reset, the Rx DMA operation is stopped after the transfer of the current packet. The next descriptor position in the Receive list is saved, and it becomes the current position after the Rx process is restarted. The Stop Receive command is effective only when the Rx process is in the Running (waiting for Rx packet) or Suspended state."]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the address set by the Channel Rx descriptor list address register (ETH_DMACRXDLAR). The position at which the Rx process was previously stopped If the DMA does not own the current descriptor, the reception is suspended and the RBU bit of the ETH_DMACSR is set. The Start Receive command is effective only when the reception is stopped. If the command is issued before setting the Channel Rx descriptor list address register (ETH_DMACRXDLAR), the DMA behavior is unpredictable. When this bit is reset, the Rx DMA operation is stopped after the transfer of the current packet. The next descriptor position in the Receive list is saved, and it becomes the current position after the Rx process is restarted. The Stop Receive command is effective only when the Rx process is in the Running (waiting for Rx packet) or Suspended state."]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMACRXCR_SPEC, bool, O>;
#[doc = "Field `RBSZ` reader - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes. The maximum buffer size is limited to 16�Kbytes. Note: The buffer size must be a multiple of 4. This is required even if the value of buffer address pointer is not aligned to bus width. If the buffer size is not a multiple of 4, it may result into an undefined behavior. Note: The LSB bits (1:0) are ignored and the DMA internally takes the LSB bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type RBSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes. The maximum buffer size is limited to 16�Kbytes. Note: The buffer size must be a multiple of 4. This is required even if the value of buffer address pointer is not aligned to bus width. If the buffer size is not a multiple of 4, it may result into an undefined behavior. Note: The LSB bits (1:0) are ignored and the DMA internally takes the LSB bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
pub type RBSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACRXCR_SPEC, u16, u16, 14, O>;
#[doc = "Field `RXPBL` reader - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in the ETH_DMACCR. Set the RXPBL\\[5:0\\]. Note: The maximum value of RXPBL must be less than or equal to half the Rx Queue size (RQS field of Rx queue operating mode register (ETH_MTLRXQOMR)) in terms of beats. This is required so that the Rx Queue has space to store at least another Rx PBL worth of data while the MTL Rx Queue Controller is transferring data to MAC.The total locations in Rx Queue of size 2048 bytes is 512, RXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
pub type RXPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPBL` writer - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in the ETH_DMACCR. Set the RXPBL\\[5:0\\]. Note: The maximum value of RXPBL must be less than or equal to half the Rx Queue size (RQS field of Rx queue operating mode register (ETH_MTLRXQOMR)) in terms of beats. This is required so that the Rx Queue has space to store at least another Rx PBL worth of data while the MTL Rx Queue Controller is transferring data to MAC.The total locations in Rx Queue of size 2048 bytes is 512, RXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
pub type RXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACRXCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush When this bit is set to 1, the Ethernet peripheral will automatically flush the packet from the Rx queues destined to DMA Rx Channel when the DMA Rx Channel is stopped after a system bus error has occurred. When this bit remains set and the DMA is re-started by the software driver, the packets residing in the Rx Queues that were received when this RxDMA was stopped, are flushed out. The packets that are received by the MAC after the RxDMA is re-started are routed to the RxDMA. The flushing happens on the Read side of the Rx queue. When this bit is set to 0 the Ethernet peripheral does not flush the packet in the Rx queue destined to DMA Rx Channel after the DMA is stopped due to a system bus error. This might cause head-of-line blocking in the corresponding RxQueue. Note: The stopping of packet flow from a Rx DMA Channel to the application by setting RPF works only when there is one-to-one mapping of Rx Queue to Rx DMA channels. In Dynamic mapping mode, setting RPF bit in ETH_DMACRXCR register might flush packets from unintended Rx Queues which are destined to the stopped Rx DMA Channel."]
pub type RPF_R = crate::BitReader<bool>;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush When this bit is set to 1, the Ethernet peripheral will automatically flush the packet from the Rx queues destined to DMA Rx Channel when the DMA Rx Channel is stopped after a system bus error has occurred. When this bit remains set and the DMA is re-started by the software driver, the packets residing in the Rx Queues that were received when this RxDMA was stopped, are flushed out. The packets that are received by the MAC after the RxDMA is re-started are routed to the RxDMA. The flushing happens on the Read side of the Rx queue. When this bit is set to 0 the Ethernet peripheral does not flush the packet in the Rx queue destined to DMA Rx Channel after the DMA is stopped due to a system bus error. This might cause head-of-line blocking in the corresponding RxQueue. Note: The stopping of packet flow from a Rx DMA Channel to the application by setting RPF works only when there is one-to-one mapping of Rx Queue to Rx DMA channels. In Dynamic mapping mode, setting RPF bit in ETH_DMACRXCR register might flush packets from unintended Rx Queues which are destined to the stopped Rx DMA Channel."]
pub type RPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMACRXCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the address set by the Channel Rx descriptor list address register (ETH_DMACRXDLAR). The position at which the Rx process was previously stopped If the DMA does not own the current descriptor, the reception is suspended and the RBU bit of the ETH_DMACSR is set. The Start Receive command is effective only when the reception is stopped. If the command is issued before setting the Channel Rx descriptor list address register (ETH_DMACRXDLAR), the DMA behavior is unpredictable. When this bit is reset, the Rx DMA operation is stopped after the transfer of the current packet. The next descriptor position in the Receive list is saved, and it becomes the current position after the Rx process is restarted. The Stop Receive command is effective only when the Rx process is in the Running (waiting for Rx packet) or Suspended state."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes. The maximum buffer size is limited to 16�Kbytes. Note: The buffer size must be a multiple of 4. This is required even if the value of buffer address pointer is not aligned to bus width. If the buffer size is not a multiple of 4, it may result into an undefined behavior. Note: The LSB bits (1:0) are ignored and the DMA internally takes the LSB bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in the ETH_DMACCR. Set the RXPBL\\[5:0\\]. Note: The maximum value of RXPBL must be less than or equal to half the Rx Queue size (RQS field of Rx queue operating mode register (ETH_MTLRXQOMR)) in terms of beats. This is required so that the Rx Queue has space to store at least another Rx PBL worth of data while the MTL Rx Queue Controller is transferring data to MAC.The total locations in Rx Queue of size 2048 bytes is 512, RXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush When this bit is set to 1, the Ethernet peripheral will automatically flush the packet from the Rx queues destined to DMA Rx Channel when the DMA Rx Channel is stopped after a system bus error has occurred. When this bit remains set and the DMA is re-started by the software driver, the packets residing in the Rx Queues that were received when this RxDMA was stopped, are flushed out. The packets that are received by the MAC after the RxDMA is re-started are routed to the RxDMA. The flushing happens on the Read side of the Rx queue. When this bit is set to 0 the Ethernet peripheral does not flush the packet in the Rx queue destined to DMA Rx Channel after the DMA is stopped due to a system bus error. This might cause head-of-line blocking in the corresponding RxQueue. Note: The stopping of packet flow from a Rx DMA Channel to the application by setting RPF works only when there is one-to-one mapping of Rx Queue to Rx DMA channels. In Dynamic mapping mode, setting RPF bit in ETH_DMACRXCR register might flush packets from unintended Rx Queues which are destined to the stopped Rx DMA Channel."]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets. The DMA tries to acquire descriptor from either of the following positions: The current position in the list: this is the address set by the Channel Rx descriptor list address register (ETH_DMACRXDLAR). The position at which the Rx process was previously stopped If the DMA does not own the current descriptor, the reception is suspended and the RBU bit of the ETH_DMACSR is set. The Start Receive command is effective only when the reception is stopped. If the command is issued before setting the Channel Rx descriptor list address register (ETH_DMACRXDLAR), the DMA behavior is unpredictable. When this bit is reset, the Rx DMA operation is stopped after the transfer of the current packet. The next descriptor position in the Receive list is saved, and it becomes the current position after the Rx process is restarted. The Stop Receive command is effective only when the Rx process is in the Running (waiting for Rx packet) or Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    #[doc = "Bits 1:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes. The maximum buffer size is limited to 16�Kbytes. Note: The buffer size must be a multiple of 4. This is required even if the value of buffer address pointer is not aligned to bus width. If the buffer size is not a multiple of 4, it may result into an undefined behavior. Note: The LSB bits (1:0) are ignored and the DMA internally takes the LSB bits as all-zero. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    #[must_use]
    pub fn rbsz(&mut self) -> RBSZ_W<1> {
        RBSZ_W::new(self)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the application bus. You can program PBL with any of the following values: 1, 2, 4, 8, 16, or 32. Any other value results in undefined behavior. To transfer more than 32 beats, perform the following steps: Set the PBLx8 mode in the ETH_DMACCR. Set the RXPBL\\[5:0\\]. Note: The maximum value of RXPBL must be less than or equal to half the Rx Queue size (RQS field of Rx queue operating mode register (ETH_MTLRXQOMR)) in terms of beats. This is required so that the Rx Queue has space to store at least another Rx PBL worth of data while the MTL Rx Queue Controller is transferring data to MAC.The total locations in Rx Queue of size 2048 bytes is 512, RXPBL and 8xPBL needs to be programmed to less than or equal to 512/2."]
    #[inline(always)]
    #[must_use]
    pub fn rxpbl(&mut self) -> RXPBL_W<16> {
        RXPBL_W::new(self)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush When this bit is set to 1, the Ethernet peripheral will automatically flush the packet from the Rx queues destined to DMA Rx Channel when the DMA Rx Channel is stopped after a system bus error has occurred. When this bit remains set and the DMA is re-started by the software driver, the packets residing in the Rx Queues that were received when this RxDMA was stopped, are flushed out. The packets that are received by the MAC after the RxDMA is re-started are routed to the RxDMA. The flushing happens on the Read side of the Rx queue. When this bit is set to 0 the Ethernet peripheral does not flush the packet in the Rx queue destined to DMA Rx Channel after the DMA is stopped due to a system bus error. This might cause head-of-line blocking in the corresponding RxQueue. Note: The stopping of packet flow from a Rx DMA Channel to the application by setting RPF works only when there is one-to-one mapping of Rx Queue to Rx DMA channels. In Dynamic mapping mode, setting RPF bit in ETH_DMACRXCR register might flush packets from unintended Rx Queues which are destined to the stopped Rx DMA Channel."]
    #[inline(always)]
    #[must_use]
    pub fn rpf(&mut self) -> RPF_W<31> {
        RPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmacrxcr](index.html) module"]
pub struct ETH_DMACRXCR_SPEC;
impl crate::RegisterSpec for ETH_DMACRXCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmacrxcr::R](R) reader structure"]
impl crate::Readable for ETH_DMACRXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmacrxcr::W](W) writer structure"]
impl crate::Writable for ETH_DMACRXCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACRXCR to value 0"]
impl crate::Resettable for ETH_DMACRXCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
