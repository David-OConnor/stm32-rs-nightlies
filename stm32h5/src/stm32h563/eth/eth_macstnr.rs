#[doc = "Register `ETH_MACSTNR` reader"]
pub struct R(crate::R<ETH_MACSTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSS` reader - Timestamp subseconds The value in this field has the subsecond representation of time, with an accuracy of 0.46 ns. When TSCTRLSSR is set in Timestamp control Register (ETH_MACTSCR), each bit represents 1 ns. The maximum value is 0x3B9A_C9FF after which it rolls-over to zero."]
pub type TSSS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp subseconds The value in this field has the subsecond representation of time, with an accuracy of 0.46 ns. When TSCTRLSSR is set in Timestamp control Register (ETH_MACTSCR), each bit represents 1 ns. The maximum value is 0x3B9A_C9FF after which it rolls-over to zero."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstnr](index.html) module"]
pub struct ETH_MACSTNR_SPEC;
impl crate::RegisterSpec for ETH_MACSTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstnr::R](R) reader structure"]
impl crate::Readable for ETH_MACSTNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACSTNR to value 0"]
impl crate::Resettable for ETH_MACSTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
