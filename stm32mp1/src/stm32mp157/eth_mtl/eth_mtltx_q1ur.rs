#[doc = "Register `ETH_MTLTxQ1UR` reader"]
pub struct R(crate::R<ETH_MTLTX_Q1UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTX_Q1UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTX_Q1UR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTX_Q1UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UFFRMCNT` reader - UFFRMCNT"]
pub type UFFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UFCNTOVF` reader - UFCNTOVF"]
pub type UFCNTOVF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - UFFRMCNT"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - UFCNTOVF"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Tx queue 1 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1ur](index.html) module"]
pub struct ETH_MTLTX_Q1UR_SPEC;
impl crate::RegisterSpec for ETH_MTLTX_Q1UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1ur::R](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1UR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1UR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1UR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
