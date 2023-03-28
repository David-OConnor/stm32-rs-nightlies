#[doc = "Register `SBS_CSLCKR` reader"]
pub struct R(crate::R<SBS_CSLCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_CSLCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_CSLCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_CSLCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_CSLCKR` writer"]
pub struct W(crate::W<SBS_CSLCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_CSLCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SBS_CSLCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_CSLCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR` writer - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
pub type LOCKSVTAIRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CSLCKR_SPEC, bool, O>;
#[doc = "Field `LOCKSMPU` reader - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
pub type LOCKSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU` writer - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
pub type LOCKSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CSLCKR_SPEC, bool, O>;
#[doc = "Field `LOCKSAU` reader - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
pub type LOCKSAU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU` writer - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
pub type LOCKSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CSLCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<0> {
        LOCKSVTAIRCR_W::new(self)
    }
    #[doc = "Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<1> {
        LOCKSMPU_W::new(self)
    }
    #[doc = "Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<2> {
        LOCKSAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS CPU secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_cslckr](index.html) module"]
pub struct SBS_CSLCKR_SPEC;
impl crate::RegisterSpec for SBS_CSLCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_cslckr::R](R) reader structure"]
impl crate::Readable for SBS_CSLCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_cslckr::W](W) writer structure"]
impl crate::Writable for SBS_CSLCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_CSLCKR to value 0"]
impl crate::Resettable for SBS_CSLCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
