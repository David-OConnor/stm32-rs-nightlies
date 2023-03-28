#[doc = "Register `ETH_MTLRXQMPOCR` reader"]
pub struct R(crate::R<ETH_MTLRXQMPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRXQMPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRXQMPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRXQMPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLRXQMPOCR` writer"]
pub struct W(crate::W<ETH_MTLRXQMPOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLRXQMPOCR_SPEC>;
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
impl From<crate::W<ETH_MTLRXQMPOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLRXQMPOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVFPKTCNT` reader - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type OVFPKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVFPKTCNT` writer - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read."]
pub type OVFPKTCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MTLRXQMPOCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type OVFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `OVFCNTOVF` writer - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
pub type OVFCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRXQMPOCR_SPEC, bool, O>;
#[doc = "Field `MISPKTCNT` reader - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MISPKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MISPKTCNT` writer - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability."]
pub type MISPKTCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MTLRXQMPOCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MISCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `MISCNTOVF` writer - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
pub type MISCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRXQMPOCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read."]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability."]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W<0> {
        OVFPKTCNT_W::new(self)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    #[inline(always)]
    #[must_use]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W<11> {
        OVFCNTOVF_W::new(self)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability."]
    #[inline(always)]
    #[must_use]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W<16> {
        MISPKTCNT_W::new(self)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    #[inline(always)]
    #[must_use]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W<27> {
        MISCNTOVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrxqmpocr](index.html) module"]
pub struct ETH_MTLRXQMPOCR_SPEC;
impl crate::RegisterSpec for ETH_MTLRXQMPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrxqmpocr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRXQMPOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlrxqmpocr::W](W) writer structure"]
impl crate::Writable for ETH_MTLRXQMPOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLRXQMPOCR to value 0"]
impl crate::Resettable for ETH_MTLRXQMPOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
