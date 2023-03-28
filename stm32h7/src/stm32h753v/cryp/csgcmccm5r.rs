#[doc = "Register `CSGCMCCM5R` reader"]
pub struct R(crate::R<CSGCMCCM5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM5R` writer"]
pub struct W(crate::W<CSGCMCCM5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM5R_SPEC>;
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
impl From<crate::W<CSGCMCCM5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM5` reader - CSGCMCCM5"]
pub type CSGCMCCM5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM5` writer - CSGCMCCM5"]
pub type CSGCMCCM5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSGCMCCM5R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    pub fn csgcmccm5(&self) -> CSGCMCCM5_R {
        CSGCMCCM5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm5(&mut self) -> CSGCMCCM5_W<0> {
        CSGCMCCM5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm5r](index.html) module"]
pub struct CSGCMCCM5R_SPEC;
impl crate::RegisterSpec for CSGCMCCM5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm5r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm5r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM5R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGCMCCM5R to value 0"]
impl crate::Resettable for CSGCMCCM5R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}