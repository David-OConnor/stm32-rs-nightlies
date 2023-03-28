#[doc = "Register `SCCR` reader"]
pub struct R(crate::R<SCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCR` writer"]
pub struct W(crate::W<SCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCR_SPEC>;
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
impl From<crate::W<SCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - power management unit bypass"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - power management unit bypass"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCCR_SPEC, bool, O>;
#[doc = "Field `LDOEN` reader - LDO enable The value is set by hardware when the package uses the LDO regulator."]
pub type LDOEN_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEN` reader - SMPS enable The value is set by hardware when the package uses the SMPS regulator."]
pub type SMPSEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - LDO enable The value is set by hardware when the package uses the LDO regulator."]
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPS enable The value is set by hardware when the package uses the SMPS regulator."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - power management unit bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR supply configuration control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccr](index.html) module"]
pub struct SCCR_SPEC;
impl crate::RegisterSpec for SCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sccr::R](R) reader structure"]
impl crate::Readable for SCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccr::W](W) writer structure"]
impl crate::Writable for SCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
