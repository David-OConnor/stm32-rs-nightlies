#[doc = "Register `LTDC_SSCR` reader"]
pub struct R(crate::R<LTDC_SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_SSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_SSCR` writer"]
pub struct W(crate::W<LTDC_SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_SSCR_SPEC>;
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
impl From<crate::W<LTDC_SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_SSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSH` reader - VSH"]
pub type VSH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSH` writer - VSH"]
pub type VSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_SSCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `HSW` reader - HSW"]
pub type HSW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSW` writer - HSW"]
pub type HSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_SSCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    #[must_use]
    pub fn vsh(&mut self) -> VSH_W<0> {
        VSH_W::new(self)
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    #[must_use]
    pub fn hsw(&mut self) -> HSW_W<16> {
        HSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_sscr](index.html) module"]
pub struct LTDC_SSCR_SPEC;
impl crate::RegisterSpec for LTDC_SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_sscr::R](R) reader structure"]
impl crate::Readable for LTDC_SSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_sscr::W](W) writer structure"]
impl crate::Writable for LTDC_SSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_SSCR to value 0"]
impl crate::Resettable for LTDC_SSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
