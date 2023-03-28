#[doc = "Register `HASH_CSR19` reader"]
pub struct R(crate::R<HASH_CSR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR19` writer"]
pub struct W(crate::W<HASH_CSR19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR19_SPEC>;
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
impl From<crate::W<HASH_CSR19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSx` reader - Context swap 19 Refer to introduction."]
pub type CSX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSx` writer - Context swap 19 Refer to introduction."]
pub type CSX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR19_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Context swap 19 Refer to introduction."]
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap 19 Refer to introduction."]
    #[inline(always)]
    #[must_use]
    pub fn csx(&mut self) -> CSX_W<0> {
        CSX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr19](index.html) module"]
pub struct HASH_CSR19_SPEC;
impl crate::RegisterSpec for HASH_CSR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr19::R](R) reader structure"]
impl crate::Readable for HASH_CSR19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr19::W](W) writer structure"]
impl crate::Writable for HASH_CSR19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_CSR19 to value 0x0022_0002"]
impl crate::Resettable for HASH_CSR19_SPEC {
    const RESET_VALUE: Self::Ux = 0x0022_0002;
}
