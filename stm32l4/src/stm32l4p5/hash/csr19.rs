#[doc = "Register `CSR19` reader"]
pub struct R(crate::R<CSR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR19` writer"]
pub struct W(crate::W<CSR19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR19_SPEC>;
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
impl From<crate::W<CSR19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR19` reader - CSR19"]
pub type CSR19_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSR19` writer - CSR19"]
pub type CSR19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR19_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    pub fn csr19(&self) -> CSR19_R {
        CSR19_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    #[must_use]
    pub fn csr19(&mut self) -> CSR19_W<0> {
        CSR19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr19](index.html) module"]
pub struct CSR19_SPEC;
impl crate::RegisterSpec for CSR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr19::R](R) reader structure"]
impl crate::Readable for CSR19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr19::W](W) writer structure"]
impl crate::Writable for CSR19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR19 to value 0"]
impl crate::Resettable for CSR19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
