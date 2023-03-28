#[doc = "Register `VVACR` reader"]
pub struct R(crate::R<VVACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VVACR` writer"]
pub struct W(crate::W<VVACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVACR_SPEC>;
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
impl From<crate::W<VVACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VA` reader - Vertical active duration"]
pub type VA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VA` writer - Vertical active duration"]
pub type VA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVACR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Vertical active duration"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Vertical active duration"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<0> {
        VA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video VA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvacr](index.html) module"]
pub struct VVACR_SPEC;
impl crate::RegisterSpec for VVACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvacr::R](R) reader structure"]
impl crate::Readable for VVACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vvacr::W](W) writer structure"]
impl crate::Writable for VVACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VVACR to value 0"]
impl crate::Resettable for VVACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
