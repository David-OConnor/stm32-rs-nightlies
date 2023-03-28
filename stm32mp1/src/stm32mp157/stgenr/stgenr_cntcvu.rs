#[doc = "Register `STGENR_CNTCVU` reader"]
pub struct R(crate::R<STGENR_CNTCVU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_CNTCVU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_CNTCVU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_CNTCVU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTCVU_U_32` reader - CNTCVU_U_32"]
pub type CNTCVU_U_32_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new(self.bits)
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cntcvu](index.html) module"]
pub struct STGENR_CNTCVU_SPEC;
impl crate::RegisterSpec for STGENR_CNTCVU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_cntcvu::R](R) reader structure"]
impl crate::Readable for STGENR_CNTCVU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_CNTCVU to value 0"]
impl crate::Resettable for STGENR_CNTCVU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
