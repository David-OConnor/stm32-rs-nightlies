#[doc = "Register `DMA_IPDR` reader"]
pub struct R(crate::R<DMA_IPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DMA IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ipdr](index.html) module"]
pub struct DMA_IPDR_SPEC;
impl crate::RegisterSpec for DMA_IPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ipdr::R](R) reader structure"]
impl crate::Readable for DMA_IPDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IPDR to value 0x0010_0002"]
impl crate::Resettable for DMA_IPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0002;
}
