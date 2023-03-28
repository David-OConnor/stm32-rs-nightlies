#[doc = "Register `MDIOS_CRDFR` reader"]
pub struct R(crate::R<MDIOS_CRDFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_CRDFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_CRDFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_CRDFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIOS_CRDFR` writer"]
pub struct W(crate::W<MDIOS_CRDFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_CRDFR_SPEC>;
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
impl From<crate::W<MDIOS_CRDFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_CRDFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRDF` reader - CRDF"]
pub type CRDF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRDF` writer - CRDF"]
pub type CRDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDIOS_CRDFR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRDF"]
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRDF"]
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
#[doc = "MDIOS clear read flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_crdfr](index.html) module"]
pub struct MDIOS_CRDFR_SPEC;
impl crate::RegisterSpec for MDIOS_CRDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_crdfr::R](R) reader structure"]
impl crate::Readable for MDIOS_CRDFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdios_crdfr::W](W) writer structure"]
impl crate::Writable for MDIOS_CRDFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_CRDFR to value 0"]
impl crate::Resettable for MDIOS_CRDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
