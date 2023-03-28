#[doc = "Register `CSR38` reader"]
pub struct R(crate::R<CSR38_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR38_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR38_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR38_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR38` writer"]
pub struct W(crate::W<CSR38_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR38_SPEC>;
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
impl From<crate::W<CSR38_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR38_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR38` reader - CSR38"]
pub type CSR38_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR38` writer - CSR38"]
pub type CSR38_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR38_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    pub fn csr38(&self) -> CSR38_R {
        CSR38_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    #[must_use]
    pub fn csr38(&mut self) -> CSR38_W<0> {
        CSR38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr38](index.html) module"]
pub struct CSR38_SPEC;
impl crate::RegisterSpec for CSR38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr38::R](R) reader structure"]
impl crate::Readable for CSR38_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr38::W](W) writer structure"]
impl crate::Writable for CSR38_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR38 to value 0"]
impl crate::Resettable for CSR38_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
