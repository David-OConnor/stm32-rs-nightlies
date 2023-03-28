#[doc = "Register `CSR8` reader"]
pub struct R(crate::R<CSR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR8` writer"]
pub struct W(crate::W<CSR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR8_SPEC>;
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
impl From<crate::W<CSR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR8` reader - CSR8"]
pub type CSR8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR8` writer - CSR8"]
pub type CSR8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR8_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    pub fn csr8(&self) -> CSR8_R {
        CSR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    #[must_use]
    pub fn csr8(&mut self) -> CSR8_W<0> {
        CSR8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr8](index.html) module"]
pub struct CSR8_SPEC;
impl crate::RegisterSpec for CSR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr8::R](R) reader structure"]
impl crate::Readable for CSR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr8::W](W) writer structure"]
impl crate::Writable for CSR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR8 to value 0"]
impl crate::Resettable for CSR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
