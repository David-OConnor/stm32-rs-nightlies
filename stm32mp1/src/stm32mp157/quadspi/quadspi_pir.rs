#[doc = "Register `QUADSPI_PIR` reader"]
pub struct R(crate::R<QUADSPI_PIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_PIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_PIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_PIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_PIR` writer"]
pub struct W(crate::W<QUADSPI_PIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_PIR_SPEC>;
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
impl From<crate::W<QUADSPI_PIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_PIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERVAL` reader - INTERVAL"]
pub type INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTERVAL` writer - INTERVAL"]
pub type INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUADSPI_PIR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INTERVAL"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<0> {
        INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI polling interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_pir](index.html) module"]
pub struct QUADSPI_PIR_SPEC;
impl crate::RegisterSpec for QUADSPI_PIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_pir::R](R) reader structure"]
impl crate::Readable for QUADSPI_PIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_pir::W](W) writer structure"]
impl crate::Writable for QUADSPI_PIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QUADSPI_PIR to value 0"]
impl crate::Resettable for QUADSPI_PIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
