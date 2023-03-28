#[doc = "Register `DDRPERFM_ID` reader"]
pub struct R(crate::R<DDRPERFM_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DDRPERFM ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_id](index.html) module"]
pub struct DDRPERFM_ID_SPEC;
impl crate::RegisterSpec for DDRPERFM_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_id::R](R) reader structure"]
impl crate::Readable for DDRPERFM_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_ID to value 0x0014_0061"]
impl crate::Resettable for DDRPERFM_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0014_0061;
}
