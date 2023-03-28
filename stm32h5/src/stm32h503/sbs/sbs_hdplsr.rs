#[doc = "Register `SBS_HDPLSR` reader"]
pub struct R(crate::R<SBS_HDPLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_HDPLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_HDPLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_HDPLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level."]
pub type HDPL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level."]
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SBS temporal isolation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_hdplsr](index.html) module"]
pub struct SBS_HDPLSR_SPEC;
impl crate::RegisterSpec for SBS_HDPLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_hdplsr::R](R) reader structure"]
impl crate::Readable for SBS_HDPLSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SBS_HDPLSR to value 0"]
impl crate::Resettable for SBS_HDPLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
