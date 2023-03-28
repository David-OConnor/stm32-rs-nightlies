#[doc = "Register `GTZC1_TZSC_MPCWM4ACFGR` reader"]
pub struct R(crate::R<GTZC1_TZSC_MPCWM4ACFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_MPCWM4ACFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_MPCWM4ACFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_MPCWM4ACFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_MPCWM4ACFGR` writer"]
pub struct W(crate::W<GTZC1_TZSC_MPCWM4ACFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_MPCWM4ACFGR_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_MPCWM4ACFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_MPCWM4ACFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SREN` reader - Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
pub type SREN_R = crate::BitReader<bool>;
#[doc = "Field `SREN` writer - Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
pub type SREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_MPCWM4ACFGR_SPEC, bool, O>;
#[doc = "Field `SRLOCK` reader - Sub-region A lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_R = crate::BitReader<bool>;
#[doc = "Field `SRLOCK` writer - Sub-region A lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_MPCWM4ACFGR_SPEC, bool, O>;
#[doc = "Field `SEC` reader - Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
pub type SEC_R = crate::BitReader<bool>;
#[doc = "Field `SEC` writer - Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_MPCWM4ACFGR_SPEC, bool, O>;
#[doc = "Field `PRIV` reader - Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
pub type PRIV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV` writer - Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_MPCWM4ACFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub-region A lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-region z enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<0> {
        SREN_W::new(self)
    }
    #[doc = "Bit 1 - Sub-region A lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn srlock(&mut self) -> SRLOCK_W<1> {
        SRLOCK_W::new(self)
    }
    #[doc = "Bit 8 - Secure sub-region A of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<8> {
        SEC_W::new(self)
    }
    #[doc = "Bit 9 - Privileged sub-region A of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<9> {
        PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_mpcwm4acfgr](index.html) module"]
pub struct GTZC1_TZSC_MPCWM4ACFGR_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_MPCWM4ACFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_mpcwm4acfgr::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_MPCWM4ACFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_mpcwm4acfgr::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_MPCWM4ACFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_MPCWM4ACFGR to value 0"]
impl crate::Resettable for GTZC1_TZSC_MPCWM4ACFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
