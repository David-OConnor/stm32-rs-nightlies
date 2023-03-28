#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear transfer error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEF_AW {
    #[doc = "1: Writing 1 clears the TEF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTEF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEF` writer - Clear transfer error flag"]
pub type CTEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, CTEF_AW, O>;
impl<'a, const O: u8> CTEF_W<'a, O> {
    #[doc = "Writing 1 clears the TEF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEF_AW::Clear)
    }
}
#[doc = "Clear transfer complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCF_AW {
    #[doc = "1: Writing 1 clears the TCF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, CTCF_AW, O>;
impl<'a, const O: u8> CTCF_W<'a, O> {
    #[doc = "Writing 1 clears the TCF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCF_AW::Clear)
    }
}
#[doc = "Clear status match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSMF_AW {
    #[doc = "1: Writing 1 clears the SMF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CSMF_AW> for bool {
    #[inline(always)]
    fn from(variant: CSMF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSMF` writer - Clear status match flag"]
pub type CSMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, CSMF_AW, O>;
impl<'a, const O: u8> CSMF_W<'a, O> {
    #[doc = "Writing 1 clears the SMF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSMF_AW::Clear)
    }
}
#[doc = "Clear timeout flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOF_AW {
    #[doc = "1: Writing 1 clears the TOF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTOF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOF` writer - Clear timeout flag"]
pub type CTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, CTOF_AW, O>;
impl<'a, const O: u8> CTOF_W<'a, O> {
    #[doc = "Writing 1 clears the TOF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTOF_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<0> {
        CTEF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<1> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 3 - Clear status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<3> {
        CSMF_W::new(self)
    }
    #[doc = "Bit 4 - Clear timeout flag"]
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
#[doc = "flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
