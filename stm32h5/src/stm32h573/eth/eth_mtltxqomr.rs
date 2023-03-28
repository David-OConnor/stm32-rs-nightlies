#[doc = "Register `ETH_MTLTXQOMR` reader"]
pub struct R(crate::R<ETH_MTLTXQOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTXQOMR` writer"]
pub struct W(crate::W<ETH_MTLTXQOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQOMR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTQ` reader - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active."]
pub type FTQ_R = crate::BitReader<bool>;
#[doc = "Field `FTQ` writer - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active."]
pub type FTQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTXQOMR_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\\[6:4\\]
of this register are ignored. This bit should be changed only when the transmission is stopped."]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\\[6:4\\]
of this register are ignored. This bit should be changed only when the transmission is stopped."]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTXQOMR_SPEC, bool, O>;
#[doc = "Field `TXQEN` reader - Transmit Queue Enable This field is used to enable/disable the transmit queue . Others: Reserved, must not be used. Note: In multiple Tx queues configuration, all the queues are disabled by default. Enable the Tx queue by programming this field."]
pub type TXQEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` reader - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset."]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset."]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTXQOMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TQS` reader - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111)."]
pub type TQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TQS` writer - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111)."]
pub type TQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTXQOMR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active."]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\\[6:4\\]
of this register are ignored. This bit should be changed only when the transmission is stopped."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue . Others: Reserved, must not be used. Note: In multiple Tx queues configuration, all the queues are disabled by default. Enable the Tx queue by programming this field."]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset."]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111)."]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values. Therefore, all the data in the Tx queue is lost or flushed. This bit is internally reset when the flushing operation is complete. Until this bit is reset, you should not write to the ETH_MTLTXQOMR register. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt packet transmission. Note: The flush operation is complete only when the Tx queue is empty and the application has accepted the pending Tx Status of all transmitted packets. To complete this flush operation, the PHY Tx clock (eth_mii_tx_clk) should be active."]
    #[inline(always)]
    #[must_use]
    pub fn ftq(&mut self) -> FTQ_W<0> {
        FTQ_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue. When this bit is set, the TTC values specified in Bits\\[6:4\\]
of this register are ignored. This bit should be changed only when the transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<1> {
        TSF_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx queue. The transmission starts when the packet size within the MTL Tx queue is larger than the threshold. In addition, full packets with length less than the threshold are also transmitted. These bits are used only when the TSF bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<4> {
        TTC_W::new(self)
    }
    #[doc = "Bits 16:18 - Transmit queue size This field indicates the size of the allocated transmit queues in blocks of 256 bytes. Queue size range from 256 bytes (TQS=0b000) to 2048 bytes (TQS=0b111)."]
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TQS_W<16> {
        TQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltxqomr](index.html) module"]
pub struct ETH_MTLTXQOMR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltxqomr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltxqomr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLTXQOMR to value 0x0007_0008"]
impl crate::Resettable for ETH_MTLTXQOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0008;
}
