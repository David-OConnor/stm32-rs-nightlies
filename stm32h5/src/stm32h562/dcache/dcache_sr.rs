#[doc = "Register `DCACHE_SR` reader"]
pub struct R(crate::R<DCACHE_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSYF` reader - full invalidate busy flag"]
pub type BUSYF_R = crate::BitReader<bool>;
#[doc = "Field `BSYENDF` reader - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
pub type BSYENDF_R = crate::BitReader<bool>;
#[doc = "Field `ERRF` reader - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
pub type ERRF_R = crate::BitReader<bool>;
#[doc = "Field `BUSYCMDF` reader - command busy flag"]
pub type BUSYCMDF_R = crate::BitReader<bool>;
#[doc = "Field `CMDENDF` reader - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
pub type CMDENDF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - full invalidate busy flag"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - command busy flag"]
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCACHE status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_sr](index.html) module"]
pub struct DCACHE_SR_SPEC;
impl crate::RegisterSpec for DCACHE_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_sr::R](R) reader structure"]
impl crate::Readable for DCACHE_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCACHE_SR to value 0x01"]
impl crate::Resettable for DCACHE_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
