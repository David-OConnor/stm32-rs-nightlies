#[doc = "Register `WABR` reader"]
pub struct R(crate::R<WABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WABR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WABR` writer"]
pub struct W(crate::W<WABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WABR_SPEC>;
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
impl From<crate::W<WABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WABR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub type INSTRUCTION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub type INSTRUCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WABR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<0> {
        INSTRUCTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WABR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wabr](index.html) module"]
pub struct WABR_SPEC;
impl crate::RegisterSpec for WABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wabr::R](R) reader structure"]
impl crate::Readable for WABR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wabr::W](W) writer structure"]
impl crate::Writable for WABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WABR to value 0"]
impl crate::Resettable for WABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
