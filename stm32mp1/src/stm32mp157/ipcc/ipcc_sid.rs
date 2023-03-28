#[doc = "Register `IPCC_SID` reader"]
pub struct R(crate::R<IPCC_SID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_SID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_SID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_SID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "IPCC Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_sid](index.html) module"]
pub struct IPCC_SID_SPEC;
impl crate::RegisterSpec for IPCC_SID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_sid::R](R) reader structure"]
impl crate::Readable for IPCC_SID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCC_SID to value 0xa3c5_dd01"]
impl crate::Resettable for IPCC_SID_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
