#[doc = "Register `DDRPERFM_SID` reader"]
pub struct R(crate::R<DDRPERFM_SID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_SID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_SID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_SID_SPEC>) -> Self {
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
#[doc = "DDRPERFM magic ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_sid](index.html) module"]
pub struct DDRPERFM_SID_SPEC;
impl crate::RegisterSpec for DDRPERFM_SID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_sid::R](R) reader structure"]
impl crate::Readable for DDRPERFM_SID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_SID to value 0xa3c5_dd01"]
impl crate::Resettable for DDRPERFM_SID_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
