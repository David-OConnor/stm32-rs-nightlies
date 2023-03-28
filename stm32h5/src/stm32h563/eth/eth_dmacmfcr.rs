#[doc = "Register `ETH_DMACMFCR` reader"]
pub struct R(crate::R<ETH_DMACMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACMFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMACMFCR` writer"]
pub struct W(crate::W<ETH_DMACMFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMACMFCR_SPEC>;
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
impl From<crate::W<ETH_DMACMFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMACMFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFC` reader - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFC` writer - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
pub type MFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMACMFCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MFCO_R = crate::BitReader<bool>;
#[doc = "Field `MFCO` writer - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
pub type MFCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMACMFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programing RPF field in Channel receive control register (ETH_DMACRXCR). The counter gets cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<0> {
        MFC_W::new(self)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further. The bit gets cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn mfco(&mut self) -> MFCO_W<15> {
        MFCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmacmfcr](index.html) module"]
pub struct ETH_DMACMFCR_SPEC;
impl crate::RegisterSpec for ETH_DMACMFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmacmfcr::R](R) reader structure"]
impl crate::Readable for ETH_DMACMFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmacmfcr::W](W) writer structure"]
impl crate::Writable for ETH_DMACMFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMACMFCR to value 0"]
impl crate::Resettable for ETH_DMACMFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
