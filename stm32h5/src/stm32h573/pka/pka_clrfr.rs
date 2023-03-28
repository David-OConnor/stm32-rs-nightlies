#[doc = "Register `PKA_CLRFR` writer"]
pub struct W(crate::W<PKA_CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKA_CLRFR_SPEC>;
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
impl From<crate::W<PKA_CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKA_CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCENDFC` writer - Clear PKA End of Operation flag"]
pub type PROCENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CLRFR_SPEC, bool, O>;
#[doc = "Field `RAMERRFC` writer - Clear PKA RAM error flag"]
pub type RAMERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CLRFR_SPEC, bool, O>;
#[doc = "Field `ADDRERRFC` writer - Clear address error flag"]
pub type ADDRERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CLRFR_SPEC, bool, O>;
#[doc = "Field `OPERRFC` writer - Clear operation error flag"]
pub type OPERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CLRFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<17> {
        PROCENDFC_W::new(self)
    }
    #[doc = "Bit 19 - Clear PKA RAM error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<19> {
        RAMERRFC_W::new(self)
    }
    #[doc = "Bit 20 - Clear address error flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<20> {
        ADDRERRFC_W::new(self)
    }
    #[doc = "Bit 21 - Clear operation error flag"]
    #[inline(always)]
    #[must_use]
    pub fn operrfc(&mut self) -> OPERRFC_W<21> {
        OPERRFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_clrfr](index.html) module"]
pub struct PKA_CLRFR_SPEC;
impl crate::RegisterSpec for PKA_CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pka_clrfr::W](W) writer structure"]
impl crate::Writable for PKA_CLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKA_CLRFR to value 0"]
impl crate::Resettable for PKA_CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
