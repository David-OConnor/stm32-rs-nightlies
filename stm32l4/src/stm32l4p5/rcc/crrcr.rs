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
#[doc = "Field `HSI48ON` reader - HSI48 clock enable"]
pub type HSI48ON_R = crate::BitReader<HSI48ON_A>;
#[doc = "HSI48 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON_A {
    #[doc = "0: HSI48 oscillator OFF"]
    Disabled = 0,
    #[doc = "1: HSI48 oscillator ON"]
    Enabled = 1,
}
impl From<HSI48ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48ON_A {
        match self.bits {
            false => HSI48ON_A::Disabled,
            true => HSI48ON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48ON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48ON_A::Enabled
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable"]
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRRCR_SPEC, HSI48ON_A, O>;
impl<'a, const O: u8> HSI48ON_W<'a, O> {
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI48ON_A::Disabled)
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI48ON_A::Enabled)
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag"]
pub type HSI48RDY_R = crate::BitReader<HSI48RDY_A>;
#[doc = "HSI48 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDY_A {
    #[doc = "0: HSI48 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI48 oscillator ready"]
    Ready = 1,
}
impl From<HSI48RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDY_A {
        match self.bits {
            false => HSI48RDY_A::NotReady,
            true => HSI48RDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDY_A::Ready
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration"]
pub type HSI48CAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:15 - HSI48 clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HSI48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<0> {
        HSI48ON_W::new(self)
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
