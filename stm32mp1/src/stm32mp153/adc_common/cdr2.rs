#[doc = "Register `CDR2` reader"]
pub struct R(crate::R<CDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA_ALT` reader - RDATA_ALT"]
pub type RDATA_ALT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RDATA_ALT"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
#[doc = "Common regular data register for dual mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr2](index.html) module"]
pub struct CDR2_SPEC;
impl crate::RegisterSpec for CDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr2::R](R) reader structure"]
impl crate::Readable for CDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR2 to value 0"]
impl crate::Resettable for CDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
