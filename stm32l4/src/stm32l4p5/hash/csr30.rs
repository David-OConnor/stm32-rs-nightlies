#[doc = "Register `CSR30` reader"]
pub struct R(crate::R<CSR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR30` writer"]
pub struct W(crate::W<CSR30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR30_SPEC>;
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
impl From<crate::W<CSR30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR30` reader - CSR30"]
pub type CSR30_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR30` writer - CSR30"]
pub type CSR30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR30_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    pub fn csr30(&self) -> CSR30_R {
        CSR30_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    #[must_use]
    pub fn csr30(&mut self) -> CSR30_W<0> {
        CSR30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr30](index.html) module"]
pub struct CSR30_SPEC;
impl crate::RegisterSpec for CSR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr30::R](R) reader structure"]
impl crate::Readable for CSR30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr30::W](W) writer structure"]
impl crate::Writable for CSR30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR30 to value 0"]
impl crate::Resettable for CSR30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
