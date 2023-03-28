#[doc = "Register `ICACHE_MMONR` reader"]
pub struct R(crate::R<ICACHE_MMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_MMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_MMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_MMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISSMON` reader - cache miss monitor counter"]
pub type MISSMON_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - cache miss monitor counter"]
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ICACHE miss monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_mmonr](index.html) module"]
pub struct ICACHE_MMONR_SPEC;
impl crate::RegisterSpec for ICACHE_MMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_mmonr::R](R) reader structure"]
impl crate::Readable for ICACHE_MMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICACHE_MMONR to value 0"]
impl crate::Resettable for ICACHE_MMONR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
