#[doc = "Register `FLT1JCHGR` reader"]
pub struct R(crate::R<FLT1JCHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1JCHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1JCHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1JCHGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT1JCHGR` writer"]
pub struct W(crate::W<FLT1JCHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT1JCHGR_SPEC>;
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
impl From<crate::W<FLT1JCHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT1JCHGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub type JCHG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub type JCHG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1JCHGR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<0> {
        JCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1jchgr](index.html) module"]
pub struct FLT1JCHGR_SPEC;
impl crate::RegisterSpec for FLT1JCHGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt1jchgr::R](R) reader structure"]
impl crate::Readable for FLT1JCHGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt1jchgr::W](W) writer structure"]
impl crate::Writable for FLT1JCHGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLT1JCHGR to value 0x01"]
impl crate::Resettable for FLT1JCHGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
