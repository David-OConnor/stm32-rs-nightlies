#[doc = "Register `AFRH` reader"]
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRH` writer"]
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub type AFSEL8_R = crate::FieldReader<u8, AFSEL8_A>;
#[doc = "Alternate function selection for port x pin y (y = 8..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL8_A {
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
}
impl From<AFSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL8_A) -> Self {
        variant as _
    }
}
impl AFSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AFSEL8_A> {
        match self.bits {
            0 => Some(AFSEL8_A::Af0),
            1 => Some(AFSEL8_A::Af1),
            2 => Some(AFSEL8_A::Af2),
            3 => Some(AFSEL8_A::Af3),
            4 => Some(AFSEL8_A::Af4),
            5 => Some(AFSEL8_A::Af5),
            6 => Some(AFSEL8_A::Af6),
            7 => Some(AFSEL8_A::Af7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Af0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFSEL8_A::Af0
    }
    #[doc = "Checks if the value of the field is `Af1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFSEL8_A::Af1
    }
    #[doc = "Checks if the value of the field is `Af2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFSEL8_A::Af2
    }
    #[doc = "Checks if the value of the field is `Af3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFSEL8_A::Af3
    }
    #[doc = "Checks if the value of the field is `Af4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFSEL8_A::Af4
    }
    #[doc = "Checks if the value of the field is `Af5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFSEL8_A::Af5
    }
    #[doc = "Checks if the value of the field is `Af6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFSEL8_A::Af6
    }
    #[doc = "Checks if the value of the field is `Af7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFSEL8_A::Af7
    }
}
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub type AFSEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, AFSEL8_A, 4, O>;
impl<'a, const O: u8> AFSEL8_W<'a, O> {
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL8_A::Af7)
    }
}
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL9_R;
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL10_R;
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL11_R;
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL12_R;
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL13_R;
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL14_R;
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_R as AFSEL15_R;
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL9_W;
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL10_W;
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL11_W;
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL12_W;
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL13_W;
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL14_W;
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x pin y (y = 8..15)"]
pub use AFSEL8_W as AFSEL15_W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<0> {
        AFSEL8_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<4> {
        AFSEL9_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<8> {
        AFSEL10_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<12> {
        AFSEL11_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<16> {
        AFSEL12_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<20> {
        AFSEL13_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<24> {
        AFSEL14_W::new(self)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<28> {
        AFSEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](index.html) module"]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrh::R](R) reader structure"]
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrh::W](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
