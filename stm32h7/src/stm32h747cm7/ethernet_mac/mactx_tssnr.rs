#[doc = "Register `MACTxTSSNR` reader"]
pub struct R(crate::R<MACTX_TSSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTX_TSSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTX_TSSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTX_TSSNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSLO` reader - Transmit Timestamp Status Low"]
pub type TXTSSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXTSSMIS` reader - Transmit Timestamp Status Missed"]
pub type TXTSSMIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - Transmit Timestamp Status Low"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Transmit Timestamp Status Missed"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactx_tssnr](index.html) module"]
pub struct MACTX_TSSNR_SPEC;
impl crate::RegisterSpec for MACTX_TSSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactx_tssnr::R](R) reader structure"]
impl crate::Readable for MACTX_TSSNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACTxTSSNR to value 0"]
impl crate::Resettable for MACTX_TSSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
