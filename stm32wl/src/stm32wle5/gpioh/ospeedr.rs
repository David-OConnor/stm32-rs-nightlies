#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_R = crate::FieldReader<u8, OSPEEDR3_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEEDR3_A {
    #[doc = "0: Low speed"]
    LowSpeed = 0,
    #[doc = "1: Medium speed"]
    MediumSpeed = 1,
    #[doc = "2: High speed"]
    HighSpeed = 2,
    #[doc = "3: Very high speed"]
    VeryHighSpeed = 3,
}
impl From<OSPEEDR3_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR3_A) -> Self {
        variant as _
    }
}
impl OSPEEDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPEEDR3_A {
        match self.bits {
            0 => OSPEEDR3_A::LowSpeed,
            1 => OSPEEDR3_A::MediumSpeed,
            2 => OSPEEDR3_A::HighSpeed,
            3 => OSPEEDR3_A::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR3_A::LowSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR3_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR3_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `VeryHighSpeed`"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEEDR3_A::VeryHighSpeed
    }
}
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSPEEDR_SPEC, u8, OSPEEDR3_A, 2, O>;
impl<'a, const O: u8> OSPEEDR3_W<'a, O> {
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::LowSpeed)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::MediumSpeed)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::HighSpeed)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::VeryHighSpeed)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<6> {
        OSPEEDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
