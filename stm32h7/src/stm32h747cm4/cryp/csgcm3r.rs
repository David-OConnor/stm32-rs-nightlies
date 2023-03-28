#[doc = "Register `CSGCM3R` reader"]
pub struct R(crate::R<CSGCM3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCM3R` writer"]
pub struct W(crate::W<CSGCM3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM3R_SPEC>;
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
impl From<crate::W<CSGCM3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM3R` reader - CSGCM3R"]
pub type CSGCM3R_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCM3R` writer - CSGCM3R"]
pub type CSGCM3R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM3R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCM3R"]
    #[inline(always)]
    pub fn csgcm3r(&self) -> CSGCM3R_R {
        CSGCM3R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM3R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm3r(&mut self) -> CSGCM3R_W<0> {
        CSGCM3R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm3r](index.html) module"]
pub struct CSGCM3R_SPEC;
impl crate::RegisterSpec for CSGCM3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcm3r::R](R) reader structure"]
impl crate::Readable for CSGCM3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcm3r::W](W) writer structure"]
impl crate::Writable for CSGCM3R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGCM3R to value 0"]
impl crate::Resettable for CSGCM3R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
