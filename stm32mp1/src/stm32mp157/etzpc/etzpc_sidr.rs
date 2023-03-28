#[doc = "Register `ETZPC_SIDR` reader"]
pub struct R(crate::R<ETZPC_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_SIDR_SPEC>) -> Self {
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
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_sidr](index.html) module"]
pub struct ETZPC_SIDR_SPEC;
impl crate::RegisterSpec for ETZPC_SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_sidr::R](R) reader structure"]
impl crate::Readable for ETZPC_SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETZPC_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for ETZPC_SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
