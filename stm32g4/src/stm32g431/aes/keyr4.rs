#[doc = "Register `KEYR4` reader"]
pub struct R(crate::R<KEYR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR4` writer"]
pub struct W(crate::W<KEYR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR4_SPEC>;
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
impl From<crate::W<KEYR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - AES key"]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - AES key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr4](index.html) module"]
pub struct KEYR4_SPEC;
impl crate::RegisterSpec for KEYR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr4::R](R) reader structure"]
impl crate::Readable for KEYR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr4::W](W) writer structure"]
impl crate::Writable for KEYR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYR4 to value 0"]
impl crate::Resettable for KEYR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
