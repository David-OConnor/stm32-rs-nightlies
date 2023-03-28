#[doc = "Register `ETH_DMACCARXDR` reader"]
pub struct R(crate::R<ETH_DMACCARXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMACCARXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMACCARXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMACCARXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaccarxdr](index.html) module"]
pub struct ETH_DMACCARXDR_SPEC;
impl crate::RegisterSpec for ETH_DMACCARXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaccarxdr::R](R) reader structure"]
impl crate::Readable for ETH_DMACCARXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMACCARXDR to value 0"]
impl crate::Resettable for ETH_DMACCARXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
