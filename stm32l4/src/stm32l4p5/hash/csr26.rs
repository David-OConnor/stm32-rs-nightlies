#[doc = "Register `CSR26` reader"]
pub struct R(crate::R<CSR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR26` writer"]
pub struct W(crate::W<CSR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR26_SPEC>;
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
impl From<crate::W<CSR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR26` reader - CSR26"]
pub type CSR26_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR26` writer - CSR26"]
pub type CSR26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR26_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    pub fn csr26(&self) -> CSR26_R {
        CSR26_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    #[must_use]
    pub fn csr26(&mut self) -> CSR26_W<0> {
        CSR26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr26](index.html) module"]
pub struct CSR26_SPEC;
impl crate::RegisterSpec for CSR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr26::R](R) reader structure"]
impl crate::Readable for CSR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr26::W](W) writer structure"]
impl crate::Writable for CSR26_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR26 to value 0"]
impl crate::Resettable for CSR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
