#[doc = "Register `PRESC` reader"]
pub struct R(crate::R<PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESC` writer"]
pub struct W(crate::W<PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESC_SPEC>;
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
impl From<crate::W<PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALER_A>;
#[doc = "Clock prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: /1"]
    Div1 = 0,
    #[doc = "1: /2"]
    Div2 = 1,
    #[doc = "2: /4"]
    Div4 = 2,
    #[doc = "3: /6"]
    Div6 = 3,
    #[doc = "4: /8"]
    Div8 = 4,
    #[doc = "5: /10"]
    Div10 = 5,
    #[doc = "6: /12"]
    Div12 = 6,
    #[doc = "7: /16"]
    Div16 = 7,
    #[doc = "8: /32"]
    Div32 = 8,
    #[doc = "9: /64"]
    Div64 = 9,
    #[doc = "10: /128"]
    Div128 = 10,
    #[doc = "11: /256"]
    Div256 = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::Div1),
            1 => Some(PRESCALER_A::Div2),
            2 => Some(PRESCALER_A::Div4),
            3 => Some(PRESCALER_A::Div6),
            4 => Some(PRESCALER_A::Div8),
            5 => Some(PRESCALER_A::Div10),
            6 => Some(PRESCALER_A::Div12),
            7 => Some(PRESCALER_A::Div16),
            8 => Some(PRESCALER_A::Div32),
            9 => Some(PRESCALER_A::Div64),
            10 => Some(PRESCALER_A::Div128),
            11 => Some(PRESCALER_A::Div256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESCALER_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESCALER_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESCALER_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::Div256
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRESC_SPEC, u8, PRESCALER_A, 4, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div8)
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div10)
    }
    #[doc = "/12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div12)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::Div256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presc](index.html) module"]
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presc::R](R) reader structure"]
impl crate::Readable for PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presc::W](W) writer structure"]
impl crate::Writable for PRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
