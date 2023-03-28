#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI14ON` reader - HSI14 clock enable"]
pub type HSI14ON_R = crate::BitReader<HSI14ON_A>;
#[doc = "HSI14 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14ON_A {
    #[doc = "0: HSI14 oscillator off"]
    Off = 0,
    #[doc = "1: HSI14 oscillator on"]
    On = 1,
}
impl From<HSI14ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14ON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI14ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14ON_A {
        match self.bits {
            false => HSI14ON_A::Off,
            true => HSI14ON_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSI14ON_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSI14ON_A::On
    }
}
#[doc = "Field `HSI14ON` writer - HSI14 clock enable"]
pub type HSI14ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, HSI14ON_A, O>;
impl<'a, const O: u8> HSI14ON_W<'a, O> {
    #[doc = "HSI14 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSI14ON_A::Off)
    }
    #[doc = "HSI14 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSI14ON_A::On)
    }
}
#[doc = "Field `HSI14RDY` reader - HR14 clock ready flag"]
pub type HSI14RDY_R = crate::BitReader<HSI14RDYR_A>;
#[doc = "HR14 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14RDYR_A {
    #[doc = "0: HSI14 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI14 oscillator ready"]
    Ready = 1,
}
impl From<HSI14RDYR_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14RDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI14RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14RDYR_A {
        match self.bits {
            false => HSI14RDYR_A::NotReady,
            true => HSI14RDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI14RDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI14RDYR_A::Ready
    }
}
#[doc = "Field `HSI14DIS` reader - HSI14 clock request from ADC disable"]
pub type HSI14DIS_R = crate::BitReader<HSI14DIS_A>;
#[doc = "HSI14 clock request from ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14DIS_A {
    #[doc = "0: ADC can turn on the HSI14 oscillator"]
    Allow = 0,
    #[doc = "1: ADC can not turn on the HSI14 oscillator"]
    Disallow = 1,
}
impl From<HSI14DIS_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI14DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14DIS_A {
        match self.bits {
            false => HSI14DIS_A::Allow,
            true => HSI14DIS_A::Disallow,
        }
    }
    #[doc = "Checks if the value of the field is `Allow`"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == HSI14DIS_A::Allow
    }
    #[doc = "Checks if the value of the field is `Disallow`"]
    #[inline(always)]
    pub fn is_disallow(&self) -> bool {
        *self == HSI14DIS_A::Disallow
    }
}
#[doc = "Field `HSI14DIS` writer - HSI14 clock request from ADC disable"]
pub type HSI14DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, HSI14DIS_A, O>;
impl<'a, const O: u8> HSI14DIS_W<'a, O> {
    #[doc = "ADC can turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(HSI14DIS_A::Allow)
    }
    #[doc = "ADC can not turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut W {
        self.variant(HSI14DIS_A::Disallow)
    }
}
#[doc = "Field `HSI14TRIM` reader - HSI14 clock trimming"]
pub type HSI14TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI14TRIM` writer - HSI14 clock trimming"]
pub type HSI14TRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `HSI14CAL` reader - HSI14 clock calibration"]
pub type HSI14CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI48ON` reader - HSI48 clock enable"]
pub type HSI48ON_R = crate::BitReader<HSI48ON_A>;
#[doc = "HSI48 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON_A {
    #[doc = "0: HSI48 oscillator off"]
    Off = 0,
    #[doc = "1: HSI48 oscillator on"]
    On = 1,
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
            false => HSI48ON_A::Off,
            true => HSI48ON_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSI48ON_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSI48ON_A::On
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable"]
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, HSI48ON_A, O>;
impl<'a, const O: u8> HSI48ON_W<'a, O> {
    #[doc = "HSI48 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSI48ON_A::Off)
    }
    #[doc = "HSI48 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSI48ON_A::On)
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag"]
pub type HSI48RDY_R = crate::BitReader<HSI48RDYR_A>;
#[doc = "HSI48 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYR_A {
    #[doc = "0: HSI48 oscillator ready"]
    NotReady = 0,
    #[doc = "1: HSI48 oscillator ready"]
    Ready = 1,
}
impl From<HSI48RDYR_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYR_A {
        match self.bits {
            false => HSI48RDYR_A::NotReady,
            true => HSI48RDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDYR_A::Ready
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 factory clock calibration"]
pub type HSI48CAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&self) -> HSI14ON_R {
        HSI14ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HR14 clock ready flag"]
    #[inline(always)]
    pub fn hsi14rdy(&self) -> HSI14RDY_R {
        HSI14RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&self) -> HSI14DIS_R {
        HSI14DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&self) -> HSI14TRIM_R {
        HSI14TRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - HSI14 clock calibration"]
    #[inline(always)]
    pub fn hsi14cal(&self) -> HSI14CAL_R {
        HSI14CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - HSI48 factory clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14on(&mut self) -> HSI14ON_W<0> {
        HSI14ON_W::new(self)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14dis(&mut self) -> HSI14DIS_W<2> {
        HSI14DIS_W::new(self)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14trim(&mut self) -> HSI14TRIM_W<3> {
        HSI14TRIM_W::new(self)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<16> {
        HSI48ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0x80"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
