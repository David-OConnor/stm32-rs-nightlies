#[doc = "Register `CSR48` reader"]
pub struct R(crate::R<CSR48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR48` writer"]
pub struct W(crate::W<CSR48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR48_SPEC>;
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
impl From<crate::W<CSR48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR48` reader - CSR48"]
pub type CSR48_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR48` writer - CSR48"]
pub type CSR48_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR48_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    pub fn csr48(&self) -> CSR48_R {
        CSR48_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    #[must_use]
    pub fn csr48(&mut self) -> CSR48_W<0> {
        CSR48_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr48](index.html) module"]
pub struct CSR48_SPEC;
impl crate::RegisterSpec for CSR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr48::R](R) reader structure"]
impl crate::Readable for CSR48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr48::W](W) writer structure"]
impl crate::Writable for CSR48_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR48 to value 0"]
impl crate::Resettable for CSR48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
