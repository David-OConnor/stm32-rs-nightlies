#[doc = "Register `DCACHE_RMMONR` reader"]
pub struct R(crate::R<DCACHE_RMMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_RMMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_RMMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_RMMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMISSMON` reader - cache read-miss monitor counter"]
pub type RMISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - cache read-miss monitor counter"]
    #[inline(always)]
    pub fn rmissmon(&self) -> RMISSMON_R {
        RMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DCACHE read-miss monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_rmmonr](index.html) module"]
pub struct DCACHE_RMMONR_SPEC;
impl crate::RegisterSpec for DCACHE_RMMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_rmmonr::R](R) reader structure"]
impl crate::Readable for DCACHE_RMMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCACHE_RMMONR to value 0"]
impl crate::Resettable for DCACHE_RMMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
