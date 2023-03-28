#[doc = "Register `DCACHE_RHMONR` reader"]
pub struct R(crate::R<DCACHE_RHMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_RHMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_RHMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_RHMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RHITMON` reader - cache read-hit monitor counter"]
pub type RHITMON_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - cache read-hit monitor counter"]
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
#[doc = "DCACHE read-hit monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_rhmonr](index.html) module"]
pub struct DCACHE_RHMONR_SPEC;
impl crate::RegisterSpec for DCACHE_RHMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_rhmonr::R](R) reader structure"]
impl crate::Readable for DCACHE_RHMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCACHE_RHMONR to value 0"]
impl crate::Resettable for DCACHE_RHMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
