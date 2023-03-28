#[doc = "Register `CSR31` reader"]
pub struct R(crate::R<CSR31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR31` writer"]
pub struct W(crate::W<CSR31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR31_SPEC>;
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
impl From<crate::W<CSR31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR31` reader - CSR31"]
pub type CSR31_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR31` writer - CSR31"]
pub type CSR31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR31_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&self) -> CSR31_R {
        CSR31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    #[must_use]
    pub fn csr31(&mut self) -> CSR31_W<0> {
        CSR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr31](index.html) module"]
pub struct CSR31_SPEC;
impl crate::RegisterSpec for CSR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr31::R](R) reader structure"]
impl crate::Readable for CSR31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr31::W](W) writer structure"]
impl crate::Writable for CSR31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR31 to value 0"]
impl crate::Resettable for CSR31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
