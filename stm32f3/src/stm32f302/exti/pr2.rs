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
#[doc = "Field `PR32` reader - Pending bit on line 32"]
pub type PR32_R = crate::BitReader<PR32R_A>;
#[doc = "Pending bit on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR32R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR32R_A> for bool {
    #[inline(always)]
    fn from(variant: PR32R_A) -> Self {
        variant as u8 != 0
    }
}
impl PR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR32R_A {
        match self.bits {
            false => PR32R_A::NotPending,
            true => PR32R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR32R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR32R_A::Pending
    }
}
#[doc = "Pending bit on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR32W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR32W_AW> for bool {
    #[inline(always)]
    fn from(variant: PR32W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR32` writer - Pending bit on line 32"]
pub type PR32_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR2_SPEC, PR32W_AW, O>;
impl<'a, const O: u8> PR32_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR32W_AW::Clear)
    }
}
#[doc = "Field `PR33` reader - Pending bit on line 33"]
pub use PR32_R as PR33_R;
#[doc = "Field `PR33` writer - Pending bit on line 33"]
pub use PR32_W as PR33_W;
impl R {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&self) -> PR32_R {
        PR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&self) -> PR33_R {
        PR33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    #[must_use]
    pub fn pr32(&mut self) -> PR32_W<0> {
        PR32_W::new(self)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    #[must_use]
    pub fn pr33(&mut self) -> PR33_W<1> {
        PR33_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
