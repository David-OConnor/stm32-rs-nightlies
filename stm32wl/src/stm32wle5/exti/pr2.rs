#[doc = "Register `PR2` reader"]
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR2` writer"]
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF34` reader - Configurable event inputs 33 Pending bit."]
pub type PIF34_R = crate::BitReader<PIF34R_A>;
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF34R_A> for bool {
    #[inline(always)]
    fn from(variant: PIF34R_A) -> Self {
        variant as u8 != 0
    }
}
impl PIF34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF34R_A {
        match self.bits {
            false => PIF34R_A::NotPending,
            true => PIF34R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF34R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF34R_A::Pending
    }
}
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF34W_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF34W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF34` writer - Configurable event inputs 33 Pending bit."]
pub type PIF34_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR2_SPEC, PIF34W_AW, O>;
impl<'a, const O: u8> PIF34_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF34W_AW::Clear)
    }
}
#[doc = "Field `PIF45` reader - Configurable event inputs 45 Pending bit."]
pub use PIF34_R as PIF45_R;
#[doc = "Field `PIF45` writer - Configurable event inputs 45 Pending bit."]
pub use PIF34_W as PIF45_W;
impl R {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&self) -> PIF34_R {
        PIF34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&self) -> PIF45_R {
        PIF45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif34(&mut self) -> PIF34_W<2> {
        PIF34_W::new(self)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif45(&mut self) -> PIF45_W<13> {
        PIF45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr2::R](R) reader structure"]
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr2::W](W) writer structure"]
impl crate::Writable for PR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x2004;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
