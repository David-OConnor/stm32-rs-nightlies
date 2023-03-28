#[doc = "Register `ETH_MACTxTSSNR` reader"]
pub struct R(crate::R<ETH_MACTX_TSSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTX_TSSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTX_TSSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTX_TSSNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSLO` reader - TXTSSLO"]
pub type TXTSSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXTSSMIS` reader - TXTSSMIS"]
pub type TXTSSMIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - TXTSSLO"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TXTSSMIS"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_tssnr](index.html) module"]
pub struct ETH_MACTX_TSSNR_SPEC;
impl crate::RegisterSpec for ETH_MACTX_TSSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactx_tssnr::R](R) reader structure"]
impl crate::Readable for ETH_MACTX_TSSNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACTxTSSNR to value 0"]
impl crate::Resettable for ETH_MACTX_TSSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
