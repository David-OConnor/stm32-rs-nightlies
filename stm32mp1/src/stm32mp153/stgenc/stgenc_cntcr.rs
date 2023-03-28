#[doc = "Register `STGENC_CNTCR` reader"]
pub struct R(crate::R<STGENC_CNTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTCR` writer"]
pub struct W(crate::W<STGENC_CNTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTCR_SPEC>;
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
impl From<crate::W<STGENC_CNTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STGENC_CNTCR_SPEC, bool, O>;
#[doc = "Field `HLTDBG` reader - HLTDBG"]
pub type HLTDBG_R = crate::BitReader<bool>;
#[doc = "Field `HLTDBG` writer - HLTDBG"]
pub type HLTDBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STGENC_CNTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    #[must_use]
    pub fn hltdbg(&mut self) -> HLTDBG_W<1> {
        HLTDBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STGENC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcr](index.html) module"]
pub struct STGENC_CNTCR_SPEC;
impl crate::RegisterSpec for STGENC_CNTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntcr::R](R) reader structure"]
impl crate::Readable for STGENC_CNTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcr::W](W) writer structure"]
impl crate::Writable for STGENC_CNTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STGENC_CNTCR to value 0"]
impl crate::Resettable for STGENC_CNTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
