#[doc = "Register `LPTIM_RCR` reader"]
pub struct R(crate::R<LPTIM_RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_RCR` writer"]
pub struct W(crate::W<LPTIM_RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_RCR_SPEC>;
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
impl From<crate::W<LPTIM_RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - Repetition register value REP is the repetition value for the LPTIM."]
pub type REP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REP` writer - Repetition register value REP is the repetition value for the LPTIM."]
pub type REP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_RCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<0> {
        REP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM repetition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_rcr](index.html) module"]
pub struct LPTIM_RCR_SPEC;
impl crate::RegisterSpec for LPTIM_RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_rcr::R](R) reader structure"]
impl crate::Readable for LPTIM_RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_rcr::W](W) writer structure"]
impl crate::Writable for LPTIM_RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_RCR to value 0"]
impl crate::Resettable for LPTIM_RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
