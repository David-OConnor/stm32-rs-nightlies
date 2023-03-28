#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR2` writer"]
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM32` reader - Interrupt Mask on external/internal line 32"]
pub type IM32_R = crate::BitReader<IM32_A>;
#[doc = "Interrupt Mask on external/internal line 32\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM32_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM32_A> for bool {
    #[inline(always)]
    fn from(variant: IM32_A) -> Self {
        variant as u8 != 0
    }
}
impl IM32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM32_A {
        match self.bits {
            false => IM32_A::Masked,
            true => IM32_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM32_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM32_A::Unmasked
    }
}
#[doc = "Field `IM32` writer - Interrupt Mask on external/internal line 32"]
pub type IM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, IM32_A, O>;
impl<'a, const O: u8> IM32_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM32_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM32_A::Unmasked)
    }
}
#[doc = "Field `IM33` reader - Interrupt Mask on external/internal line 33"]
pub use IM32_R as IM33_R;
#[doc = "Field `IM34` reader - Interrupt Mask on external/internal line 34"]
pub use IM32_R as IM34_R;
#[doc = "Field `IM35` reader - Interrupt Mask on external/internal line 35"]
pub use IM32_R as IM35_R;
#[doc = "Field `IM36` reader - Interrupt Mask on external/internal line 36"]
pub use IM32_R as IM36_R;
#[doc = "Field `IM37` reader - Interrupt Mask on external/internal line 37"]
pub use IM32_R as IM37_R;
#[doc = "Field `IM40` reader - Interrupt Mask on external/internal line 40"]
pub use IM32_R as IM40_R;
#[doc = "Field `IM41` reader - Interrupt Mask on external/internal line 41"]
pub use IM32_R as IM41_R;
#[doc = "Field `IM42` reader - Interrupt Mask on external/internal line 42"]
pub use IM32_R as IM42_R;
#[doc = "Field `IM43` reader - Interrupt Mask on external/internal line 43"]
pub use IM32_R as IM43_R;
#[doc = "Field `IM33` writer - Interrupt Mask on external/internal line 33"]
pub use IM32_W as IM33_W;
#[doc = "Field `IM34` writer - Interrupt Mask on external/internal line 34"]
pub use IM32_W as IM34_W;
#[doc = "Field `IM35` writer - Interrupt Mask on external/internal line 35"]
pub use IM32_W as IM35_W;
#[doc = "Field `IM36` writer - Interrupt Mask on external/internal line 36"]
pub use IM32_W as IM36_W;
#[doc = "Field `IM37` writer - Interrupt Mask on external/internal line 37"]
pub use IM32_W as IM37_W;
#[doc = "Field `IM40` writer - Interrupt Mask on external/internal line 40"]
pub use IM32_W as IM40_W;
#[doc = "Field `IM41` writer - Interrupt Mask on external/internal line 41"]
pub use IM32_W as IM41_W;
#[doc = "Field `IM42` writer - Interrupt Mask on external/internal line 42"]
pub use IM32_W as IM42_W;
#[doc = "Field `IM43` writer - Interrupt Mask on external/internal line 43"]
pub use IM32_W as IM43_W;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<0> {
        IM32_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<1> {
        IM33_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<2> {
        IM34_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> IM35_W<3> {
        IM35_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> IM36_W<4> {
        IM36_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<5> {
        IM37_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<8> {
        IM40_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<9> {
        IM41_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    #[must_use]
    pub fn im43(&mut self) -> IM43_W<11> {
        IM43_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr2::W](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_ff87"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff87;
}
