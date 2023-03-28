#[doc = "Register `DIFSEL` reader"]
pub struct R(crate::R<DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIFSEL` writer"]
pub struct W(crate::W<DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIFSEL_SPEC>;
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
impl From<crate::W<DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFSEL0` reader - ADC channel differential or single-ended mode for channel"]
pub type DIFSEL0_R = crate::BitReader<DIFSEL0_A>;
#[doc = "ADC channel differential or single-ended mode for channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL0_A {
    #[doc = "0: Input channel is configured in single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Input channel is configured in differential mode"]
    Differential = 1,
}
impl From<DIFSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSEL0_A {
        match self.bits {
            false => DIFSEL0_A::SingleEnded,
            true => DIFSEL0_A::Differential,
        }
    }
    #[doc = "Checks if the value of the field is `SingleEnded`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL0_A::SingleEnded
    }
    #[doc = "Checks if the value of the field is `Differential`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL0_A::Differential
    }
}
#[doc = "Field `DIFSEL0` writer - ADC channel differential or single-ended mode for channel"]
pub type DIFSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFSEL_SPEC, DIFSEL0_A, O>;
impl<'a, const O: u8> DIFSEL0_W<'a, O> {
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SingleEnded)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::Differential)
    }
}
#[doc = "Field `DIFSEL1` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL1_R;
#[doc = "Field `DIFSEL2` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL2_R;
#[doc = "Field `DIFSEL3` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL3_R;
#[doc = "Field `DIFSEL4` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL4_R;
#[doc = "Field `DIFSEL5` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL5_R;
#[doc = "Field `DIFSEL6` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL6_R;
#[doc = "Field `DIFSEL7` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL7_R;
#[doc = "Field `DIFSEL8` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL8_R;
#[doc = "Field `DIFSEL9` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL9_R;
#[doc = "Field `DIFSEL10` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL10_R;
#[doc = "Field `DIFSEL11` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL11_R;
#[doc = "Field `DIFSEL12` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL12_R;
#[doc = "Field `DIFSEL13` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL13_R;
#[doc = "Field `DIFSEL14` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL14_R;
#[doc = "Field `DIFSEL15` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL15_R;
#[doc = "Field `DIFSEL16` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL16_R;
#[doc = "Field `DIFSEL17` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL17_R;
#[doc = "Field `DIFSEL18` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL18_R;
#[doc = "Field `DIFSEL19` reader - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_R as DIFSEL19_R;
#[doc = "Field `DIFSEL1` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL1_W;
#[doc = "Field `DIFSEL2` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL2_W;
#[doc = "Field `DIFSEL3` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL3_W;
#[doc = "Field `DIFSEL4` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL4_W;
#[doc = "Field `DIFSEL5` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL5_W;
#[doc = "Field `DIFSEL6` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL6_W;
#[doc = "Field `DIFSEL7` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL7_W;
#[doc = "Field `DIFSEL8` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL8_W;
#[doc = "Field `DIFSEL9` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL9_W;
#[doc = "Field `DIFSEL10` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL10_W;
#[doc = "Field `DIFSEL11` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL11_W;
#[doc = "Field `DIFSEL12` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL12_W;
#[doc = "Field `DIFSEL13` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL13_W;
#[doc = "Field `DIFSEL14` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL14_W;
#[doc = "Field `DIFSEL15` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL15_W;
#[doc = "Field `DIFSEL16` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL16_W;
#[doc = "Field `DIFSEL17` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL17_W;
#[doc = "Field `DIFSEL18` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL18_W;
#[doc = "Field `DIFSEL19` writer - ADC channel differential or single-ended mode for channel"]
pub use DIFSEL0_W as DIFSEL19_W;
impl R {
    #[doc = "Bit 0 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel0(&self) -> DIFSEL0_R {
        DIFSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel1(&self) -> DIFSEL1_R {
        DIFSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel2(&self) -> DIFSEL2_R {
        DIFSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel3(&self) -> DIFSEL3_R {
        DIFSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel4(&self) -> DIFSEL4_R {
        DIFSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel5(&self) -> DIFSEL5_R {
        DIFSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel6(&self) -> DIFSEL6_R {
        DIFSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel7(&self) -> DIFSEL7_R {
        DIFSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel8(&self) -> DIFSEL8_R {
        DIFSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel9(&self) -> DIFSEL9_R {
        DIFSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel10(&self) -> DIFSEL10_R {
        DIFSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel11(&self) -> DIFSEL11_R {
        DIFSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel12(&self) -> DIFSEL12_R {
        DIFSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel13(&self) -> DIFSEL13_R {
        DIFSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel14(&self) -> DIFSEL14_R {
        DIFSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel15(&self) -> DIFSEL15_R {
        DIFSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel16(&self) -> DIFSEL16_R {
        DIFSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel17(&self) -> DIFSEL17_R {
        DIFSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel18(&self) -> DIFSEL18_R {
        DIFSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel19(&self) -> DIFSEL19_R {
        DIFSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel0(&mut self) -> DIFSEL0_W<0> {
        DIFSEL0_W::new(self)
    }
    #[doc = "Bit 1 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel1(&mut self) -> DIFSEL1_W<1> {
        DIFSEL1_W::new(self)
    }
    #[doc = "Bit 2 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel2(&mut self) -> DIFSEL2_W<2> {
        DIFSEL2_W::new(self)
    }
    #[doc = "Bit 3 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel3(&mut self) -> DIFSEL3_W<3> {
        DIFSEL3_W::new(self)
    }
    #[doc = "Bit 4 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel4(&mut self) -> DIFSEL4_W<4> {
        DIFSEL4_W::new(self)
    }
    #[doc = "Bit 5 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel5(&mut self) -> DIFSEL5_W<5> {
        DIFSEL5_W::new(self)
    }
    #[doc = "Bit 6 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel6(&mut self) -> DIFSEL6_W<6> {
        DIFSEL6_W::new(self)
    }
    #[doc = "Bit 7 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel7(&mut self) -> DIFSEL7_W<7> {
        DIFSEL7_W::new(self)
    }
    #[doc = "Bit 8 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel8(&mut self) -> DIFSEL8_W<8> {
        DIFSEL8_W::new(self)
    }
    #[doc = "Bit 9 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel9(&mut self) -> DIFSEL9_W<9> {
        DIFSEL9_W::new(self)
    }
    #[doc = "Bit 10 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel10(&mut self) -> DIFSEL10_W<10> {
        DIFSEL10_W::new(self)
    }
    #[doc = "Bit 11 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel11(&mut self) -> DIFSEL11_W<11> {
        DIFSEL11_W::new(self)
    }
    #[doc = "Bit 12 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel12(&mut self) -> DIFSEL12_W<12> {
        DIFSEL12_W::new(self)
    }
    #[doc = "Bit 13 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel13(&mut self) -> DIFSEL13_W<13> {
        DIFSEL13_W::new(self)
    }
    #[doc = "Bit 14 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel14(&mut self) -> DIFSEL14_W<14> {
        DIFSEL14_W::new(self)
    }
    #[doc = "Bit 15 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel15(&mut self) -> DIFSEL15_W<15> {
        DIFSEL15_W::new(self)
    }
    #[doc = "Bit 16 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel16(&mut self) -> DIFSEL16_W<16> {
        DIFSEL16_W::new(self)
    }
    #[doc = "Bit 17 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel17(&mut self) -> DIFSEL17_W<17> {
        DIFSEL17_W::new(self)
    }
    #[doc = "Bit 18 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel18(&mut self) -> DIFSEL18_W<18> {
        DIFSEL18_W::new(self)
    }
    #[doc = "Bit 19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel19(&mut self) -> DIFSEL19_W<19> {
        DIFSEL19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC channel differential or single-ended mode selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsel](index.html) module"]
pub struct DIFSEL_SPEC;
impl crate::RegisterSpec for DIFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [difsel::R](R) reader structure"]
impl crate::Readable for DIFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [difsel::W](W) writer structure"]
impl crate::Writable for DIFSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
