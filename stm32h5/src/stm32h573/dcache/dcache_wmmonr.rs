#[doc = "Register `DCACHE_WMMONR` reader"]
pub struct R(crate::R<DCACHE_WMMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_WMMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_WMMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_WMMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WMISSMON` reader - cache write-miss monitor counter"]
pub type WMISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - cache write-miss monitor counter"]
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DCACHE write-miss monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_wmmonr](index.html) module"]
pub struct DCACHE_WMMONR_SPEC;
impl crate::RegisterSpec for DCACHE_WMMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_wmmonr::R](R) reader structure"]
impl crate::Readable for DCACHE_WMMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCACHE_WMMONR to value 0"]
impl crate::Resettable for DCACHE_WMMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
