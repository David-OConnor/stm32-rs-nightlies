#[doc = "Register `ETH_DMACCATXDR` reader"]
pub struct R(crate::R<ETH_DMACCATXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACCATXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACCATXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACCATXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaccatxdr](index.html) module"]
pub struct ETH_DMACCATXDR_SPEC;
impl crate::RegisterSpec for ETH_DMACCATXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaccatxdr::R](R) reader structure"]
impl crate::Readable for ETH_DMACCATXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMACCATXDR to value 0"]
impl crate::Resettable for ETH_DMACCATXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
