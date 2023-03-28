#[doc = "Register `CPT2DR` reader"]
pub struct R(crate::R<CPT2DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type CPT2X_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2dr](index.html) module"]
pub struct CPT2DR_SPEC;
impl crate::RegisterSpec for CPT2DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt2dr::R](R) reader structure"]
impl crate::Readable for CPT2DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPT2DR to value 0"]
impl crate::Resettable for CPT2DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}