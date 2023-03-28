#[doc = "Register `WCCR` reader"]
pub struct R(crate::R<WCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCCR` writer"]
pub struct W(crate::W<WCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCCR_SPEC>;
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
impl From<crate::W<WCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFRESH` reader - REFRESH"]
pub type REFRESH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REFRESH` writer - REFRESH"]
pub type REFRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<0> {
        REFRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wccr](index.html) module"]
pub struct WCCR_SPEC;
impl crate::RegisterSpec for WCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wccr::R](R) reader structure"]
impl crate::Readable for WCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wccr::W](W) writer structure"]
impl crate::Writable for WCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCCR to value 0"]
impl crate::Resettable for WCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
