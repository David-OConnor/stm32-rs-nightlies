#[doc = "Register `PUCRC` reader"]
pub struct R(crate::R<PUCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRC` writer"]
pub struct W(crate::W<PUCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRC_SPEC>;
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
impl From<crate::W<PUCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - PU0"]
pub type PU0_R = crate::BitReader<PU0_A>;
#[doc = "PU0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0_A {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
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
#[doc = "Field `PU0` writer - PU0"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, PU0_A, O>;
impl<'a, const O: u8> PU0_W<'a, O> {
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU0_A::Disabled)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU0_A::Enabled)
    }
}
#[doc = "Field `PU1` reader - PU1"]
pub use PU0_R as PU1_R;
#[doc = "Field `PU2` reader - PU2"]
pub use PU0_R as PU2_R;
#[doc = "Field `PU3` reader - PU3"]
pub use PU0_R as PU3_R;
#[doc = "Field `PU4` reader - PU4"]
pub use PU0_R as PU4_R;
#[doc = "Field `PU5` reader - PU5"]
pub use PU0_R as PU5_R;
#[doc = "Field `PU6` reader - PU6"]
pub use PU0_R as PU6_R;
#[doc = "Field `PU1` writer - PU1"]
pub use PU0_W as PU1_W;
#[doc = "Field `PU2` writer - PU2"]
pub use PU0_W as PU2_W;
#[doc = "Field `PU3` writer - PU3"]
pub use PU0_W as PU3_W;
#[doc = "Field `PU4` writer - PU4"]
pub use PU0_W as PU4_W;
#[doc = "Field `PU5` writer - PU5"]
pub use PU0_W as PU5_W;
#[doc = "Field `PU6` writer - PU6"]
pub use PU0_W as PU6_W;
#[doc = "Field `PU13` reader - PU13"]
pub type PU13_R = crate::BitReader<PU13_A>;
#[doc = "PU13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU13_A {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    Enabled = 1,
}
impl From<PU13_A> for bool {
    #[inline(always)]
    fn from(variant: PU13_A) -> Self {
        variant as u8 != 0
    }
}
impl PU13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU13_A {
        match self.bits {
            false => PU13_A::Disabled,
            true => PU13_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU13_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU13_A::Enabled
    }
}
#[doc = "Field `PU13` writer - PU13"]
pub type PU13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, PU13_A, O>;
impl<'a, const O: u8> PU13_W<'a, O> {
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU13_A::Disabled)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU13_A::Enabled)
    }
}
#[doc = "Field `PU14` reader - PU14"]
pub use PU13_R as PU14_R;
#[doc = "Field `PU15` reader - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub use PU13_R as PU15_R;
#[doc = "Field `PU14` writer - PU14"]
pub use PU13_W as PU14_W;
#[doc = "Field `PU15` writer - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub use PU13_W as PU15_W;
impl R {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<5> {
        PU5_W::new(self)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> PU13_W<13> {
        PU13_W::new(self)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> PU14_W<14> {
        PU14_W::new(self)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
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
#[doc = "Power Port C pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrc](index.html) module"]
pub struct PUCRC_SPEC;
impl crate::RegisterSpec for PUCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrc::R](R) reader structure"]
impl crate::Readable for PUCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrc::W](W) writer structure"]
impl crate::Writable for PUCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRC to value 0"]
impl crate::Resettable for PUCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
