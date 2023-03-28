#[doc = "Register `QUADSPI_LPTR` reader"]
pub struct R(crate::R<QUADSPI_LPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_LPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_LPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_LPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_LPTR` writer"]
pub struct W(crate::W<QUADSPI_LPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_LPTR_SPEC>;
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
impl From<crate::W<QUADSPI_LPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_LPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - TIMEOUT"]
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUT` writer - TIMEOUT"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUADSPI_LPTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_lptr](index.html) module"]
pub struct QUADSPI_LPTR_SPEC;
impl crate::RegisterSpec for QUADSPI_LPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_lptr::R](R) reader structure"]
impl crate::Readable for QUADSPI_LPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_lptr::W](W) writer structure"]
impl crate::Writable for QUADSPI_LPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QUADSPI_LPTR to value 0"]
impl crate::Resettable for QUADSPI_LPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
