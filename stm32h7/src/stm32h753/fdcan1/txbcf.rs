#[doc = "Register `TXBCF` reader"]
pub struct R(crate::R<TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF` reader - Cancellation Finished"]
pub type CF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](index.html) module"]
pub struct TXBCF_SPEC;
impl crate::RegisterSpec for TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcf::R](R) reader structure"]
impl crate::Readable for TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
