#[doc = "Register `CPT1ER` reader"]
pub struct R(crate::R<CPT1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT1ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type CPT1X_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1er](index.html) module"]
pub struct CPT1ER_SPEC;
impl crate::RegisterSpec for CPT1ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt1er::R](R) reader structure"]
impl crate::Readable for CPT1ER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPT1ER to value 0"]
impl crate::Resettable for CPT1ER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
