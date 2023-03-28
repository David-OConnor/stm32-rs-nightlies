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
#[doc = "Field `DIFSEL_0` reader - Differential mode for channels 0"]
pub type DIFSEL_0_R = crate::BitReader<DIFSEL_0_A>;
#[doc = "Differential mode for channels 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL_0_A {
    #[doc = "0: Input channel is configured in single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Input channel is configured in differential mode"]
    Differential = 1,
}
impl From<DIFSEL_0_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL_0_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFSEL_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSEL_0_A {
        match self.bits {
            false => DIFSEL_0_A::SingleEnded,
            true => DIFSEL_0_A::Differential,
        }
    }
    #[doc = "Checks if the value of the field is `SingleEnded`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL_0_A::SingleEnded
    }
    #[doc = "Checks if the value of the field is `Differential`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL_0_A::Differential
    }
}
#[doc = "Field `DIFSEL_0` writer - Differential mode for channels 0"]
pub type DIFSEL_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFSEL_SPEC, DIFSEL_0_A, O>;
impl<'a, const O: u8> DIFSEL_0_W<'a, O> {
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_0_A::SingleEnded)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_0_A::Differential)
    }
}
#[doc = "Field `DIFSEL_1` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_1_R;
#[doc = "Field `DIFSEL_2` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_2_R;
#[doc = "Field `DIFSEL_3` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_3_R;
#[doc = "Field `DIFSEL_4` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_4_R;
#[doc = "Field `DIFSEL_5` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_5_R;
#[doc = "Field `DIFSEL_6` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_6_R;
#[doc = "Field `DIFSEL_7` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_7_R;
#[doc = "Field `DIFSEL_8` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_8_R;
#[doc = "Field `DIFSEL_9` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_9_R;
#[doc = "Field `DIFSEL_10` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_10_R;
#[doc = "Field `DIFSEL_11` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_11_R;
#[doc = "Field `DIFSEL_12` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_12_R;
#[doc = "Field `DIFSEL_13` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_13_R;
#[doc = "Field `DIFSEL_14` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_14_R;
#[doc = "Field `DIFSEL_15` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_15_R;
#[doc = "Field `DIFSEL_16` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_16_R;
#[doc = "Field `DIFSEL_17` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_17_R;
#[doc = "Field `DIFSEL_18` reader - Differential mode for channels 0"]
pub use DIFSEL_0_R as DIFSEL_18_R;
#[doc = "Field `DIFSEL_1` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_1_W;
#[doc = "Field `DIFSEL_2` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_2_W;
#[doc = "Field `DIFSEL_3` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_3_W;
#[doc = "Field `DIFSEL_4` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_4_W;
#[doc = "Field `DIFSEL_5` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_5_W;
#[doc = "Field `DIFSEL_6` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_6_W;
#[doc = "Field `DIFSEL_7` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_7_W;
#[doc = "Field `DIFSEL_8` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_8_W;
#[doc = "Field `DIFSEL_9` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_9_W;
#[doc = "Field `DIFSEL_10` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_10_W;
#[doc = "Field `DIFSEL_11` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_11_W;
#[doc = "Field `DIFSEL_12` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_12_W;
#[doc = "Field `DIFSEL_13` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_13_W;
#[doc = "Field `DIFSEL_14` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_14_W;
#[doc = "Field `DIFSEL_15` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_15_W;
#[doc = "Field `DIFSEL_16` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_16_W;
#[doc = "Field `DIFSEL_17` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_17_W;
#[doc = "Field `DIFSEL_18` writer - Differential mode for channels 0"]
pub use DIFSEL_0_W as DIFSEL_18_W;
impl R {
    #[doc = "Bit 0 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_1(&self) -> DIFSEL_1_R {
        DIFSEL_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_2(&self) -> DIFSEL_2_R {
        DIFSEL_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_3(&self) -> DIFSEL_3_R {
        DIFSEL_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_4(&self) -> DIFSEL_4_R {
        DIFSEL_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_5(&self) -> DIFSEL_5_R {
        DIFSEL_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_6(&self) -> DIFSEL_6_R {
        DIFSEL_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_7(&self) -> DIFSEL_7_R {
        DIFSEL_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_8(&self) -> DIFSEL_8_R {
        DIFSEL_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_9(&self) -> DIFSEL_9_R {
        DIFSEL_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_10(&self) -> DIFSEL_10_R {
        DIFSEL_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_11(&self) -> DIFSEL_11_R {
        DIFSEL_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_12(&self) -> DIFSEL_12_R {
        DIFSEL_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_13(&self) -> DIFSEL_13_R {
        DIFSEL_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_14(&self) -> DIFSEL_14_R {
        DIFSEL_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_15(&self) -> DIFSEL_15_R {
        DIFSEL_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_16(&self) -> DIFSEL_16_R {
        DIFSEL_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_17(&self) -> DIFSEL_17_R {
        DIFSEL_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_18(&self) -> DIFSEL_18_R {
        DIFSEL_18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_0(&mut self) -> DIFSEL_0_W<0> {
        DIFSEL_0_W::new(self)
    }
    #[doc = "Bit 1 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_1(&mut self) -> DIFSEL_1_W<1> {
        DIFSEL_1_W::new(self)
    }
    #[doc = "Bit 2 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_2(&mut self) -> DIFSEL_2_W<2> {
        DIFSEL_2_W::new(self)
    }
    #[doc = "Bit 3 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_3(&mut self) -> DIFSEL_3_W<3> {
        DIFSEL_3_W::new(self)
    }
    #[doc = "Bit 4 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_4(&mut self) -> DIFSEL_4_W<4> {
        DIFSEL_4_W::new(self)
    }
    #[doc = "Bit 5 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_5(&mut self) -> DIFSEL_5_W<5> {
        DIFSEL_5_W::new(self)
    }
    #[doc = "Bit 6 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_6(&mut self) -> DIFSEL_6_W<6> {
        DIFSEL_6_W::new(self)
    }
    #[doc = "Bit 7 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_7(&mut self) -> DIFSEL_7_W<7> {
        DIFSEL_7_W::new(self)
    }
    #[doc = "Bit 8 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_8(&mut self) -> DIFSEL_8_W<8> {
        DIFSEL_8_W::new(self)
    }
    #[doc = "Bit 9 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_9(&mut self) -> DIFSEL_9_W<9> {
        DIFSEL_9_W::new(self)
    }
    #[doc = "Bit 10 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_10(&mut self) -> DIFSEL_10_W<10> {
        DIFSEL_10_W::new(self)
    }
    #[doc = "Bit 11 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_11(&mut self) -> DIFSEL_11_W<11> {
        DIFSEL_11_W::new(self)
    }
    #[doc = "Bit 12 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_12(&mut self) -> DIFSEL_12_W<12> {
        DIFSEL_12_W::new(self)
    }
    #[doc = "Bit 13 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_13(&mut self) -> DIFSEL_13_W<13> {
        DIFSEL_13_W::new(self)
    }
    #[doc = "Bit 14 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_14(&mut self) -> DIFSEL_14_W<14> {
        DIFSEL_14_W::new(self)
    }
    #[doc = "Bit 15 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_15(&mut self) -> DIFSEL_15_W<15> {
        DIFSEL_15_W::new(self)
    }
    #[doc = "Bit 16 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_16(&mut self) -> DIFSEL_16_W<16> {
        DIFSEL_16_W::new(self)
    }
    #[doc = "Bit 17 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_17(&mut self) -> DIFSEL_17_W<17> {
        DIFSEL_17_W::new(self)
    }
    #[doc = "Bit 18 - Differential mode for channels 0"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_18(&mut self) -> DIFSEL_18_W<18> {
        DIFSEL_18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Differential Mode Selection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsel](index.html) module"]
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
