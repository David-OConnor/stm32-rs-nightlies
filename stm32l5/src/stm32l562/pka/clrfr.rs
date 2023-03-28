#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCENDFC` writer - clear PKA end of operation flag"]
pub type PROCENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `RAMERRFC` writer - CLEAR PKA RAM ERROR FLAG"]
pub type RAMERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `ADDRERRFC` writer - clear address error flag"]
pub type ADDRERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 17 - clear PKA end of operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<17> {
        PROCENDFC_W::new(self)
    }
    #[doc = "Bit 19 - CLEAR PKA RAM ERROR FLAG"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<19> {
        RAMERRFC_W::new(self)
    }
    #[doc = "Bit 20 - clear address error flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<20> {
        ADDRERRFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
