#[doc = "Register `PUPDR` reader"]
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUPDR` writer"]
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUPD0` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD0_R = crate::FieldReader<u8, PUPD0_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD0_A {
    #[doc = "0: No pull-up, pull-down"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD0_A) -> Self {
        variant as _
    }
}
impl PUPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPD0_A> {
        match self.bits {
            0 => Some(PUPD0_A::Floating),
            1 => Some(PUPD0_A::PullUp),
            2 => Some(PUPD0_A::PullDown),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Floating`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPD0_A::Floating
    }
    #[doc = "Checks if the value of the field is `PullUp`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPD0_A::PullUp
    }
    #[doc = "Checks if the value of the field is `PullDown`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPD0_A::PullDown
    }
}
#[doc = "Field `PUPD0` writer - Port x configuration bits (y = 0..15)"]
pub type PUPD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, PUPD0_A, 2, O>;
impl<'a, const O: u8> PUPD0_W<'a, O> {
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD0_A::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD0_A::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD0_A::PullDown)
    }
}
#[doc = "Field `PUPD1` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD1_R;
#[doc = "Field `PUPD2` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD2_R;
#[doc = "Field `PUPD3` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD3_R;
#[doc = "Field `PUPD4` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD4_R;
#[doc = "Field `PUPD5` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD5_R;
#[doc = "Field `PUPD6` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD6_R;
#[doc = "Field `PUPD7` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD7_R;
#[doc = "Field `PUPD8` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD8_R;
#[doc = "Field `PUPD9` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD9_R;
#[doc = "Field `PUPD10` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD10_R;
#[doc = "Field `PUPD11` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD11_R;
#[doc = "Field `PUPD12` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD12_R;
#[doc = "Field `PUPD13` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD13_R;
#[doc = "Field `PUPD14` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD14_R;
#[doc = "Field `PUPD15` reader - Port x configuration bits (y = 0..15)"]
pub use PUPD0_R as PUPD15_R;
#[doc = "Field `PUPD1` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD1_W;
#[doc = "Field `PUPD2` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD2_W;
#[doc = "Field `PUPD3` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD3_W;
#[doc = "Field `PUPD4` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD4_W;
#[doc = "Field `PUPD5` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD5_W;
#[doc = "Field `PUPD6` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD6_W;
#[doc = "Field `PUPD7` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD7_W;
#[doc = "Field `PUPD8` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD8_W;
#[doc = "Field `PUPD9` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD9_W;
#[doc = "Field `PUPD10` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD10_W;
#[doc = "Field `PUPD11` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD11_W;
#[doc = "Field `PUPD12` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD12_W;
#[doc = "Field `PUPD13` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD13_W;
#[doc = "Field `PUPD14` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD14_W;
#[doc = "Field `PUPD15` writer - Port x configuration bits (y = 0..15)"]
pub use PUPD0_W as PUPD15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd0(&mut self) -> PUPD0_W<0> {
        PUPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd1(&mut self) -> PUPD1_W<2> {
        PUPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd2(&mut self) -> PUPD2_W<4> {
        PUPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd3(&mut self) -> PUPD3_W<6> {
        PUPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd4(&mut self) -> PUPD4_W<8> {
        PUPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd5(&mut self) -> PUPD5_W<10> {
        PUPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd6(&mut self) -> PUPD6_W<12> {
        PUPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd7(&mut self) -> PUPD7_W<14> {
        PUPD7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd8(&mut self) -> PUPD8_W<16> {
        PUPD8_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd9(&mut self) -> PUPD9_W<18> {
        PUPD9_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd10(&mut self) -> PUPD10_W<20> {
        PUPD10_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd11(&mut self) -> PUPD11_W<22> {
        PUPD11_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd12(&mut self) -> PUPD12_W<24> {
        PUPD12_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd13(&mut self) -> PUPD13_W<26> {
        PUPD13_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd14(&mut self) -> PUPD14_W<28> {
        PUPD14_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupd15(&mut self) -> PUPD15_W<30> {
        PUPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](index.html) module"]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pupdr::R](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pupdr::W](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
