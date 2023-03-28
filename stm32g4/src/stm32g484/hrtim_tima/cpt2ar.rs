#[doc = "Register `CPT2AR` reader"]
pub struct R(crate::R<CPT2AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type CPT2X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIR` reader - Timerx Capture 1 Direction status"]
pub type DIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2ar](index.html) module"]
pub struct CPT2AR_SPEC;
impl crate::RegisterSpec for CPT2AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt2ar::R](R) reader structure"]
impl crate::Readable for CPT2AR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPT2AR to value 0"]
impl crate::Resettable for CPT2AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
