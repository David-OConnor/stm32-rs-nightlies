#[doc = "Register `HSEM_C1ICR` reader"]
pub struct R(crate::R<HSEM_C1ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_C1ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_C1ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_C1ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEM_C1ICR` writer"]
pub struct W(crate::W<HSEM_C1ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_C1ICR_SPEC>;
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
impl From<crate::W<HSEM_C1ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_C1ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC` reader - ISC"]
pub type ISC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISC` writer - ISC"]
pub type ISC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_C1ICR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    #[must_use]
    pub fn isc(&mut self) -> ISC_W<0> {
        ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM i1terrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_c1icr](index.html) module"]
pub struct HSEM_C1ICR_SPEC;
impl crate::RegisterSpec for HSEM_C1ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_c1icr::R](R) reader structure"]
impl crate::Readable for HSEM_C1ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsem_c1icr::W](W) writer structure"]
impl crate::Writable for HSEM_C1ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_C1ICR to value 0"]
impl crate::Resettable for HSEM_C1ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
