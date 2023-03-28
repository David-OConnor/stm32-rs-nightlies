#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - EXTI 8 configuration bits"]
pub type EXTI8_R = crate::FieldReader<u8, EXTI8_A>;
#[doc = "EXTI 8 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: PA8 pin"]
    Pa8 = 0,
    #[doc = "1: PB8 pin"]
    Pb8 = 1,
    #[doc = "2: PC8 pin"]
    Pc8 = 2,
    #[doc = "3: PD8 pin"]
    Pd8 = 3,
    #[doc = "4: PE8 pin"]
    Pe8 = 4,
    #[doc = "5: PF8 pin"]
    Pf8 = 5,
    #[doc = "6: PG8 pin"]
    Pg8 = 6,
    #[doc = "7: PH8 pin"]
    Ph8 = 7,
    #[doc = "8: PI8 pin"]
    Pi8 = 8,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
impl EXTI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::Pa8),
            1 => Some(EXTI8_A::Pb8),
            2 => Some(EXTI8_A::Pc8),
            3 => Some(EXTI8_A::Pd8),
            4 => Some(EXTI8_A::Pe8),
            5 => Some(EXTI8_A::Pf8),
            6 => Some(EXTI8_A::Pg8),
            7 => Some(EXTI8_A::Ph8),
            8 => Some(EXTI8_A::Pi8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8_A::Pa8
    }
    #[doc = "Checks if the value of the field is `Pb8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8_A::Pb8
    }
    #[doc = "Checks if the value of the field is `Pc8`"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == EXTI8_A::Pc8
    }
    #[doc = "Checks if the value of the field is `Pd8`"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == EXTI8_A::Pd8
    }
    #[doc = "Checks if the value of the field is `Pe8`"]
    #[inline(always)]
    pub fn is_pe8(&self) -> bool {
        *self == EXTI8_A::Pe8
    }
    #[doc = "Checks if the value of the field is `Pf8`"]
    #[inline(always)]
    pub fn is_pf8(&self) -> bool {
        *self == EXTI8_A::Pf8
    }
    #[doc = "Checks if the value of the field is `Pg8`"]
    #[inline(always)]
    pub fn is_pg8(&self) -> bool {
        *self == EXTI8_A::Pg8
    }
    #[doc = "Checks if the value of the field is `Ph8`"]
    #[inline(always)]
    pub fn is_ph8(&self) -> bool {
        *self == EXTI8_A::Ph8
    }
    #[doc = "Checks if the value of the field is `Pi8`"]
    #[inline(always)]
    pub fn is_pi8(&self) -> bool {
        *self == EXTI8_A::Pi8
    }
}
#[doc = "Field `EXTI8` writer - EXTI 8 configuration bits"]
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI8_A, 4, O>;
impl<'a, const O: u8> EXTI8_W<'a, O> {
    #[doc = "PA8 pin"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pa8)
    }
    #[doc = "PB8 pin"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pb8)
    }
    #[doc = "PC8 pin"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pc8)
    }
    #[doc = "PD8 pin"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pd8)
    }
    #[doc = "PE8 pin"]
    #[inline(always)]
    pub fn pe8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pe8)
    }
    #[doc = "PF8 pin"]
    #[inline(always)]
    pub fn pf8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pf8)
    }
    #[doc = "PG8 pin"]
    #[inline(always)]
    pub fn pg8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pg8)
    }
    #[doc = "PH8 pin"]
    #[inline(always)]
    pub fn ph8(self) -> &'a mut W {
        self.variant(EXTI8_A::Ph8)
    }
    #[doc = "PI8 pin"]
    #[inline(always)]
    pub fn pi8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pi8)
    }
}
#[doc = "Field `EXTI9` reader - EXTI 9 configuration bits"]
pub type EXTI9_R = crate::FieldReader<u8, EXTI9_A>;
#[doc = "EXTI 9 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: PA9 pin"]
    Pa9 = 0,
    #[doc = "1: PB9 pin"]
    Pb9 = 1,
    #[doc = "2: PC9 pin"]
    Pc9 = 2,
    #[doc = "3: PD9 pin"]
    Pd9 = 3,
    #[doc = "4: PE9 pin"]
    Pe9 = 4,
    #[doc = "5: PF9 pin"]
    Pf9 = 5,
    #[doc = "6: PG9 pin"]
    Pg9 = 6,
    #[doc = "7: PH9 pin"]
    Ph9 = 7,
    #[doc = "8: PI9 pin"]
    Pi9 = 8,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
impl EXTI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::Pa9),
            1 => Some(EXTI9_A::Pb9),
            2 => Some(EXTI9_A::Pc9),
            3 => Some(EXTI9_A::Pd9),
            4 => Some(EXTI9_A::Pe9),
            5 => Some(EXTI9_A::Pf9),
            6 => Some(EXTI9_A::Pg9),
            7 => Some(EXTI9_A::Ph9),
            8 => Some(EXTI9_A::Pi9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9_A::Pa9
    }
    #[doc = "Checks if the value of the field is `Pb9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9_A::Pb9
    }
    #[doc = "Checks if the value of the field is `Pc9`"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == EXTI9_A::Pc9
    }
    #[doc = "Checks if the value of the field is `Pd9`"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == EXTI9_A::Pd9
    }
    #[doc = "Checks if the value of the field is `Pe9`"]
    #[inline(always)]
    pub fn is_pe9(&self) -> bool {
        *self == EXTI9_A::Pe9
    }
    #[doc = "Checks if the value of the field is `Pf9`"]
    #[inline(always)]
    pub fn is_pf9(&self) -> bool {
        *self == EXTI9_A::Pf9
    }
    #[doc = "Checks if the value of the field is `Pg9`"]
    #[inline(always)]
    pub fn is_pg9(&self) -> bool {
        *self == EXTI9_A::Pg9
    }
    #[doc = "Checks if the value of the field is `Ph9`"]
    #[inline(always)]
    pub fn is_ph9(&self) -> bool {
        *self == EXTI9_A::Ph9
    }
    #[doc = "Checks if the value of the field is `Pi9`"]
    #[inline(always)]
    pub fn is_pi9(&self) -> bool {
        *self == EXTI9_A::Pi9
    }
}
#[doc = "Field `EXTI9` writer - EXTI 9 configuration bits"]
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI9_A, 4, O>;
impl<'a, const O: u8> EXTI9_W<'a, O> {
    #[doc = "PA9 pin"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pa9)
    }
    #[doc = "PB9 pin"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pb9)
    }
    #[doc = "PC9 pin"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pc9)
    }
    #[doc = "PD9 pin"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pd9)
    }
    #[doc = "PE9 pin"]
    #[inline(always)]
    pub fn pe9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pe9)
    }
    #[doc = "PF9 pin"]
    #[inline(always)]
    pub fn pf9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pf9)
    }
    #[doc = "PG9 pin"]
    #[inline(always)]
    pub fn pg9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pg9)
    }
    #[doc = "PH9 pin"]
    #[inline(always)]
    pub fn ph9(self) -> &'a mut W {
        self.variant(EXTI9_A::Ph9)
    }
    #[doc = "PI9 pin"]
    #[inline(always)]
    pub fn pi9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pi9)
    }
}
#[doc = "Field `EXTI10` reader - EXTI 10 configuration bits"]
pub type EXTI10_R = crate::FieldReader<u8, EXTI10_A>;
#[doc = "EXTI 10 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: PA10 pin"]
    Pa10 = 0,
    #[doc = "1: PB10 pin"]
    Pb10 = 1,
    #[doc = "2: PC10 pin"]
    Pc10 = 2,
    #[doc = "3: PD10 pin"]
    Pd10 = 3,
    #[doc = "4: PE10 pin"]
    Pe10 = 4,
    #[doc = "5: PF10 pin"]
    Pf10 = 5,
    #[doc = "6: PG10 pin"]
    Pg10 = 6,
    #[doc = "7: PH10 pin"]
    Ph10 = 7,
    #[doc = "8: PI10 pin"]
    Pi10 = 8,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
impl EXTI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::Pa10),
            1 => Some(EXTI10_A::Pb10),
            2 => Some(EXTI10_A::Pc10),
            3 => Some(EXTI10_A::Pd10),
            4 => Some(EXTI10_A::Pe10),
            5 => Some(EXTI10_A::Pf10),
            6 => Some(EXTI10_A::Pg10),
            7 => Some(EXTI10_A::Ph10),
            8 => Some(EXTI10_A::Pi10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10_A::Pa10
    }
    #[doc = "Checks if the value of the field is `Pb10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10_A::Pb10
    }
    #[doc = "Checks if the value of the field is `Pc10`"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == EXTI10_A::Pc10
    }
    #[doc = "Checks if the value of the field is `Pd10`"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == EXTI10_A::Pd10
    }
    #[doc = "Checks if the value of the field is `Pe10`"]
    #[inline(always)]
    pub fn is_pe10(&self) -> bool {
        *self == EXTI10_A::Pe10
    }
    #[doc = "Checks if the value of the field is `Pf10`"]
    #[inline(always)]
    pub fn is_pf10(&self) -> bool {
        *self == EXTI10_A::Pf10
    }
    #[doc = "Checks if the value of the field is `Pg10`"]
    #[inline(always)]
    pub fn is_pg10(&self) -> bool {
        *self == EXTI10_A::Pg10
    }
    #[doc = "Checks if the value of the field is `Ph10`"]
    #[inline(always)]
    pub fn is_ph10(&self) -> bool {
        *self == EXTI10_A::Ph10
    }
    #[doc = "Checks if the value of the field is `Pi10`"]
    #[inline(always)]
    pub fn is_pi10(&self) -> bool {
        *self == EXTI10_A::Pi10
    }
}
#[doc = "Field `EXTI10` writer - EXTI 10 configuration bits"]
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI10_A, 4, O>;
impl<'a, const O: u8> EXTI10_W<'a, O> {
    #[doc = "PA10 pin"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pa10)
    }
    #[doc = "PB10 pin"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pb10)
    }
    #[doc = "PC10 pin"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pc10)
    }
    #[doc = "PD10 pin"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pd10)
    }
    #[doc = "PE10 pin"]
    #[inline(always)]
    pub fn pe10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pe10)
    }
    #[doc = "PF10 pin"]
    #[inline(always)]
    pub fn pf10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pf10)
    }
    #[doc = "PG10 pin"]
    #[inline(always)]
    pub fn pg10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pg10)
    }
    #[doc = "PH10 pin"]
    #[inline(always)]
    pub fn ph10(self) -> &'a mut W {
        self.variant(EXTI10_A::Ph10)
    }
    #[doc = "PI10 pin"]
    #[inline(always)]
    pub fn pi10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pi10)
    }
}
#[doc = "Field `EXTI11` reader - EXTI 11 configuration bits"]
pub type EXTI11_R = crate::FieldReader<u8, EXTI11_A>;
#[doc = "EXTI 11 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: PA11 pin"]
    Pa11 = 0,
    #[doc = "1: PB11 pin"]
    Pb11 = 1,
    #[doc = "2: PC11 pin"]
    Pc11 = 2,
    #[doc = "3: PD11 pin"]
    Pd11 = 3,
    #[doc = "4: PE11 pin"]
    Pe11 = 4,
    #[doc = "5: PF11 pin"]
    Pf11 = 5,
    #[doc = "6: PG11 pin"]
    Pg11 = 6,
    #[doc = "7: PH11 pin"]
    Ph11 = 7,
    #[doc = "8: PI11 pin"]
    Pi11 = 8,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
impl EXTI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::Pa11),
            1 => Some(EXTI11_A::Pb11),
            2 => Some(EXTI11_A::Pc11),
            3 => Some(EXTI11_A::Pd11),
            4 => Some(EXTI11_A::Pe11),
            5 => Some(EXTI11_A::Pf11),
            6 => Some(EXTI11_A::Pg11),
            7 => Some(EXTI11_A::Ph11),
            8 => Some(EXTI11_A::Pi11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11_A::Pa11
    }
    #[doc = "Checks if the value of the field is `Pb11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11_A::Pb11
    }
    #[doc = "Checks if the value of the field is `Pc11`"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == EXTI11_A::Pc11
    }
    #[doc = "Checks if the value of the field is `Pd11`"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == EXTI11_A::Pd11
    }
    #[doc = "Checks if the value of the field is `Pe11`"]
    #[inline(always)]
    pub fn is_pe11(&self) -> bool {
        *self == EXTI11_A::Pe11
    }
    #[doc = "Checks if the value of the field is `Pf11`"]
    #[inline(always)]
    pub fn is_pf11(&self) -> bool {
        *self == EXTI11_A::Pf11
    }
    #[doc = "Checks if the value of the field is `Pg11`"]
    #[inline(always)]
    pub fn is_pg11(&self) -> bool {
        *self == EXTI11_A::Pg11
    }
    #[doc = "Checks if the value of the field is `Ph11`"]
    #[inline(always)]
    pub fn is_ph11(&self) -> bool {
        *self == EXTI11_A::Ph11
    }
    #[doc = "Checks if the value of the field is `Pi11`"]
    #[inline(always)]
    pub fn is_pi11(&self) -> bool {
        *self == EXTI11_A::Pi11
    }
}
#[doc = "Field `EXTI11` writer - EXTI 11 configuration bits"]
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI11_A, 4, O>;
impl<'a, const O: u8> EXTI11_W<'a, O> {
    #[doc = "PA11 pin"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pa11)
    }
    #[doc = "PB11 pin"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pb11)
    }
    #[doc = "PC11 pin"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pc11)
    }
    #[doc = "PD11 pin"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pd11)
    }
    #[doc = "PE11 pin"]
    #[inline(always)]
    pub fn pe11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pe11)
    }
    #[doc = "PF11 pin"]
    #[inline(always)]
    pub fn pf11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pf11)
    }
    #[doc = "PG11 pin"]
    #[inline(always)]
    pub fn pg11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pg11)
    }
    #[doc = "PH11 pin"]
    #[inline(always)]
    pub fn ph11(self) -> &'a mut W {
        self.variant(EXTI11_A::Ph11)
    }
    #[doc = "PI11 pin"]
    #[inline(always)]
    pub fn pi11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pi11)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<4> {
        EXTI9_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<8> {
        EXTI10_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<12> {
        EXTI11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
