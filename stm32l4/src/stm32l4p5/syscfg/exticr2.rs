#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type EXTI4_R = crate::FieldReader<u8, EXTI4_A>;
#[doc = "EXTI 4 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4_A {
    #[doc = "0: PA4 pin"]
    Pa4 = 0,
    #[doc = "1: PB4 pin"]
    Pb4 = 1,
    #[doc = "2: PC4 pin"]
    Pc4 = 2,
    #[doc = "3: PD4 pin"]
    Pd4 = 3,
    #[doc = "4: PE4 pin"]
    Pe4 = 4,
    #[doc = "5: PF4 pin"]
    Pf4 = 5,
    #[doc = "6: PG4 pin"]
    Pg4 = 6,
    #[doc = "7: PH4 pin"]
    Ph4 = 7,
    #[doc = "8: PI4 pin"]
    Pi4 = 8,
}
impl From<EXTI4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_A) -> Self {
        variant as _
    }
}
impl EXTI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI4_A> {
        match self.bits {
            0 => Some(EXTI4_A::Pa4),
            1 => Some(EXTI4_A::Pb4),
            2 => Some(EXTI4_A::Pc4),
            3 => Some(EXTI4_A::Pd4),
            4 => Some(EXTI4_A::Pe4),
            5 => Some(EXTI4_A::Pf4),
            6 => Some(EXTI4_A::Pg4),
            7 => Some(EXTI4_A::Ph4),
            8 => Some(EXTI4_A::Pi4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4_A::Pa4
    }
    #[doc = "Checks if the value of the field is `Pb4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4_A::Pb4
    }
    #[doc = "Checks if the value of the field is `Pc4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4_A::Pc4
    }
    #[doc = "Checks if the value of the field is `Pd4`"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == EXTI4_A::Pd4
    }
    #[doc = "Checks if the value of the field is `Pe4`"]
    #[inline(always)]
    pub fn is_pe4(&self) -> bool {
        *self == EXTI4_A::Pe4
    }
    #[doc = "Checks if the value of the field is `Pf4`"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4_A::Pf4
    }
    #[doc = "Checks if the value of the field is `Pg4`"]
    #[inline(always)]
    pub fn is_pg4(&self) -> bool {
        *self == EXTI4_A::Pg4
    }
    #[doc = "Checks if the value of the field is `Ph4`"]
    #[inline(always)]
    pub fn is_ph4(&self) -> bool {
        *self == EXTI4_A::Ph4
    }
    #[doc = "Checks if the value of the field is `Pi4`"]
    #[inline(always)]
    pub fn is_pi4(&self) -> bool {
        *self == EXTI4_A::Pi4
    }
}
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type EXTI4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI4_A, 4, O>;
impl<'a, const O: u8> EXTI4_W<'a, O> {
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pa4)
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pb4)
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pc4)
    }
    #[doc = "PD4 pin"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pd4)
    }
    #[doc = "PE4 pin"]
    #[inline(always)]
    pub fn pe4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pe4)
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pf4)
    }
    #[doc = "PG4 pin"]
    #[inline(always)]
    pub fn pg4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pg4)
    }
    #[doc = "PH4 pin"]
    #[inline(always)]
    pub fn ph4(self) -> &'a mut W {
        self.variant(EXTI4_A::Ph4)
    }
    #[doc = "PI4 pin"]
    #[inline(always)]
    pub fn pi4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pi4)
    }
}
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type EXTI5_R = crate::FieldReader<u8, EXTI5_A>;
#[doc = "EXTI 5 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5_A {
    #[doc = "0: PA5 pin"]
    Pa5 = 0,
    #[doc = "1: PB5 pin"]
    Pb5 = 1,
    #[doc = "2: PC5 pin"]
    Pc5 = 2,
    #[doc = "3: PD5 pin"]
    Pd5 = 3,
    #[doc = "4: PE5 pin"]
    Pe5 = 4,
    #[doc = "5: PF5 pin"]
    Pf5 = 5,
    #[doc = "6: PG5 pin"]
    Pg5 = 6,
    #[doc = "7: PH5 pin"]
    Ph5 = 7,
    #[doc = "8: PI5 pin"]
    Pi5 = 8,
}
impl From<EXTI5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_A) -> Self {
        variant as _
    }
}
impl EXTI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI5_A> {
        match self.bits {
            0 => Some(EXTI5_A::Pa5),
            1 => Some(EXTI5_A::Pb5),
            2 => Some(EXTI5_A::Pc5),
            3 => Some(EXTI5_A::Pd5),
            4 => Some(EXTI5_A::Pe5),
            5 => Some(EXTI5_A::Pf5),
            6 => Some(EXTI5_A::Pg5),
            7 => Some(EXTI5_A::Ph5),
            8 => Some(EXTI5_A::Pi5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5_A::Pa5
    }
    #[doc = "Checks if the value of the field is `Pb5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5_A::Pb5
    }
    #[doc = "Checks if the value of the field is `Pc5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5_A::Pc5
    }
    #[doc = "Checks if the value of the field is `Pd5`"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == EXTI5_A::Pd5
    }
    #[doc = "Checks if the value of the field is `Pe5`"]
    #[inline(always)]
    pub fn is_pe5(&self) -> bool {
        *self == EXTI5_A::Pe5
    }
    #[doc = "Checks if the value of the field is `Pf5`"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5_A::Pf5
    }
    #[doc = "Checks if the value of the field is `Pg5`"]
    #[inline(always)]
    pub fn is_pg5(&self) -> bool {
        *self == EXTI5_A::Pg5
    }
    #[doc = "Checks if the value of the field is `Ph5`"]
    #[inline(always)]
    pub fn is_ph5(&self) -> bool {
        *self == EXTI5_A::Ph5
    }
    #[doc = "Checks if the value of the field is `Pi5`"]
    #[inline(always)]
    pub fn is_pi5(&self) -> bool {
        *self == EXTI5_A::Pi5
    }
}
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type EXTI5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI5_A, 4, O>;
impl<'a, const O: u8> EXTI5_W<'a, O> {
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pa5)
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pb5)
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pc5)
    }
    #[doc = "PD5 pin"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pd5)
    }
    #[doc = "PE5 pin"]
    #[inline(always)]
    pub fn pe5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pe5)
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pf5)
    }
    #[doc = "PG5 pin"]
    #[inline(always)]
    pub fn pg5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pg5)
    }
    #[doc = "PH5 pin"]
    #[inline(always)]
    pub fn ph5(self) -> &'a mut W {
        self.variant(EXTI5_A::Ph5)
    }
    #[doc = "PI5 pin"]
    #[inline(always)]
    pub fn pi5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pi5)
    }
}
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type EXTI6_R = crate::FieldReader<u8, EXTI6_A>;
#[doc = "EXTI 6 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6_A {
    #[doc = "0: PA6 pin"]
    Pa6 = 0,
    #[doc = "1: PB6 pin"]
    Pb6 = 1,
    #[doc = "2: PC6 pin"]
    Pc6 = 2,
    #[doc = "3: PD6 pin"]
    Pd6 = 3,
    #[doc = "4: PE6 pin"]
    Pe6 = 4,
    #[doc = "5: PF6 pin"]
    Pf6 = 5,
    #[doc = "6: PG6 pin"]
    Pg6 = 6,
    #[doc = "7: PH6 pin"]
    Ph6 = 7,
    #[doc = "8: PI6 pin"]
    Pi6 = 8,
}
impl From<EXTI6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_A) -> Self {
        variant as _
    }
}
impl EXTI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI6_A> {
        match self.bits {
            0 => Some(EXTI6_A::Pa6),
            1 => Some(EXTI6_A::Pb6),
            2 => Some(EXTI6_A::Pc6),
            3 => Some(EXTI6_A::Pd6),
            4 => Some(EXTI6_A::Pe6),
            5 => Some(EXTI6_A::Pf6),
            6 => Some(EXTI6_A::Pg6),
            7 => Some(EXTI6_A::Ph6),
            8 => Some(EXTI6_A::Pi6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6_A::Pa6
    }
    #[doc = "Checks if the value of the field is `Pb6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6_A::Pb6
    }
    #[doc = "Checks if the value of the field is `Pc6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6_A::Pc6
    }
    #[doc = "Checks if the value of the field is `Pd6`"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == EXTI6_A::Pd6
    }
    #[doc = "Checks if the value of the field is `Pe6`"]
    #[inline(always)]
    pub fn is_pe6(&self) -> bool {
        *self == EXTI6_A::Pe6
    }
    #[doc = "Checks if the value of the field is `Pf6`"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6_A::Pf6
    }
    #[doc = "Checks if the value of the field is `Pg6`"]
    #[inline(always)]
    pub fn is_pg6(&self) -> bool {
        *self == EXTI6_A::Pg6
    }
    #[doc = "Checks if the value of the field is `Ph6`"]
    #[inline(always)]
    pub fn is_ph6(&self) -> bool {
        *self == EXTI6_A::Ph6
    }
    #[doc = "Checks if the value of the field is `Pi6`"]
    #[inline(always)]
    pub fn is_pi6(&self) -> bool {
        *self == EXTI6_A::Pi6
    }
}
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type EXTI6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI6_A, 4, O>;
impl<'a, const O: u8> EXTI6_W<'a, O> {
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pa6)
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pb6)
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pc6)
    }
    #[doc = "PD6 pin"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pd6)
    }
    #[doc = "PE6 pin"]
    #[inline(always)]
    pub fn pe6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pe6)
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pf6)
    }
    #[doc = "PG6 pin"]
    #[inline(always)]
    pub fn pg6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pg6)
    }
    #[doc = "PH6 pin"]
    #[inline(always)]
    pub fn ph6(self) -> &'a mut W {
        self.variant(EXTI6_A::Ph6)
    }
    #[doc = "PI6 pin"]
    #[inline(always)]
    pub fn pi6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pi6)
    }
}
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type EXTI7_R = crate::FieldReader<u8, EXTI7_A>;
#[doc = "EXTI 7 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7_A {
    #[doc = "0: PA7 pin"]
    Pa7 = 0,
    #[doc = "1: PB7 pin"]
    Pb7 = 1,
    #[doc = "2: PC7 pin"]
    Pc7 = 2,
    #[doc = "3: PD7 pin"]
    Pd7 = 3,
    #[doc = "4: PE7 pin"]
    Pe7 = 4,
    #[doc = "5: PF7 pin"]
    Pf7 = 5,
    #[doc = "6: PG7 pin"]
    Pg7 = 6,
    #[doc = "7: PH7 pin"]
    Ph7 = 7,
    #[doc = "8: PI7 pin"]
    Pi7 = 8,
}
impl From<EXTI7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_A) -> Self {
        variant as _
    }
}
impl EXTI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI7_A> {
        match self.bits {
            0 => Some(EXTI7_A::Pa7),
            1 => Some(EXTI7_A::Pb7),
            2 => Some(EXTI7_A::Pc7),
            3 => Some(EXTI7_A::Pd7),
            4 => Some(EXTI7_A::Pe7),
            5 => Some(EXTI7_A::Pf7),
            6 => Some(EXTI7_A::Pg7),
            7 => Some(EXTI7_A::Ph7),
            8 => Some(EXTI7_A::Pi7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7_A::Pa7
    }
    #[doc = "Checks if the value of the field is `Pb7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7_A::Pb7
    }
    #[doc = "Checks if the value of the field is `Pc7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7_A::Pc7
    }
    #[doc = "Checks if the value of the field is `Pd7`"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == EXTI7_A::Pd7
    }
    #[doc = "Checks if the value of the field is `Pe7`"]
    #[inline(always)]
    pub fn is_pe7(&self) -> bool {
        *self == EXTI7_A::Pe7
    }
    #[doc = "Checks if the value of the field is `Pf7`"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7_A::Pf7
    }
    #[doc = "Checks if the value of the field is `Pg7`"]
    #[inline(always)]
    pub fn is_pg7(&self) -> bool {
        *self == EXTI7_A::Pg7
    }
    #[doc = "Checks if the value of the field is `Ph7`"]
    #[inline(always)]
    pub fn is_ph7(&self) -> bool {
        *self == EXTI7_A::Ph7
    }
    #[doc = "Checks if the value of the field is `Pi7`"]
    #[inline(always)]
    pub fn is_pi7(&self) -> bool {
        *self == EXTI7_A::Pi7
    }
}
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type EXTI7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI7_A, 4, O>;
impl<'a, const O: u8> EXTI7_W<'a, O> {
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pa7)
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pb7)
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pc7)
    }
    #[doc = "PD7 pin"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pd7)
    }
    #[doc = "PE7 pin"]
    #[inline(always)]
    pub fn pe7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pe7)
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pf7)
    }
    #[doc = "PG7 pin"]
    #[inline(always)]
    pub fn pg7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pg7)
    }
    #[doc = "PH7 pin"]
    #[inline(always)]
    pub fn ph7(self) -> &'a mut W {
        self.variant(EXTI7_A::Ph7)
    }
    #[doc = "PI7 pin"]
    #[inline(always)]
    pub fn pi7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pi7)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<0> {
        EXTI4_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<4> {
        EXTI5_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<8> {
        EXTI6_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<12> {
        EXTI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
