#[doc = "Register `WRPR1` reader"]
pub struct R(crate::R<WRPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPR1` writer"]
pub struct W(crate::W<WRPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPR1_SPEC>;
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
impl From<crate::W<WRPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP1` reader - Write protection"]
pub type WRP1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRP1` writer - Write protection"]
pub type WRP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1(&mut self) -> WRP1_W<0> {
        WRP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr1](index.html) module"]
pub struct WRPR1_SPEC;
impl crate::RegisterSpec for WRPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr1::R](R) reader structure"]
impl crate::Readable for WRPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpr1::W](W) writer structure"]
impl crate::Writable for WRPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPR1 to value 0"]
impl crate::Resettable for WRPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
