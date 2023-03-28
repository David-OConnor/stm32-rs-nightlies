#[doc = "Register `ETH_MACSTSR` reader"]
pub struct R(crate::R<ETH_MACSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
pub type TSS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstsr](index.html) module"]
pub struct ETH_MACSTSR_SPEC;
impl crate::RegisterSpec for ETH_MACSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstsr::R](R) reader structure"]
impl crate::Readable for ETH_MACSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACSTSR to value 0"]
impl crate::Resettable for ETH_MACSTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
