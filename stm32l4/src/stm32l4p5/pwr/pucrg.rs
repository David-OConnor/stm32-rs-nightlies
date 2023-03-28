#[doc = "Register `PUCRG` reader"]
pub struct R(crate::R<PUCRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRG` writer"]
pub struct W(crate::W<PUCRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRG_SPEC>;
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
impl From<crate::W<PUCRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - Port G pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader<PU0_A>;
#[doc = "Port G pull-up bit y (y=0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0_A {
    #[doc = "0: Pull-Up on Pxx is disabled"]
    Disabled = 0,
    #[doc = "1: Pull-Up on Pxx is enabled"]
    Enabled = 1,
}
impl From<PU0_A> for bool {
    #[inline(always)]
    fn from(variant: PU0_A) -> Self {
        variant as u8 != 0
    }
}
impl PU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU0_A {
        match self.bits {
            false => PU0_A::Disabled,
            true => PU0_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0_A::Enabled
    }
}
#[doc = "Field `PU0` writer - Port G pull-up bit y (y=0..15)"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRG_SPEC, PU0_A, O>;
impl<'a, const O: u8> PU0_W<'a, O> {
    #[doc = "Pull-Up on Pxx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU0_A::Disabled)
    }
    #[doc = "Pull-Up on Pxx is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU0_A::Enabled)
    }
}
#[doc = "Field `PU1` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU1_R;
#[doc = "Field `PU2` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU2_R;
#[doc = "Field `PU3` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU3_R;
#[doc = "Field `PU4` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU4_R;
#[doc = "Field `PU5` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU5_R;
#[doc = "Field `PU6` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU6_R;
#[doc = "Field `PU7` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU7_R;
#[doc = "Field `PU8` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU8_R;
#[doc = "Field `PU9` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU9_R;
#[doc = "Field `PU10` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU10_R;
#[doc = "Field `PU11` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU11_R;
#[doc = "Field `PU12` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU12_R;
#[doc = "Field `PU13` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU13_R;
#[doc = "Field `PU14` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU14_R;
#[doc = "Field `PU15` reader - Port G pull-up bit y (y=0..15)"]
pub use PU0_R as PU15_R;
#[doc = "Field `PU1` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU1_W;
#[doc = "Field `PU2` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU2_W;
#[doc = "Field `PU3` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU3_W;
#[doc = "Field `PU4` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU4_W;
#[doc = "Field `PU5` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU5_W;
#[doc = "Field `PU6` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU6_W;
#[doc = "Field `PU7` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU7_W;
#[doc = "Field `PU8` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU8_W;
#[doc = "Field `PU9` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU9_W;
#[doc = "Field `PU10` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU10_W;
#[doc = "Field `PU11` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU11_W;
#[doc = "Field `PU12` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU12_W;
#[doc = "Field `PU13` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU13_W;
#[doc = "Field `PU14` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU14_W;
#[doc = "Field `PU15` writer - Port G pull-up bit y (y=0..15)"]
pub use PU0_W as PU15_W;
impl R {
    #[doc = "Bit 0 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Bit 1 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 2 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 3 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Bit 4 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    #[doc = "Bit 5 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<5> {
        PU5_W::new(self)
    }
    #[doc = "Bit 6 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 7 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    #[doc = "Bit 8 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<8> {
        PU8_W::new(self)
    }
    #[doc = "Bit 9 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<9> {
        PU9_W::new(self)
    }
    #[doc = "Bit 10 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<10> {
        PU10_W::new(self)
    }
    #[doc = "Bit 11 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<11> {
        PU11_W::new(self)
    }
    #[doc = "Bit 12 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu12(&mut self) -> PU12_W<12> {
        PU12_W::new(self)
    }
    #[doc = "Bit 13 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> PU13_W<13> {
        PU13_W::new(self)
    }
    #[doc = "Bit 14 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> PU14_W<14> {
        PU14_W::new(self)
    }
    #[doc = "Bit 15 - Port G pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> PU15_W<15> {
        PU15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port G pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrg](index.html) module"]
pub struct PUCRG_SPEC;
impl crate::RegisterSpec for PUCRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrg::R](R) reader structure"]
impl crate::Readable for PUCRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrg::W](W) writer structure"]
impl crate::Writable for PUCRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRG to value 0"]
impl crate::Resettable for PUCRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
