#[doc = "Register `OCTOSPI_DCR4` reader"]
pub struct R(crate::R<OCTOSPI_DCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_DCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_DCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_DCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_DCR4` writer"]
pub struct W(crate::W<OCTOSPI_DCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_DCR4_SPEC>;
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
impl From<crate::W<OCTOSPI_DCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_DCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFRESH` reader - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
pub type REFRESH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REFRESH` writer - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
pub type REFRESH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OCTOSPI_DCR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<0> {
        REFRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI device configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_dcr4](index.html) module"]
pub struct OCTOSPI_DCR4_SPEC;
impl crate::RegisterSpec for OCTOSPI_DCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_dcr4::R](R) reader structure"]
impl crate::Readable for OCTOSPI_DCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_dcr4::W](W) writer structure"]
impl crate::Writable for OCTOSPI_DCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_DCR4 to value 0"]
impl crate::Resettable for OCTOSPI_DCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
