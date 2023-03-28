#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI16CAL` reader - nternal high speed clock calibration"]
pub type HSI16CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI16TRIM` reader - High speed internal clock trimming"]
pub type HSI16TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI16TRIM` writer - High speed internal clock trimming"]
pub type HSI16TRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<u8, MSIRANGE_A>;
#[doc = "MSI clock ranges\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 65.536 kHz"]
    Range0 = 0,
    #[doc = "1: range 1 around 131.072 kHz"]
    Range1 = 1,
    #[doc = "2: range 2 around 262.144 kHz"]
    Range2 = 2,
    #[doc = "3: range 3 around 524.288 kHz"]
    Range3 = 3,
    #[doc = "4: range 4 around 1.048 MHz"]
    Range4 = 4,
    #[doc = "5: range 5 around 2.097 MHz (reset value)"]
    Range5 = 5,
    #[doc = "6: range 6 around 4.194 MHz"]
    Range6 = 6,
    #[doc = "7: not allowed"]
    Range7 = 7,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRANGE_A {
        match self.bits {
            0 => MSIRANGE_A::Range0,
            1 => MSIRANGE_A::Range1,
            2 => MSIRANGE_A::Range2,
            3 => MSIRANGE_A::Range3,
            4 => MSIRANGE_A::Range4,
            5 => MSIRANGE_A::Range5,
            6 => MSIRANGE_A::Range6,
            7 => MSIRANGE_A::Range7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Range0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == MSIRANGE_A::Range0
    }
    #[doc = "Checks if the value of the field is `Range1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == MSIRANGE_A::Range1
    }
    #[doc = "Checks if the value of the field is `Range2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == MSIRANGE_A::Range2
    }
    #[doc = "Checks if the value of the field is `Range3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == MSIRANGE_A::Range3
    }
    #[doc = "Checks if the value of the field is `Range4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == MSIRANGE_A::Range4
    }
    #[doc = "Checks if the value of the field is `Range5`"]
    #[inline(always)]
    pub fn is_range5(&self) -> bool {
        *self == MSIRANGE_A::Range5
    }
    #[doc = "Checks if the value of the field is `Range6`"]
    #[inline(always)]
    pub fn is_range6(&self) -> bool {
        *self == MSIRANGE_A::Range6
    }
    #[doc = "Checks if the value of the field is `Range7`"]
    #[inline(always)]
    pub fn is_range7(&self) -> bool {
        *self == MSIRANGE_A::Range7
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ICSCR_SPEC, u8, MSIRANGE_A, 3, O>;
impl<'a, const O: u8> MSIRANGE_W<'a, O> {
    #[doc = "range 0 around 65.536 kHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range0)
    }
    #[doc = "range 1 around 131.072 kHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range1)
    }
    #[doc = "range 2 around 262.144 kHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range2)
    }
    #[doc = "range 3 around 524.288 kHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range3)
    }
    #[doc = "range 4 around 1.048 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range4)
    }
    #[doc = "range 5 around 2.097 MHz (reset value)"]
    #[inline(always)]
    pub fn range5(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range5)
    }
    #[doc = "range 6 around 4.194 MHz"]
    #[inline(always)]
    pub fn range6(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range6)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn range7(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range7)
    }
}
#[doc = "Field `MSICAL` reader - MSI clock calibration"]
pub type MSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSITRIM` reader - MSI clock trimming"]
pub type MSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSITRIM` writer - MSI clock trimming"]
pub type MSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsi16cal(&self) -> HSI16CAL_R {
        HSI16CAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&self) -> HSI16TRIM_R {
        HSI16TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16trim(&mut self) -> HSI16TRIM_W<8> {
        HSI16TRIM_W::new(self)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<13> {
        MSIRANGE_W::new(self)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn msitrim(&mut self) -> MSITRIM_W<24> {
        MSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0xb000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb000;
}
