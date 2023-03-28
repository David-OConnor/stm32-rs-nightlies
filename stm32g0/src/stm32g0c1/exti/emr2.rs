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
#[doc = "Field `EM32` reader - CPU wakeup with event mask on event input"]
pub type EM32_R = crate::BitReader<EM32_A>;
#[doc = "CPU wakeup with event mask on event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM32_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM32_A> for bool {
    #[inline(always)]
    fn from(variant: EM32_A) -> Self {
        variant as u8 != 0
    }
}
impl EM32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM32_A {
        match self.bits {
            false => EM32_A::Masked,
            true => EM32_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM32_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM32_A::Unmasked
    }
}
#[doc = "Field `EM32` writer - CPU wakeup with event mask on event input"]
pub type EM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, EM32_A, O>;
impl<'a, const O: u8> EM32_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM32_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM32_A::Unmasked)
    }
}
#[doc = "Field `EM33` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM33_R;
#[doc = "Field `EM34` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM34_R;
#[doc = "Field `EM35` reader - CPU wakeup with event mask on event input"]
pub use EM32_R as EM35_R;
#[doc = "Field `EM33` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM33_W;
#[doc = "Field `EM34` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM34_W;
#[doc = "Field `EM35` writer - CPU wakeup with event mask on event input"]
pub use EM32_W as EM35_W;
impl R {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> EM32_W<0> {
        EM32_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> EM33_W<1> {
        EM33_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> EM34_W<2> {
        EM34_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> EM35_W<3> {
        EM35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
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
