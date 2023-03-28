#[doc = "Register `STGENC_CNTFID0` reader"]
pub struct R(crate::R<STGENC_CNTFID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTFID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTFID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTFID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTFID0` writer"]
pub struct W(crate::W<STGENC_CNTFID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTFID0_SPEC>;
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
impl From<crate::W<STGENC_CNTFID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTFID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - FREQ"]
pub type FREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FREQ` writer - FREQ"]
pub type FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STGENC_CNTFID0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntfid0](index.html) module"]
pub struct STGENC_CNTFID0_SPEC;
impl crate::RegisterSpec for STGENC_CNTFID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntfid0::R](R) reader structure"]
impl crate::Readable for STGENC_CNTFID0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntfid0::W](W) writer structure"]
impl crate::Writable for STGENC_CNTFID0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STGENC_CNTFID0 to value 0"]
impl crate::Resettable for STGENC_CNTFID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
