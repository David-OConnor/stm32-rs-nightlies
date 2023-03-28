#[doc = "Register `WRPR3` reader"]
pub struct R(crate::R<WRPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPR3` writer"]
pub struct W(crate::W<WRPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPR3_SPEC>;
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
impl From<crate::W<WRPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP3` reader - WRP3"]
pub type WRP3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRP3` writer - WRP3"]
pub type WRP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPR3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WRP3"]
    #[inline(always)]
    pub fn wrp3(&self) -> WRP3_R {
        WRP3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRP3"]
    #[inline(always)]
    #[must_use]
    pub fn wrp3(&mut self) -> WRP3_W<0> {
        WRP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr3](index.html) module"]
pub struct WRPR3_SPEC;
impl crate::RegisterSpec for WRPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr3::R](R) reader structure"]
impl crate::Readable for WRPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpr3::W](W) writer structure"]
impl crate::Writable for WRPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPR3 to value 0"]
impl crate::Resettable for WRPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
