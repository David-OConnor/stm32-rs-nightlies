#[doc = "Register `ETH_MTLTxQ0ESR` reader"]
pub struct R(crate::R<ETH_MTLTX_Q0ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTX_Q0ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTX_Q0ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTX_Q0ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ABS` reader - ABS"]
pub type ABS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - ABS"]
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Tx queue x ETS status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0esr](index.html) module"]
pub struct ETH_MTLTX_Q0ESR_SPEC;
impl crate::RegisterSpec for ETH_MTLTX_Q0ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q0esr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q0ESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLTxQ0ESR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q0ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
