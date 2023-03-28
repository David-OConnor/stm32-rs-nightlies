#[doc = "Register `CSR39` reader"]
pub struct R(crate::R<CSR39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR39` writer"]
pub struct W(crate::W<CSR39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR39_SPEC>;
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
impl From<crate::W<CSR39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR39` reader - CSR39"]
pub type CSR39_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR39` writer - CSR39"]
pub type CSR39_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR39_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR39"]
    #[inline(always)]
    pub fn csr39(&self) -> CSR39_R {
        CSR39_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR39"]
    #[inline(always)]
    #[must_use]
    pub fn csr39(&mut self) -> CSR39_W<0> {
        CSR39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr39](index.html) module"]
pub struct CSR39_SPEC;
impl crate::RegisterSpec for CSR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr39::R](R) reader structure"]
impl crate::Readable for CSR39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr39::W](W) writer structure"]
impl crate::Writable for CSR39_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR39 to value 0"]
impl crate::Resettable for CSR39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
