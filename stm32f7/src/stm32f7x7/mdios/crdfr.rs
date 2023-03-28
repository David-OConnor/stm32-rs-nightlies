#[doc = "Register `CRDFR` reader"]
pub struct R(crate::R<CRDFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRDFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRDFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRDFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRDFR` writer"]
pub struct W(crate::W<CRDFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRDFR_SPEC>;
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
impl From<crate::W<CRDFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRDFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRDF` reader - Clear the read flag"]
pub type CRDF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRDF` writer - Clear the read flag"]
pub type CRDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRDFR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    #[must_use]
    pub fn crdf(&mut self) -> CRDF_W<0> {
        CRDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS clear read flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crdfr](index.html) module"]
pub struct CRDFR_SPEC;
impl crate::RegisterSpec for CRDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crdfr::R](R) reader structure"]
impl crate::Readable for CRDFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crdfr::W](W) writer structure"]
impl crate::Writable for CRDFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRDFR to value 0"]
impl crate::Resettable for CRDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
