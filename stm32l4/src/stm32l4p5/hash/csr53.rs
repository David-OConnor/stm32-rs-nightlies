#[doc = "Register `CSR53` reader"]
pub struct R(crate::R<CSR53_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR53_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR53_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR53_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR53` writer"]
pub struct W(crate::W<CSR53_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR53_SPEC>;
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
impl From<crate::W<CSR53_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR53_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR53` reader - CSR53"]
pub type CSR53_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR53` writer - CSR53"]
pub type CSR53_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR53_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR53"]
    #[inline(always)]
    pub fn csr53(&self) -> CSR53_R {
        CSR53_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR53"]
    #[inline(always)]
    #[must_use]
    pub fn csr53(&mut self) -> CSR53_W<0> {
        CSR53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr53](index.html) module"]
pub struct CSR53_SPEC;
impl crate::RegisterSpec for CSR53_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr53::R](R) reader structure"]
impl crate::Readable for CSR53_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr53::W](W) writer structure"]
impl crate::Writable for CSR53_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR53 to value 0"]
impl crate::Resettable for CSR53_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
