#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - EXTI12 configuration bits"]
pub type EXTI12_R = crate::FieldReader<u8, EXTI12_A>;
#[doc = "EXTI12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: PA12 pin"]
    Pa12 = 0,
    #[doc = "1: PB12 pin"]
    Pb12 = 1,
    #[doc = "2: PC12 pin"]
    Pc12 = 2,
    #[doc = "3: PD12 pin"]
    Pd12 = 3,
    #[doc = "4: PE12 pin"]
    Pe12 = 4,
    #[doc = "5: PF12 pin"]
    Pf12 = 5,
    #[doc = "6: PG12 pin"]
    Pg12 = 6,
    #[doc = "7: PH12 pin"]
    Ph12 = 7,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
impl EXTI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::Pa12),
            1 => Some(EXTI12_A::Pb12),
            2 => Some(EXTI12_A::Pc12),
            3 => Some(EXTI12_A::Pd12),
            4 => Some(EXTI12_A::Pe12),
            5 => Some(EXTI12_A::Pf12),
            6 => Some(EXTI12_A::Pg12),
            7 => Some(EXTI12_A::Ph12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_A::Pa12
    }
    #[doc = "Checks if the value of the field is `Pb12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12_A::Pb12
    }
    #[doc = "Checks if the value of the field is `Pc12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12_A::Pc12
    }
    #[doc = "Checks if the value of the field is `Pd12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12_A::Pd12
    }
    #[doc = "Checks if the value of the field is `Pe12`"]
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12_A::Pe12
    }
    #[doc = "Checks if the value of the field is `Pf12`"]
    #[inline(always)]
    pub fn is_pf12(&self) -> bool {
        *self == EXTI12_A::Pf12
    }
    #[doc = "Checks if the value of the field is `Pg12`"]
    #[inline(always)]
    pub fn is_pg12(&self) -> bool {
        *self == EXTI12_A::Pg12
    }
    #[doc = "Checks if the value of the field is `Ph12`"]
    #[inline(always)]
    pub fn is_ph12(&self) -> bool {
        *self == EXTI12_A::Ph12
    }
}
#[doc = "Field `EXTI12` writer - EXTI12 configuration bits"]
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI12_A, 4, O>;
impl<'a, const O: u8> EXTI12_W<'a, O> {
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pa12)
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pb12)
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pc12)
    }
    #[doc = "PD12 pin"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pd12)
    }
    #[doc = "PE12 pin"]
    #[inline(always)]
    pub fn pe12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pe12)
    }
    #[doc = "PF12 pin"]
    #[inline(always)]
    pub fn pf12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pf12)
    }
    #[doc = "PG12 pin"]
    #[inline(always)]
    pub fn pg12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pg12)
    }
    #[doc = "PH12 pin"]
    #[inline(always)]
    pub fn ph12(self) -> &'a mut W {
        self.variant(EXTI12_A::Ph12)
    }
}
#[doc = "Field `EXTI13` reader - EXTI13 configuration bits"]
pub type EXTI13_R = crate::FieldReader<u8, EXTI13_A>;
#[doc = "EXTI13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: PA13 pin"]
    Pa13 = 0,
    #[doc = "1: PB13 pin"]
    Pb13 = 1,
    #[doc = "2: PC13 pin"]
    Pc13 = 2,
    #[doc = "3: PD13 pin"]
    Pd13 = 3,
    #[doc = "4: PE13 pin"]
    Pe13 = 4,
    #[doc = "5: PF13 pin"]
    Pf13 = 5,
    #[doc = "6: PG13 pin"]
    Pg13 = 6,
    #[doc = "7: PH13 pin"]
    Ph13 = 7,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
impl EXTI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::Pa13),
            1 => Some(EXTI13_A::Pb13),
            2 => Some(EXTI13_A::Pc13),
            3 => Some(EXTI13_A::Pd13),
            4 => Some(EXTI13_A::Pe13),
            5 => Some(EXTI13_A::Pf13),
            6 => Some(EXTI13_A::Pg13),
            7 => Some(EXTI13_A::Ph13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_A::Pa13
    }
    #[doc = "Checks if the value of the field is `Pb13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13_A::Pb13
    }
    #[doc = "Checks if the value of the field is `Pc13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_A::Pc13
    }
    #[doc = "Checks if the value of the field is `Pd13`"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13_A::Pd13
    }
    #[doc = "Checks if the value of the field is `Pe13`"]
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13_A::Pe13
    }
    #[doc = "Checks if the value of the field is `Pf13`"]
    #[inline(always)]
    pub fn is_pf13(&self) -> bool {
        *self == EXTI13_A::Pf13
    }
    #[doc = "Checks if the value of the field is `Pg13`"]
    #[inline(always)]
    pub fn is_pg13(&self) -> bool {
        *self == EXTI13_A::Pg13
    }
    #[doc = "Checks if the value of the field is `Ph13`"]
    #[inline(always)]
    pub fn is_ph13(&self) -> bool {
        *self == EXTI13_A::Ph13
    }
}
#[doc = "Field `EXTI13` writer - EXTI13 configuration bits"]
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI13_A, 4, O>;
impl<'a, const O: u8> EXTI13_W<'a, O> {
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pa13)
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pb13)
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pc13)
    }
    #[doc = "PD13 pin"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pd13)
    }
    #[doc = "PE13 pin"]
    #[inline(always)]
    pub fn pe13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pe13)
    }
    #[doc = "PF13 pin"]
    #[inline(always)]
    pub fn pf13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pf13)
    }
    #[doc = "PG13 pin"]
    #[inline(always)]
    pub fn pg13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pg13)
    }
    #[doc = "PH13 pin"]
    #[inline(always)]
    pub fn ph13(self) -> &'a mut W {
        self.variant(EXTI13_A::Ph13)
    }
}
#[doc = "Field `EXTI14` reader - EXTI14 configuration bits"]
pub type EXTI14_R = crate::FieldReader<u8, EXTI14_A>;
#[doc = "EXTI14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: PA14 pin"]
    Pa14 = 0,
    #[doc = "1: PB14 pin"]
    Pb14 = 1,
    #[doc = "2: PC14 pin"]
    Pc14 = 2,
    #[doc = "3: PD14 pin"]
    Pd14 = 3,
    #[doc = "4: PE14 pin"]
    Pe14 = 4,
    #[doc = "5: PF14 pin"]
    Pf14 = 5,
    #[doc = "6: PG14 pin"]
    Pg14 = 6,
    #[doc = "7: PH14 pin"]
    Ph14 = 7,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
impl EXTI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::Pa14),
            1 => Some(EXTI14_A::Pb14),
            2 => Some(EXTI14_A::Pc14),
            3 => Some(EXTI14_A::Pd14),
            4 => Some(EXTI14_A::Pe14),
            5 => Some(EXTI14_A::Pf14),
            6 => Some(EXTI14_A::Pg14),
            7 => Some(EXTI14_A::Ph14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_A::Pa14
    }
    #[doc = "Checks if the value of the field is `Pb14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_A::Pb14
    }
    #[doc = "Checks if the value of the field is `Pc14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_A::Pc14
    }
    #[doc = "Checks if the value of the field is `Pd14`"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14_A::Pd14
    }
    #[doc = "Checks if the value of the field is `Pe14`"]
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14_A::Pe14
    }
    #[doc = "Checks if the value of the field is `Pf14`"]
    #[inline(always)]
    pub fn is_pf14(&self) -> bool {
        *self == EXTI14_A::Pf14
    }
    #[doc = "Checks if the value of the field is `Pg14`"]
    #[inline(always)]
    pub fn is_pg14(&self) -> bool {
        *self == EXTI14_A::Pg14
    }
    #[doc = "Checks if the value of the field is `Ph14`"]
    #[inline(always)]
    pub fn is_ph14(&self) -> bool {
        *self == EXTI14_A::Ph14
    }
}
#[doc = "Field `EXTI14` writer - EXTI14 configuration bits"]
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI14_A, 4, O>;
impl<'a, const O: u8> EXTI14_W<'a, O> {
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pa14)
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pb14)
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pc14)
    }
    #[doc = "PD14 pin"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pd14)
    }
    #[doc = "PE14 pin"]
    #[inline(always)]
    pub fn pe14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pe14)
    }
    #[doc = "PF14 pin"]
    #[inline(always)]
    pub fn pf14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pf14)
    }
    #[doc = "PG14 pin"]
    #[inline(always)]
    pub fn pg14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pg14)
    }
    #[doc = "PH14 pin"]
    #[inline(always)]
    pub fn ph14(self) -> &'a mut W {
        self.variant(EXTI14_A::Ph14)
    }
}
#[doc = "Field `EXTI15` reader - EXTI15 configuration bits"]
pub type EXTI15_R = crate::FieldReader<u8, EXTI15_A>;
#[doc = "EXTI15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: PA15 pin"]
    Pa15 = 0,
    #[doc = "1: PB15 pin"]
    Pb15 = 1,
    #[doc = "2: PC15 pin"]
    Pc15 = 2,
    #[doc = "3: PD15 pin"]
    Pd15 = 3,
    #[doc = "4: PE15 pin"]
    Pe15 = 4,
    #[doc = "5: PF15 pin"]
    Pf15 = 5,
    #[doc = "6: PG15 pin"]
    Pg15 = 6,
    #[doc = "7: PH15 pin"]
    Ph15 = 7,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
impl EXTI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::Pa15),
            1 => Some(EXTI15_A::Pb15),
            2 => Some(EXTI15_A::Pc15),
            3 => Some(EXTI15_A::Pd15),
            4 => Some(EXTI15_A::Pe15),
            5 => Some(EXTI15_A::Pf15),
            6 => Some(EXTI15_A::Pg15),
            7 => Some(EXTI15_A::Ph15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_A::Pa15
    }
    #[doc = "Checks if the value of the field is `Pb15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_A::Pb15
    }
    #[doc = "Checks if the value of the field is `Pc15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_A::Pc15
    }
    #[doc = "Checks if the value of the field is `Pd15`"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15_A::Pd15
    }
    #[doc = "Checks if the value of the field is `Pe15`"]
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15_A::Pe15
    }
    #[doc = "Checks if the value of the field is `Pf15`"]
    #[inline(always)]
    pub fn is_pf15(&self) -> bool {
        *self == EXTI15_A::Pf15
    }
    #[doc = "Checks if the value of the field is `Pg15`"]
    #[inline(always)]
    pub fn is_pg15(&self) -> bool {
        *self == EXTI15_A::Pg15
    }
    #[doc = "Checks if the value of the field is `Ph15`"]
    #[inline(always)]
    pub fn is_ph15(&self) -> bool {
        *self == EXTI15_A::Ph15
    }
}
#[doc = "Field `EXTI15` writer - EXTI15 configuration bits"]
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI15_A, 4, O>;
impl<'a, const O: u8> EXTI15_W<'a, O> {
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pa15)
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pb15)
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pc15)
    }
    #[doc = "PD15 pin"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pd15)
    }
    #[doc = "PE15 pin"]
    #[inline(always)]
    pub fn pe15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pe15)
    }
    #[doc = "PF15 pin"]
    #[inline(always)]
    pub fn pf15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pf15)
    }
    #[doc = "PG15 pin"]
    #[inline(always)]
    pub fn pg15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pg15)
    }
    #[doc = "PH15 pin"]
    #[inline(always)]
    pub fn ph15(self) -> &'a mut W {
        self.variant(EXTI15_A::Ph15)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<4> {
        EXTI13_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<8> {
        EXTI14_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<12> {
        EXTI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
