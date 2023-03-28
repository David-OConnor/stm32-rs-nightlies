#[doc = "Register `PUCRI` reader"]
pub struct R(crate::R<PUCRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRI` writer"]
pub struct W(crate::W<PUCRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRI_SPEC>;
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
impl From<crate::W<PUCRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - Port I pull-up bit y (y=0..11)"]
pub type PU0_R = crate::BitReader<PU0_A>;
#[doc = "Port I pull-up bit y (y=0..11)\n\nValue on reset: 0"]
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
#[doc = "Field `PU0` writer - Port I pull-up bit y (y=0..11)"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRI_SPEC, PU0_A, O>;
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
#[doc = "Field `PU1` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU1_R;
#[doc = "Field `PU2` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU2_R;
#[doc = "Field `PU3` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU3_R;
#[doc = "Field `PU4` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU4_R;
#[doc = "Field `PU5` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU5_R;
#[doc = "Field `PU6` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU6_R;
#[doc = "Field `PU7` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU7_R;
#[doc = "Field `PU8` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU8_R;
#[doc = "Field `PU9` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU9_R;
#[doc = "Field `PU10` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU10_R;
#[doc = "Field `PU11` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU11_R;
#[doc = "Field `PU1` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU1_W;
#[doc = "Field `PU2` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU2_W;
#[doc = "Field `PU3` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU3_W;
#[doc = "Field `PU4` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU4_W;
#[doc = "Field `PU5` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU5_W;
#[doc = "Field `PU6` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU6_W;
#[doc = "Field `PU7` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU7_W;
#[doc = "Field `PU8` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU8_W;
#[doc = "Field `PU9` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU9_W;
#[doc = "Field `PU10` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU10_W;
#[doc = "Field `PU11` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU11_W;
impl R {
    #[doc = "Bit 0 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Bit 1 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 2 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 3 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Bit 4 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    #[doc = "Bit 5 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<5> {
        PU5_W::new(self)
    }
    #[doc = "Bit 6 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 7 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    #[doc = "Bit 8 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<8> {
        PU8_W::new(self)
    }
    #[doc = "Bit 9 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<9> {
        PU9_W::new(self)
    }
    #[doc = "Bit 10 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<10> {
        PU10_W::new(self)
    }
    #[doc = "Bit 11 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<11> {
        PU11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port I pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucri](index.html) module"]
pub struct PUCRI_SPEC;
impl crate::RegisterSpec for PUCRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucri::R](R) reader structure"]
impl crate::Readable for PUCRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucri::W](W) writer structure"]
impl crate::Writable for PUCRI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRI to value 0"]
impl crate::Resettable for PUCRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
