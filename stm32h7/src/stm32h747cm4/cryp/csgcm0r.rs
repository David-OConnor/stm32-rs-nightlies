#[doc = "Register `CSGCM0R` reader"]
pub struct R(crate::R<CSGCM0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCM0R` writer"]
pub struct W(crate::W<CSGCM0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM0R_SPEC>;
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
impl From<crate::W<CSGCM0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM0R` reader - CSGCM0R"]
pub type CSGCM0R_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCM0R` writer - CSGCM0R"]
pub type CSGCM0R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM0R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcm0r(&self) -> CSGCM0R_R {
        CSGCM0R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm0r(&mut self) -> CSGCM0R_W<0> {
        CSGCM0R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm0r](index.html) module"]
pub struct CSGCM0R_SPEC;
impl crate::RegisterSpec for CSGCM0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcm0r::R](R) reader structure"]
impl crate::Readable for CSGCM0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcm0r::W](W) writer structure"]
impl crate::Writable for CSGCM0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGCM0R to value 0"]
impl crate::Resettable for CSGCM0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
