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
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_R = crate::FieldReader<u8, AFSEL0_A>;
#[doc = "Alternate function selection for port x bit y (y = 0..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL0_A {
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
impl From<AFSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL0_A) -> Self {
        variant as _
    }
}
impl AFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFSEL0_A {
        match self.bits {
            0 => AFSEL0_A::Af0,
            1 => AFSEL0_A::Af1,
            2 => AFSEL0_A::Af2,
            3 => AFSEL0_A::Af3,
            4 => AFSEL0_A::Af4,
            5 => AFSEL0_A::Af5,
            6 => AFSEL0_A::Af6,
            7 => AFSEL0_A::Af7,
            8 => AFSEL0_A::Af8,
            9 => AFSEL0_A::Af9,
            10 => AFSEL0_A::Af10,
            11 => AFSEL0_A::Af11,
            12 => AFSEL0_A::Af12,
            13 => AFSEL0_A::Af13,
            14 => AFSEL0_A::Af14,
            15 => AFSEL0_A::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Af0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFSEL0_A::Af0
    }
    #[doc = "Checks if the value of the field is `Af1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFSEL0_A::Af1
    }
    #[doc = "Checks if the value of the field is `Af2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFSEL0_A::Af2
    }
    #[doc = "Checks if the value of the field is `Af3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFSEL0_A::Af3
    }
    #[doc = "Checks if the value of the field is `Af4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFSEL0_A::Af4
    }
    #[doc = "Checks if the value of the field is `Af5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFSEL0_A::Af5
    }
    #[doc = "Checks if the value of the field is `Af6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFSEL0_A::Af6
    }
    #[doc = "Checks if the value of the field is `Af7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFSEL0_A::Af7
    }
    #[doc = "Checks if the value of the field is `Af8`"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFSEL0_A::Af8
    }
    #[doc = "Checks if the value of the field is `Af9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFSEL0_A::Af9
    }
    #[doc = "Checks if the value of the field is `Af10`"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFSEL0_A::Af10
    }
    #[doc = "Checks if the value of the field is `Af11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFSEL0_A::Af11
    }
    #[doc = "Checks if the value of the field is `Af12`"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFSEL0_A::Af12
    }
    #[doc = "Checks if the value of the field is `Af13`"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFSEL0_A::Af13
    }
    #[doc = "Checks if the value of the field is `Af14`"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFSEL0_A::Af14
    }
    #[doc = "Checks if the value of the field is `Af15`"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFSEL0_A::Af15
    }
}
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AFRL_SPEC, u8, AFSEL0_A, 4, O>;
impl<'a, const O: u8> AFSEL0_W<'a, O> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFSEL0_A::Af15)
    }
}
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL1_R;
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL2_R;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL3_R;
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL4_R;
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL5_R;
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL6_R;
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_R as AFSEL7_R;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL1_W;
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL2_W;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL3_W;
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL4_W;
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL5_W;
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL6_W;
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub use AFSEL0_W as AFSEL7_W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<0> {
        AFSEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<4> {
        AFSEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> AFSEL2_W<8> {
        AFSEL2_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<12> {
        AFSEL3_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> AFSEL4_W<16> {
        AFSEL4_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> AFSEL5_W<20> {
        AFSEL5_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> AFSEL6_W<24> {
        AFSEL6_W::new(self)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> AFSEL7_W<28> {
        AFSEL7_W::new(self)
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
