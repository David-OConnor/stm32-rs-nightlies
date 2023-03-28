#[doc = "Register `CSR25` reader"]
pub struct R(crate::R<CSR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR25` writer"]
pub struct W(crate::W<CSR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR25_SPEC>;
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
impl From<crate::W<CSR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR25` reader - CSR25"]
pub type CSR25_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR25` writer - CSR25"]
pub type CSR25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR25_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&self) -> CSR25_R {
        CSR25_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    #[must_use]
    pub fn csr25(&mut self) -> CSR25_W<0> {
        CSR25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr25](index.html) module"]
pub struct CSR25_SPEC;
impl crate::RegisterSpec for CSR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr25::R](R) reader structure"]
impl crate::Readable for CSR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr25::W](W) writer structure"]
impl crate::Writable for CSR25_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR25 to value 0"]
impl crate::Resettable for CSR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
