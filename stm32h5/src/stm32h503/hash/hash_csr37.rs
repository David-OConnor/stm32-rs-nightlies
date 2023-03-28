#[doc = "Register `HASH_CSR37` reader"]
pub struct R(crate::R<HASH_CSR37_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR37_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR37_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR37_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR37` writer"]
pub struct W(crate::W<HASH_CSR37_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR37_SPEC>;
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
impl From<crate::W<HASH_CSR37_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR37_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS37` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS37_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CS37` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS37_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR37_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs37(&self) -> CS37_R {
        CS37_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs37(&mut self) -> CS37_W<0> {
        CS37_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap register 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr37](index.html) module"]
pub struct HASH_CSR37_SPEC;
impl crate::RegisterSpec for HASH_CSR37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr37::R](R) reader structure"]
impl crate::Readable for HASH_CSR37_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr37::W](W) writer structure"]
impl crate::Writable for HASH_CSR37_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_CSR37 to value 0"]
impl crate::Resettable for HASH_CSR37_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
