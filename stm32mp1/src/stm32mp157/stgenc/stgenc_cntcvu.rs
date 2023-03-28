#[doc = "Register `STGENC_CNTCVU` reader"]
pub struct R(crate::R<STGENC_CNTCVU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTCVU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTCVU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTCVU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTCVU` writer"]
pub struct W(crate::W<STGENC_CNTCVU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTCVU_SPEC>;
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
impl From<crate::W<STGENC_CNTCVU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTCVU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTCVU_U_32` reader - CNTCVU_U_32"]
pub type CNTCVU_U_32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTCVU_U_32` writer - CNTCVU_U_32"]
pub type CNTCVU_U_32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STGENC_CNTCVU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    #[must_use]
    pub fn cntcvu_u_32(&mut self) -> CNTCVU_U_32_W<0> {
        CNTCVU_U_32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcvu](index.html) module"]
pub struct STGENC_CNTCVU_SPEC;
impl crate::RegisterSpec for STGENC_CNTCVU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntcvu::R](R) reader structure"]
impl crate::Readable for STGENC_CNTCVU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcvu::W](W) writer structure"]
impl crate::Writable for STGENC_CNTCVU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STGENC_CNTCVU to value 0"]
impl crate::Resettable for STGENC_CNTCVU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
