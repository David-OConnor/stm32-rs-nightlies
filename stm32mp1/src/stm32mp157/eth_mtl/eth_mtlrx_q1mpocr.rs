#[doc = "Register `ETH_MTLRxQ1MPOCR` reader"]
pub struct R(crate::R<ETH_MTLRX_Q1MPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRX_Q1MPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRX_Q1MPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRX_Q1MPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFPKTCNT` reader - OVFPKTCNT"]
pub type OVFPKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVFCNTOVF` reader - OVFCNTOVF"]
pub type OVFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `MISPKTCNT` reader - MISPKTCNT"]
pub type MISPKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MISCNTOVF` reader - MISCNTOVF"]
pub type MISCNTOVF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - OVFPKTCNT"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - OVFCNTOVF"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - MISPKTCNT"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - MISCNTOVF"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Rx queue 1 missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1mpocr](index.html) module"]
pub struct ETH_MTLRX_Q1MPOCR_SPEC;
impl crate::RegisterSpec for ETH_MTLRX_Q1MPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q1mpocr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q1MPOCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLRxQ1MPOCR to value 0"]
impl crate::Resettable for ETH_MTLRX_Q1MPOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
