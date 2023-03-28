#[doc = "Register `MACTSECNR` reader"]
pub struct R(crate::R<MACTSECNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSECNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSECNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSECNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSECNR` writer"]
pub struct W(crate::W<MACTSECNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSECNR_SPEC>;
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
impl From<crate::W<MACTSECNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSECNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEC` reader - Timestamp Egress Correction"]
pub type TSEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSEC` writer - Timestamp Egress Correction"]
pub type TSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTSECNR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Egress Correction"]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Egress Correction"]
    #[inline(always)]
    #[must_use]
    pub fn tsec(&mut self) -> TSEC_W<0> {
        TSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Egress correction nanosecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsecnr](index.html) module"]
pub struct MACTSECNR_SPEC;
impl crate::RegisterSpec for MACTSECNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactsecnr::R](R) reader structure"]
impl crate::Readable for MACTSECNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactsecnr::W](W) writer structure"]
impl crate::Writable for MACTSECNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSECNR to value 0"]
impl crate::Resettable for MACTSECNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
