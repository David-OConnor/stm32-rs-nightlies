#[doc = "Register `CSR49` reader"]
pub struct R(crate::R<CSR49_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR49_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR49_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR49_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR49` writer"]
pub struct W(crate::W<CSR49_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR49_SPEC>;
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
impl From<crate::W<CSR49_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR49_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR49` reader - CSR49"]
pub type CSR49_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR49` writer - CSR49"]
pub type CSR49_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR49_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    pub fn csr49(&self) -> CSR49_R {
        CSR49_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    #[must_use]
    pub fn csr49(&mut self) -> CSR49_W<0> {
        CSR49_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr49](index.html) module"]
pub struct CSR49_SPEC;
impl crate::RegisterSpec for CSR49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr49::R](R) reader structure"]
impl crate::Readable for CSR49_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr49::W](W) writer structure"]
impl crate::Writable for CSR49_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR49 to value 0"]
impl crate::Resettable for CSR49_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
