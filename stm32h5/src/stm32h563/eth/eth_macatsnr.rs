#[doc = "Register `ETH_MACATSNR` reader"]
pub struct R(crate::R<ETH_MACATSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACATSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACATSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACATSNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AUXTSLO` reader - Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp."]
pub type AUXTSLO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp."]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Auxiliary timestamp nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macatsnr](index.html) module"]
pub struct ETH_MACATSNR_SPEC;
impl crate::RegisterSpec for ETH_MACATSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macatsnr::R](R) reader structure"]
impl crate::Readable for ETH_MACATSNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACATSNR to value 0"]
impl crate::Resettable for ETH_MACATSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
