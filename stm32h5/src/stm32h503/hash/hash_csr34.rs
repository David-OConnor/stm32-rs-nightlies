#[doc = "Register `HASH_CSR34` reader"]
pub struct R(crate::R<HASH_CSR34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR34` writer"]
pub struct W(crate::W<HASH_CSR34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR34_SPEC>;
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
impl From<crate::W<HASH_CSR34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS34` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CS34` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR34_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs34(&self) -> CS34_R {
        CS34_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs34(&mut self) -> CS34_W<0> {
        CS34_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr34](index.html) module"]
pub struct HASH_CSR34_SPEC;
impl crate::RegisterSpec for HASH_CSR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr34::R](R) reader structure"]
impl crate::Readable for HASH_CSR34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr34::W](W) writer structure"]
impl crate::Writable for HASH_CSR34_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_CSR34 to value 0"]
impl crate::Resettable for HASH_CSR34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
