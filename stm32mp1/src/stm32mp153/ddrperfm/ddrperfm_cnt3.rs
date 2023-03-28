#[doc = "Register `DDRPERFM_CNT3` reader"]
pub struct R(crate::R<DDRPERFM_CNT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CNT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CNT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CNT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "DDRPERFM event counter 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt3](index.html) module"]
pub struct DDRPERFM_CNT3_SPEC;
impl crate::RegisterSpec for DDRPERFM_CNT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_cnt3::R](R) reader structure"]
impl crate::Readable for DDRPERFM_CNT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_CNT3 to value 0"]
impl crate::Resettable for DDRPERFM_CNT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
