#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR32` reader - Event mask on external/internal line 32"]
pub type MR32_R = crate::BitReader<MR32_A>;
#[doc = "Event mask on external/internal line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR32_A> for bool {
    #[inline(always)]
    fn from(variant: MR32_A) -> Self {
        variant as u8 != 0
    }
}
impl MR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR32_A {
        match self.bits {
            false => MR32_A::Masked,
            true => MR32_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32_A::Unmasked
    }
}
#[doc = "Field `MR32` writer - Event mask on external/internal line 32"]
pub type MR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, MR32_A, O>;
impl<'a, const O: u8> MR32_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR32_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR32_A::Unmasked)
    }
}
#[doc = "Field `MR33` reader - Event mask on external/internal line 33"]
pub use MR32_R as MR33_R;
#[doc = "Field `MR34` reader - Event mask on external/internal line 34"]
pub use MR32_R as MR34_R;
#[doc = "Field `MR35` reader - Event mask on external/internal line 35"]
pub use MR32_R as MR35_R;
#[doc = "Field `MR36` reader - Event mask on external/internal line 36"]
pub use MR32_R as MR36_R;
#[doc = "Field `MR37` reader - Event mask on external/internal line 37"]
pub use MR32_R as MR37_R;
#[doc = "Field `MR38` reader - Event mask on external/internal line 38"]
pub use MR32_R as MR38_R;
#[doc = "Field `MR39` reader - Event mask on external/internal line 39"]
pub use MR32_R as MR39_R;
#[doc = "Field `MR40` reader - Event mask on external/internal line 40"]
pub use MR32_R as MR40_R;
#[doc = "Field `MR33` writer - Event mask on external/internal line 33"]
pub use MR32_W as MR33_W;
#[doc = "Field `MR34` writer - Event mask on external/internal line 34"]
pub use MR32_W as MR34_W;
#[doc = "Field `MR35` writer - Event mask on external/internal line 35"]
pub use MR32_W as MR35_W;
#[doc = "Field `MR36` writer - Event mask on external/internal line 36"]
pub use MR32_W as MR36_W;
#[doc = "Field `MR37` writer - Event mask on external/internal line 37"]
pub use MR32_W as MR37_W;
#[doc = "Field `MR38` writer - Event mask on external/internal line 38"]
pub use MR32_W as MR38_W;
#[doc = "Field `MR39` writer - Event mask on external/internal line 39"]
pub use MR32_W as MR39_W;
#[doc = "Field `MR40` writer - Event mask on external/internal line 40"]
pub use MR32_W as MR40_W;
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<0> {
        MR32_W::new(self)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<1> {
        MR33_W::new(self)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<2> {
        MR34_W::new(self)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<3> {
        MR35_W::new(self)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    #[must_use]
    pub fn mr36(&mut self) -> MR36_W<4> {
        MR36_W::new(self)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    #[must_use]
    pub fn mr37(&mut self) -> MR37_W<5> {
        MR37_W::new(self)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    #[must_use]
    pub fn mr38(&mut self) -> MR38_W<6> {
        MR38_W::new(self)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    #[must_use]
    pub fn mr39(&mut self) -> MR39_W<7> {
        MR39_W::new(self)
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    #[must_use]
    pub fn mr40(&mut self) -> MR40_W<8> {
        MR40_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
