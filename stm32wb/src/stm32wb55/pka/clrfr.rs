#[doc = "Register `CLRFR` reader"]
pub struct R(crate::R<CLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRFR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `PROCENDFC` reader - Clear PKA End of Operation flag"]
pub type PROCENDFC_R = crate::BitReader<bool>;
#[doc = "Field `PROCENDFC` writer - Clear PKA End of Operation flag"]
pub type PROCENDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `RAMERRFC` reader - Clear RAM error flag"]
pub type RAMERRFC_R = crate::BitReader<bool>;
#[doc = "Field `RAMERRFC` writer - Clear RAM error flag"]
pub type RAMERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `ADDRERRFC` reader - Clear Address error flag"]
pub type ADDRERRFC_R = crate::BitReader<bool>;
#[doc = "Field `ADDRERRFC` writer - Clear Address error flag"]
pub type ADDRERRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendfc(&self) -> PROCENDFC_R {
        PROCENDFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    pub fn ramerrfc(&self) -> RAMERRFC_R {
        RAMERRFC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&self) -> ADDRERRFC_R {
        ADDRERRFC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<17> {
        PROCENDFC_W::new(self)
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<19> {
        RAMERRFC_W::new(self)
    }
    #[doc = "Bit 20 - Clear Address error flag"]
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
#[doc = "PKA clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrfr::R](R) reader structure"]
impl crate::Readable for CLRFR_SPEC {
    type Reader = R;
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
