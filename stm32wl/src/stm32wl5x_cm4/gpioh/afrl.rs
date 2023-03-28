#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_R = crate::FieldReader<u8, AFRL3_A>;
#[doc = "Alternate function selection for port x bit y (y = 0..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRL3_A {
    #[doc = "0: AF0"]
    Af0 = 0,
    #[doc = "1: AF1"]
    Af1 = 1,
    #[doc = "2: AF2"]
    Af2 = 2,
    #[doc = "3: AF3"]
    Af3 = 3,
    #[doc = "4: AF4"]
    Af4 = 4,
    #[doc = "5: AF5"]
    Af5 = 5,
    #[doc = "6: AF6"]
    Af6 = 6,
    #[doc = "7: AF7"]
    Af7 = 7,
    #[doc = "8: AF8"]
    Af8 = 8,
    #[doc = "9: AF9"]
    Af9 = 9,
    #[doc = "10: AF10"]
    Af10 = 10,
    #[doc = "11: AF11"]
    Af11 = 11,
    #[doc = "12: AF12"]
    Af12 = 12,
    #[doc = "13: AF13"]
    Af13 = 13,
    #[doc = "14: AF14"]
    Af14 = 14,
    #[doc = "15: AF15"]
    Af15 = 15,
}
impl From<AFRL3_A> for u8 {
    #[inline(always)]
    fn from(variant: AFRL3_A) -> Self {
        variant as _
    }
}
impl AFRL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFRL3_A {
        match self.bits {
            0 => AFRL3_A::Af0,
            1 => AFRL3_A::Af1,
            2 => AFRL3_A::Af2,
            3 => AFRL3_A::Af3,
            4 => AFRL3_A::Af4,
            5 => AFRL3_A::Af5,
            6 => AFRL3_A::Af6,
            7 => AFRL3_A::Af7,
            8 => AFRL3_A::Af8,
            9 => AFRL3_A::Af9,
            10 => AFRL3_A::Af10,
            11 => AFRL3_A::Af11,
            12 => AFRL3_A::Af12,
            13 => AFRL3_A::Af13,
            14 => AFRL3_A::Af14,
            15 => AFRL3_A::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Af0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRL3_A::Af0
    }
    #[doc = "Checks if the value of the field is `Af1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRL3_A::Af1
    }
    #[doc = "Checks if the value of the field is `Af2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRL3_A::Af2
    }
    #[doc = "Checks if the value of the field is `Af3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRL3_A::Af3
    }
    #[doc = "Checks if the value of the field is `Af4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRL3_A::Af4
    }
    #[doc = "Checks if the value of the field is `Af5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRL3_A::Af5
    }
    #[doc = "Checks if the value of the field is `Af6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRL3_A::Af6
    }
    #[doc = "Checks if the value of the field is `Af7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRL3_A::Af7
    }
    #[doc = "Checks if the value of the field is `Af8`"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRL3_A::Af8
    }
    #[doc = "Checks if the value of the field is `Af9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRL3_A::Af9
    }
    #[doc = "Checks if the value of the field is `Af10`"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRL3_A::Af10
    }
    #[doc = "Checks if the value of the field is `Af11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRL3_A::Af11
    }
    #[doc = "Checks if the value of the field is `Af12`"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRL3_A::Af12
    }
    #[doc = "Checks if the value of the field is `Af13`"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRL3_A::Af13
    }
    #[doc = "Checks if the value of the field is `Af14`"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRL3_A::Af14
    }
    #[doc = "Checks if the value of the field is `Af15`"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRL3_A::Af15
    }
}
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AFRL_SPEC, u8, AFRL3_A, 4, O>;
impl<'a, const O: u8> AFRL3_W<'a, O> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL3_A::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL3_A::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL3_A::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL3_A::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL3_A::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL3_A::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL3_A::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL3_A::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL3_A::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL3_A::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL3_A::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL3_A::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL3_A::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL3_A::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL3_A::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL3_A::Af15)
    }
}
impl R {
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl3(&mut self) -> AFRL3_W<12> {
        AFRL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
