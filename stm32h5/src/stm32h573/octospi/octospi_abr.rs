#[doc = "Register `OCTOSPI_ABR` reader"]
pub struct R(crate::R<OCTOSPI_ABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_ABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_ABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_ABR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_ABR` writer"]
pub struct W(crate::W<OCTOSPI_ABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_ABR_SPEC>;
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
impl From<crate::W<OCTOSPI_ABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_ABR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALTERNATE` reader - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
pub type ALTERNATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALTERNATE` writer - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
pub type ALTERNATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OCTOSPI_ABR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<0> {
        ALTERNATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI alternate bytes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_abr](index.html) module"]
pub struct OCTOSPI_ABR_SPEC;
impl crate::RegisterSpec for OCTOSPI_ABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_abr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_ABR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_abr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_ABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_ABR to value 0"]
impl crate::Resettable for OCTOSPI_ABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}