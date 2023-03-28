#[doc = "Register `PDCRC` reader"]
pub struct R(crate::R<PDCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRC` writer"]
pub struct W(crate::W<PDCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRC_SPEC>;
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
impl From<crate::W<PDCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - PD0"]
pub type PD0_R = crate::BitReader<PD0_A>;
#[doc = "PD0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0_A {
    #[doc = "0: Disable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Enabled = 1,
}
impl From<PD0_A> for bool {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD0_A {
        match self.bits {
            false => PD0_A::Disabled,
            true => PD0_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD0_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD0_A::Enabled
    }
}
#[doc = "Field `PD0` writer - PD0"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, PD0_A, O>;
impl<'a, const O: u8> PD0_W<'a, O> {
    #[doc = "Disable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD0_A::Disabled)
    }
    #[doc = "Enable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD0_A::Enabled)
    }
}
#[doc = "Field `PD1` reader - PD1"]
pub use PD0_R as PD1_R;
#[doc = "Field `PD2` reader - PD2"]
pub use PD0_R as PD2_R;
#[doc = "Field `PD3` reader - PD3"]
pub use PD0_R as PD3_R;
#[doc = "Field `PD4` reader - PD4"]
pub use PD0_R as PD4_R;
#[doc = "Field `PD5` reader - PD5"]
pub use PD0_R as PD5_R;
#[doc = "Field `PD6` reader - PD6"]
pub use PD0_R as PD6_R;
#[doc = "Field `PD1` writer - PD1"]
pub use PD0_W as PD1_W;
#[doc = "Field `PD2` writer - PD2"]
pub use PD0_W as PD2_W;
#[doc = "Field `PD3` writer - PD3"]
pub use PD0_W as PD3_W;
#[doc = "Field `PD4` writer - PD4"]
pub use PD0_W as PD4_W;
#[doc = "Field `PD5` writer - PD5"]
pub use PD0_W as PD5_W;
#[doc = "Field `PD6` writer - PD6"]
pub use PD0_W as PD6_W;
#[doc = "Field `PD13` reader - PD13"]
pub type PD13_R = crate::BitReader<PD13_A>;
#[doc = "PD13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD13_A {
    #[doc = "0: Disable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Enabled = 1,
}
impl From<PD13_A> for bool {
    #[inline(always)]
    fn from(variant: PD13_A) -> Self {
        variant as u8 != 0
    }
}
impl PD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD13_A {
        match self.bits {
            false => PD13_A::Disabled,
            true => PD13_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD13_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD13_A::Enabled
    }
}
#[doc = "Field `PD13` writer - PD13"]
pub type PD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, PD13_A, O>;
impl<'a, const O: u8> PD13_W<'a, O> {
    #[doc = "Disable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD13_A::Disabled)
    }
    #[doc = "Enable the pull-down on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD13_A::Enabled)
    }
}
#[doc = "Field `PD14` reader - PD14"]
pub use PD13_R as PD14_R;
#[doc = "Field `PD15` reader - Port PC\\[y\\]
pull-down (y=13 to 15)"]
pub use PD13_R as PD15_R;
#[doc = "Field `PD14` writer - PD14"]
pub use PD13_W as PD14_W;
#[doc = "Field `PD15` writer - Port PC\\[y\\]
pull-down (y=13 to 15)"]
pub use PD13_W as PD15_W;
impl R {
    #[doc = "Bit 0 - PD0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PD1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PD2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PD3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PD4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PD5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PD6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - PD13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PD14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-down (y=13 to 15)"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PD0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - PD1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - PD2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - PD3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - PD4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - PD5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - PD6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 13 - PD13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - PD14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-down (y=13 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port C pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrc](index.html) module"]
pub struct PDCRC_SPEC;
impl crate::RegisterSpec for PDCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrc::R](R) reader structure"]
impl crate::Readable for PDCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrc::W](W) writer structure"]
impl crate::Writable for PDCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCRC to value 0"]
impl crate::Resettable for PDCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
