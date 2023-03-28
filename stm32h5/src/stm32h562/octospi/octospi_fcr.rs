#[doc = "Register `OCTOSPI_FCR` writer"]
pub struct W(crate::W<OCTOSPI_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_FCR_SPEC>;
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
impl From<crate::W<OCTOSPI_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the OCTOSPI_SR register."]
pub type CTEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_FCR_SPEC, bool, O>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the OCTOSPI_SR register."]
pub type CTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_FCR_SPEC, bool, O>;
#[doc = "Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the OCTOSPI_SR register."]
pub type CSMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_FCR_SPEC, bool, O>;
#[doc = "Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the OCTOSPI_SR register."]
pub type CTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_FCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the OCTOSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<0> {
        CTEF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the OCTOSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<1> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the OCTOSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<3> {
        CSMF_W::new(self)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the OCTOSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<4> {
        CTOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_fcr](index.html) module"]
pub struct OCTOSPI_FCR_SPEC;
impl crate::RegisterSpec for OCTOSPI_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [octospi_fcr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_FCR to value 0"]
impl crate::Resettable for OCTOSPI_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
