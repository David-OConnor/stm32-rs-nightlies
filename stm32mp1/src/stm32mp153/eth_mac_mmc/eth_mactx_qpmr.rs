#[doc = "Register `ETH_MACTxQPMR` reader"]
pub struct R(crate::R<ETH_MACTX_QPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTX_QPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTX_QPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTX_QPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSTQ0` reader - PSTQ0"]
pub type PSTQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSTQ1` reader - PSTQ1"]
pub type PSTQ1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PSTQ0"]
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSTQ1"]
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_qpmr](index.html) module"]
pub struct ETH_MACTX_QPMR_SPEC;
impl crate::RegisterSpec for ETH_MACTX_QPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactx_qpmr::R](R) reader structure"]
impl crate::Readable for ETH_MACTX_QPMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACTxQPMR to value 0"]
impl crate::Resettable for ETH_MACTX_QPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
