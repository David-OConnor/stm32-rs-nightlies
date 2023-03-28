#[doc = "Register `DDRPERFM_CNT2` reader"]
pub struct R(crate::R<DDRPERFM_CNT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CNT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CNT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CNT2_SPEC>) -> Self {
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
#[doc = "DDRPERFM event counter 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt2](index.html) module"]
pub struct DDRPERFM_CNT2_SPEC;
impl crate::RegisterSpec for DDRPERFM_CNT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_cnt2::R](R) reader structure"]
impl crate::Readable for DDRPERFM_CNT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_CNT2 to value 0"]
impl crate::Resettable for DDRPERFM_CNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
