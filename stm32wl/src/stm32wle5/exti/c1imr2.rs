#[doc = "Register `C1IMR2` reader"]
pub struct R(crate::R<C1IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IMR2` writer"]
pub struct W(crate::W<C1IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IMR2_SPEC>;
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
impl From<crate::W<C1IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM34` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM34_R = crate::BitReader<IM34_A>;
#[doc = "CPUm Wakeup with interrupt Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM34_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM34_A> for bool {
    #[inline(always)]
    fn from(variant: IM34_A) -> Self {
        variant as u8 != 0
    }
}
impl IM34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM34_A {
        match self.bits {
            false => IM34_A::Masked,
            true => IM34_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM34_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM34_A::Unmasked
    }
}
#[doc = "Field `IM34` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1IMR2_SPEC, IM34_A, O>;
impl<'a, const O: u8> IM34_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM34_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM34_A::Unmasked)
    }
}
#[doc = "Field `IM38` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM38_R;
#[doc = "Field `IM42` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM42_R;
#[doc = "Field `IM43` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM43_R;
#[doc = "Field `IM44` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM44_R;
#[doc = "Field `IM45` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM45_R;
#[doc = "Field `IM46` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_R as IM46_R;
#[doc = "Field `IM38` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM38_W;
#[doc = "Field `IM42` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM42_W;
#[doc = "Field `IM43` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM43_W;
#[doc = "Field `IM44` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM44_W;
#[doc = "Field `IM45` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM45_W;
#[doc = "Field `IM46` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub use IM34_W as IM46_W;
impl R {
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<2> {
        IM34_W::new(self)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<6> {
        IM38_W::new(self)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im43(&mut self) -> IM43_W<11> {
        IM43_W::new(self)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im44(&mut self) -> IM44_W<12> {
        IM44_W::new(self)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im45(&mut self) -> IM45_W<13> {
        IM45_W::new(self)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im46(&mut self) -> IM46_W<14> {
        IM46_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr2](index.html) module"]
pub struct C1IMR2_SPEC;
impl crate::RegisterSpec for C1IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1imr2::R](R) reader structure"]
impl crate::Readable for C1IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1imr2::W](W) writer structure"]
impl crate::Writable for C1IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1IMR2 to value 0"]
impl crate::Resettable for C1IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
