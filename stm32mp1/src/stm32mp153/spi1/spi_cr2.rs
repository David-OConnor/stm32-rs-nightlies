#[doc = "Register `SPI_CR2` reader"]
pub struct R(crate::R<SPI_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CR2` writer"]
pub struct W(crate::W<SPI_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CR2_SPEC>;
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
impl From<crate::W<SPI_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSIZE` reader - TSIZE"]
pub type TSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSIZE` writer - TSIZE"]
pub type TSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CR2_SPEC, u16, u16, 16, O>;
#[doc = "Field `TSER` reader - TSER"]
pub type TSER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSER` writer - TSER"]
pub type TSER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TSIZE"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TSER"]
    #[inline(always)]
    pub fn tser(&self) -> TSER_R {
        TSER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn tsize(&mut self) -> TSIZE_W<0> {
        TSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - TSER"]
    #[inline(always)]
    #[must_use]
    pub fn tser(&mut self) -> TSER_W<16> {
        TSER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr2](index.html) module"]
pub struct SPI_CR2_SPEC;
impl crate::RegisterSpec for SPI_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cr2::R](R) reader structure"]
impl crate::Readable for SPI_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cr2::W](W) writer structure"]
impl crate::Writable for SPI_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CR2 to value 0"]
impl crate::Resettable for SPI_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
