#[doc = "Register `SECEPOCHR_CUR` reader"]
pub struct R(crate::R<SECEPOCHR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECEPOCHR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECEPOCHR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECEPOCHR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_EPOCH` reader - Non-volatile secure EPOCH counter"]
pub type SEC_EPOCH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Non-volatile secure EPOCH counter"]
    #[inline(always)]
    pub fn sec_epoch(&self) -> SEC_EPOCH_R {
        SEC_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FLASH secure EPOCH register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secepochr_cur](index.html) module"]
pub struct SECEPOCHR_CUR_SPEC;
impl crate::RegisterSpec for SECEPOCHR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secepochr_cur::R](R) reader structure"]
impl crate::Readable for SECEPOCHR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECEPOCHR_CUR to value 0"]
impl crate::Resettable for SECEPOCHR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
