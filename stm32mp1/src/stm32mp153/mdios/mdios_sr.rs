#[doc = "Register `MDIOS_SR` reader"]
pub struct R(crate::R<MDIOS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERF` reader - PERF"]
pub type PERF_R = crate::BitReader<bool>;
#[doc = "Field `SERF` reader - SERF"]
pub type SERF_R = crate::BitReader<bool>;
#[doc = "Field `TERF` reader - TERF"]
pub type TERF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PERF"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERF"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TERF"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_sr](index.html) module"]
pub struct MDIOS_SR_SPEC;
impl crate::RegisterSpec for MDIOS_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_sr::R](R) reader structure"]
impl crate::Readable for MDIOS_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_SR to value 0"]
impl crate::Resettable for MDIOS_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
