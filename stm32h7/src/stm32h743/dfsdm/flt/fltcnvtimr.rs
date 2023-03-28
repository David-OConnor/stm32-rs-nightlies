#[doc = "Register `FLTCNVTIMR` reader"]
pub struct R(crate::R<FLTCNVTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCNVTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCNVTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCNVTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time"]
pub type CNVCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltcnvtimr](index.html) module"]
pub struct FLTCNVTIMR_SPEC;
impl crate::RegisterSpec for FLTCNVTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltcnvtimr::R](R) reader structure"]
impl crate::Readable for FLTCNVTIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLTCNVTIMR to value 0"]
impl crate::Resettable for FLTCNVTIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
