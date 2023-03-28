#[doc = "Register `STGENC_CNTCVL` reader"]
pub struct R(crate::R<STGENC_CNTCVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTCVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTCVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTCVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTCVL` writer"]
pub struct W(crate::W<STGENC_CNTCVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTCVL_SPEC>;
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
impl From<crate::W<STGENC_CNTCVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTCVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTCVL_L_32` reader - CNTCVL_L_32"]
pub type CNTCVL_L_32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTCVL_L_32` writer - CNTCVL_L_32"]
pub type CNTCVL_L_32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STGENC_CNTCVL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    #[must_use]
    pub fn cntcvl_l_32(&mut self) -> CNTCVL_L_32_W<0> {
        CNTCVL_L_32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcvl](index.html) module"]
pub struct STGENC_CNTCVL_SPEC;
impl crate::RegisterSpec for STGENC_CNTCVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntcvl::R](R) reader structure"]
impl crate::Readable for STGENC_CNTCVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcvl::W](W) writer structure"]
impl crate::Writable for STGENC_CNTCVL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STGENC_CNTCVL to value 0"]
impl crate::Resettable for STGENC_CNTCVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
