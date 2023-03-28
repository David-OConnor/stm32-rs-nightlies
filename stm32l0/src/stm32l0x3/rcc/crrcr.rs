#[doc = "Register `CRRCR` reader"]
pub struct R(crate::R<CRRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRRCR` writer"]
pub struct W(crate::W<CRRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRRCR_SPEC>;
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
impl From<crate::W<CRRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI48ON` reader - 48MHz HSI clock enable bit"]
pub type HSI48ON_R = crate::BitReader<bool>;
#[doc = "Field `HSI48ON` writer - 48MHz HSI clock enable bit"]
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRRCR_SPEC, bool, O>;
#[doc = "Field `HSI48RDY` reader - 48MHz HSI clock ready flag"]
pub type HSI48RDY_R = crate::BitReader<bool>;
#[doc = "Field `HSI48DIV6EN` reader - 48 MHz HSI clock divided by 6 output enable"]
pub type HSI48DIV6EN_R = crate::BitReader<bool>;
#[doc = "Field `HSI48DIV6EN` writer - 48 MHz HSI clock divided by 6 output enable"]
pub type HSI48DIV6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRRCR_SPEC, bool, O>;
#[doc = "Field `HSI48CAL` reader - 48 MHz HSI clock calibration"]
pub type HSI48CAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 48MHz HSI clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    pub fn hsi48div6en(&self) -> HSI48DIV6EN_R {
        HSI48DIV6EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 48 MHz HSI clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<0> {
        HSI48ON_W::new(self)
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48div6en(&mut self) -> HSI48DIV6EN_W<2> {
        HSI48DIV6EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock recovery RC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crrcr](index.html) module"]
pub struct CRRCR_SPEC;
impl crate::RegisterSpec for CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crrcr::R](R) reader structure"]
impl crate::Readable for CRRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crrcr::W](W) writer structure"]
impl crate::Writable for CRRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
