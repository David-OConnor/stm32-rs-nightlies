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
#[doc = "Field `AFR0` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR0_R = crate::FieldReader<u8, AFR0_A>;
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFR0_A {
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
impl From<AFR0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFR0_A) -> Self {
        variant as _
    }
}
impl AFR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFR0_A {
        match self.bits {
            0 => AFR0_A::Af0,
            1 => AFR0_A::Af1,
            2 => AFR0_A::Af2,
            3 => AFR0_A::Af3,
            4 => AFR0_A::Af4,
            5 => AFR0_A::Af5,
            6 => AFR0_A::Af6,
            7 => AFR0_A::Af7,
            8 => AFR0_A::Af8,
            9 => AFR0_A::Af9,
            10 => AFR0_A::Af10,
            11 => AFR0_A::Af11,
            12 => AFR0_A::Af12,
            13 => AFR0_A::Af13,
            14 => AFR0_A::Af14,
            15 => AFR0_A::Af15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Af0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFR0_A::Af0
    }
    #[doc = "Checks if the value of the field is `Af1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFR0_A::Af1
    }
    #[doc = "Checks if the value of the field is `Af2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFR0_A::Af2
    }
    #[doc = "Checks if the value of the field is `Af3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFR0_A::Af3
    }
    #[doc = "Checks if the value of the field is `Af4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFR0_A::Af4
    }
    #[doc = "Checks if the value of the field is `Af5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFR0_A::Af5
    }
    #[doc = "Checks if the value of the field is `Af6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFR0_A::Af6
    }
    #[doc = "Checks if the value of the field is `Af7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFR0_A::Af7
    }
    #[doc = "Checks if the value of the field is `Af8`"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFR0_A::Af8
    }
    #[doc = "Checks if the value of the field is `Af9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFR0_A::Af9
    }
    #[doc = "Checks if the value of the field is `Af10`"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFR0_A::Af10
    }
    #[doc = "Checks if the value of the field is `Af11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFR0_A::Af11
    }
    #[doc = "Checks if the value of the field is `Af12`"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFR0_A::Af12
    }
    #[doc = "Checks if the value of the field is `Af13`"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFR0_A::Af13
    }
    #[doc = "Checks if the value of the field is `Af14`"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFR0_A::Af14
    }
    #[doc = "Checks if the value of the field is `Af15`"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFR0_A::Af15
    }
}
#[doc = "Field `AFR0` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AFRL_SPEC, u8, AFR0_A, 4, O>;
impl<'a, const O: u8> AFR0_W<'a, O> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR0_A::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR0_A::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR0_A::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR0_A::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR0_A::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR0_A::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR0_A::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR0_A::Af7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR0_A::Af8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR0_A::Af9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR0_A::Af10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR0_A::Af11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR0_A::Af12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR0_A::Af13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR0_A::Af14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR0_A::Af15)
    }
}
#[doc = "Field `AFR1` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR1_R;
#[doc = "Field `AFR2` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR2_R;
#[doc = "Field `AFR3` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR3_R;
#[doc = "Field `AFR4` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR4_R;
#[doc = "Field `AFR5` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR5_R;
#[doc = "Field `AFR6` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR6_R;
#[doc = "Field `AFR7` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_R as AFR7_R;
#[doc = "Field `AFR1` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR1_W;
#[doc = "Field `AFR2` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR2_W;
#[doc = "Field `AFR3` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR3_W;
#[doc = "Field `AFR4` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR4_W;
#[doc = "Field `AFR5` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR5_W;
#[doc = "Field `AFR6` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR6_W;
#[doc = "Field `AFR7` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub use AFR0_W as AFR7_W;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR0_W<0> {
        AFR0_W::new(self)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR1_W<4> {
        AFR1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr2(&mut self) -> AFR2_W<8> {
        AFR2_W::new(self)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR3_W<12> {
        AFR3_W::new(self)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr4(&mut self) -> AFR4_W<16> {
        AFR4_W::new(self)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr5(&mut self) -> AFR5_W<20> {
        AFR5_W::new(self)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR6_W<24> {
        AFR6_W::new(self)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR7_W<28> {
        AFR7_W::new(self)
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
