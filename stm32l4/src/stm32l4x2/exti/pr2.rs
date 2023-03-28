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
#[doc = "Field `PIF35` reader - Pending interrupt flag on line 35"]
pub type PIF35_R = crate::BitReader<PIF35R_A>;
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF35R_A> for bool {
    #[inline(always)]
    fn from(variant: PIF35R_A) -> Self {
        variant as u8 != 0
    }
}
impl PIF35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF35R_A {
        match self.bits {
            false => PIF35R_A::NotPending,
            true => PIF35R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF35R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF35R_A::Pending
    }
}
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF35W_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF35W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF35` writer - Pending interrupt flag on line 35"]
pub type PIF35_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR2_SPEC, PIF35W_AW, O>;
impl<'a, const O: u8> PIF35_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF35W_AW::Clear)
    }
}
#[doc = "Field `PIF36` reader - Pending interrupt flag on line 36"]
pub use PIF35_R as PIF36_R;
#[doc = "Field `PIF37` reader - Pending interrupt flag on line 37"]
pub use PIF35_R as PIF37_R;
#[doc = "Field `PIF38` reader - Pending interrupt flag on line 38"]
pub use PIF35_R as PIF38_R;
#[doc = "Field `PIF36` writer - Pending interrupt flag on line 36"]
pub use PIF35_W as PIF36_W;
#[doc = "Field `PIF37` writer - Pending interrupt flag on line 37"]
pub use PIF35_W as PIF37_W;
#[doc = "Field `PIF38` writer - Pending interrupt flag on line 38"]
pub use PIF35_W as PIF38_W;
impl R {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&self) -> PIF35_R {
        PIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&self) -> PIF36_R {
        PIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&self) -> PIF37_R {
        PIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&self) -> PIF38_R {
        PIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    #[must_use]
    pub fn pif35(&mut self) -> PIF35_W<3> {
        PIF35_W::new(self)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    #[must_use]
    pub fn pif36(&mut self) -> PIF36_W<4> {
        PIF36_W::new(self)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    #[must_use]
    pub fn pif37(&mut self) -> PIF37_W<5> {
        PIF37_W::new(self)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    #[must_use]
    pub fn pif38(&mut self) -> PIF38_W<6> {
        PIF38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x78;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
