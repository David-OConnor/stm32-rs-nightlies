#[doc = "Register `MACTSICNR` reader"]
pub struct R(crate::R<MACTSICNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSICNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSICNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSICNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSICNR` writer"]
pub struct W(crate::W<MACTSICNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSICNR_SPEC>;
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
impl From<crate::W<MACTSICNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSICNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSIC` reader - Timestamp Ingress Correction"]
pub type TSIC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSIC` writer - Timestamp Ingress Correction"]
pub type TSIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTSICNR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Ingress Correction"]
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Ingress Correction"]
    #[inline(always)]
    #[must_use]
    pub fn tsic(&mut self) -> TSIC_W<0> {
        TSIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Ingress correction nanosecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsicnr](index.html) module"]
pub struct MACTSICNR_SPEC;
impl crate::RegisterSpec for MACTSICNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactsicnr::R](R) reader structure"]
impl crate::Readable for MACTSICNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactsicnr::W](W) writer structure"]
impl crate::Writable for MACTSICNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSICNR to value 0"]
impl crate::Resettable for MACTSICNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
