#[doc = "Register `NSEPOCHR_CUR` reader"]
pub struct R(crate::R<NSEPOCHR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSEPOCHR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSEPOCHR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSEPOCHR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NS_EPOCH` reader - Non-volatile non-secure EPOCH counter"]
pub type NS_EPOCH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Non-volatile non-secure EPOCH counter"]
    #[inline(always)]
    pub fn ns_epoch(&self) -> NS_EPOCH_R {
        NS_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FLASH non-secure EPOCH register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsepochr_cur](index.html) module"]
pub struct NSEPOCHR_CUR_SPEC;
impl crate::RegisterSpec for NSEPOCHR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsepochr_cur::R](R) reader structure"]
impl crate::Readable for NSEPOCHR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NSEPOCHR_CUR to value 0"]
impl crate::Resettable for NSEPOCHR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
