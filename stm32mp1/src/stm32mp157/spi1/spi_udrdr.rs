#[doc = "Register `SPI_UDRDR` reader"]
pub struct R(crate::R<SPI_UDRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_UDRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_UDRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_UDRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_UDRDR` writer"]
pub struct W(crate::W<SPI_UDRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_UDRDR_SPEC>;
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
impl From<crate::W<SPI_UDRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_UDRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDRDR` reader - UDRDR"]
pub type UDRDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDRDR` writer - UDRDR"]
pub type UDRDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_UDRDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    #[must_use]
    pub fn udrdr(&mut self) -> UDRDR_W<0> {
        UDRDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI underrun data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_udrdr](index.html) module"]
pub struct SPI_UDRDR_SPEC;
impl crate::RegisterSpec for SPI_UDRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_udrdr::R](R) reader structure"]
impl crate::Readable for SPI_UDRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_udrdr::W](W) writer structure"]
impl crate::Writable for SPI_UDRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_UDRDR to value 0"]
impl crate::Resettable for SPI_UDRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
